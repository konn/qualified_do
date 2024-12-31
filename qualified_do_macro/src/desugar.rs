use super::types;
use super::types::*;
use proc_macro2::*;
use quote::{quote, ToTokens};
use std::collections::{HashSet, VecDeque};
use syn::visit_mut::VisitMut;
use syn::{parse_quote, visit::*, ExprPath, PatIdent};
use syn::{Error, Pat};

fn mk_bind_cont(namespace: Namespace, counter: &mut u64, p: Pat, body: TokenStream) -> TokenStream {
    if let Pat::Ident(ident) = p {
        return quote! { move |#ident| #body };
    }
    let err = format!("Pattern match failed:\n  expected: {}", p.to_token_stream());
    let var = fresh_var(counter);
    quote! { move |#var|
        match #var {
            #p => #body,
            _ => #namespace::fail(#err),
        }
    }
}

fn fresh_var(counter: &mut u64) -> syn::Ident {
    *counter += 1;
    Ident::new(&format!("__qdo_arg_{}", counter), Span::call_site())
}

impl QDo {
    pub fn desugar(self) -> Result<TokenStream, syn::Error> {
        if let Some(e) = self.clone().desugar_applicative() {
            Ok(e)
        } else {
            self.desugar_monad()
        }
    }

    pub fn desugar_monad(self) -> Result<TokenStream, syn::Error> {
        let QDo {
            namespace,
            mut statements,
            trailing_semi,
        } = self;

        let pure = quote! { #namespace::pure };
        let guard = quote! { #namespace::guard };
        let and_then = quote! { #namespace::and_then };
        let counter = &mut 0;

        let last = if trailing_semi {
            quote! { #pure(()) }
        } else {
            let last = statements.pop().unwrap();
            match last {
                DoStatement::Expr(expr) => quote! { #expr },
                DoStatement::Return(Return { expr, .. }) => quote! { #pure(#expr) },
                t => {
                    return Err(Error::new_spanned(
                        t,
                        "Expected an expression or return statement at the last",
                    ))
                }
            }
        };
        statements
            .into_iter()
            .try_rfold(last, |acc, stmt| match stmt {
                DoStatement::Expr(expr) => Ok(quote! { #and_then(#expr, |_| #acc) }),
                DoStatement::Return(Return { expr, .. }) => {
                    Ok(quote! { #and_then(#pure(#expr), |_| #acc) })
                }
                DoStatement::Let(Let { pat, expr, .. }) => Ok(quote! { {let #pat = #expr; #acc} }),
                DoStatement::Bind(Bind { pat, body, .. }) => {
                    let closure = mk_bind_cont(namespace.clone(), counter, pat, acc);
                    Ok(quote! {
                        #and_then(#body, #closure)
                    })
                }
                DoStatement::Guard(Guard { cond, .. }) => Ok(quote! {
                    #and_then(#guard(#cond), |()| #acc)
                }),
            })
    }

    pub fn desugar_applicative(self) -> Option<TokenStream> {
        use DoStatement::*;
        let mut statements = self.statements.clone();
        let counter = &mut 0;
        enum Scrutinee {
            Let(syn::Expr),
            Bind(syn::Expr),
            Ret(syn::Expr),
            Guard(syn::Expr),
        }
        let last = if self.trailing_semi {
            Some(Return(parse_quote! {return ()}))
        } else {
            statements.pop()
        };

        if let Some(Return(ret)) = last {
            // TODO: resolve duplicate binding collectly.
            let namespace = self.namespace;
            let mut bound = HashSet::<Ident>::with_capacity(statements.len());
            let mut scrutinees = VecDeque::new();
            let fmap = quote! { #namespace::fmap };
            let zip = |x: TokenStream, y: Scrutinee| {
                let a = fresh_var(counter);
                let b = fresh_var(counter);
                use Scrutinee::*;
                match y {
                    Bind(y) => quote! { #namespace::zip_with(|#a, #b| (#a, #b), #x, #y) },
                    Let(y) => quote! { #namespace::fmap(|#a| { let #b = #y; (#a, #b) }, #x) },
                    Ret(y) => quote! { #namespace::fmap(|#a| { let #b = #y; (#a, #b)}, #x) },
                    Guard(g) => {
                        quote! { #namespace::zip_with(|#a, #b| (#a, #b), #x, #namespace::guard(#g))}
                    }
                }
            };
            for stmt in statements {
                let mut call_visitor = ExprVarWalker::default();
                call_visitor.visit_expr(stmt.body());
                if call_visitor.free.intersection(&bound).next().is_some() {
                    return None;
                }
                // TODO: Supprot `#[infalliable]` attribute
                let binder = match stmt.binder().cloned() {
                    Some(syn::Pat::Ident(pident)) => Some(Some(pident)),
                    Some(_) => None,
                    None => Some(None),
                }?;
                let scrutinee = match stmt {
                    Let(types::Let { expr, .. }) => Scrutinee::Let(expr),
                    Bind(types::Bind { body, .. }) => Scrutinee::Bind(body),
                    Expr(expr) => Scrutinee::Let(expr),
                    Return(types::Return { expr, .. }) => Scrutinee::Ret(expr),
                    Guard(types::Guard { cond: expr, .. }) => Scrutinee::Guard(expr),
                };
                let pat: syn::Pat = if let Some(pident) = binder {
                    bound.insert(pident.ident.clone());
                    syn::Pat::Ident(pident)
                } else {
                    parse_quote! { _ }
                };
                scrutinees.push_back((scrutinee, pat));
            }

            let mut sealer = PatVarSealer::default();
            for (_, p) in scrutinees.iter_mut().rev() {
                sealer.visit_pat_mut(p);
            }
            if let Some((scrut0, pat0)) = scrutinees.pop_front() {
                let (scrutinees, pats): (Vec<_>, Vec<_>) = scrutinees.into_iter().unzip();
                let scrut0 = match scrut0 {
                    Scrutinee::Bind(e) => e.into_token_stream(),
                    Scrutinee::Let(e) => quote! { #namespace::pure(#e) },
                    Scrutinee::Ret(e) => quote! { #namespace::pure(#e) },
                    Scrutinee::Guard(e) => quote! { #namespace::guard(#e) },
                };
                let body = scrutinees.into_iter().fold(scrut0, zip);
                let pat = pats.into_iter().fold(pat0.into_token_stream(), |x, y| {
                    quote! { (#x, #y) }
                });
                let types::Return { expr: result, .. } = ret;
                Some(quote! { #fmap(|#pat| #result, #body) })
            } else {
                let pure = quote! { #namespace::pure };
                let types::Return { expr: result, .. } = ret;
                Some(quote! { #pure(#result) })
            }
        } else {
            None
        }
    }
}

#[derive(Default, Clone)]
struct PatVarWalker {
    pat_idents: HashSet<Ident>,
}

impl Visit<'_> for PatVarWalker {
    fn visit_pat_ident(&mut self, node: &PatIdent) {
        self.pat_idents.insert(node.ident.clone());
    }
}

#[derive(Default, Clone)]
struct ExprVarWalker {
    free: HashSet<Ident>,
    bound: HashSet<Ident>,
}
impl Visit<'_> for ExprVarWalker {
    fn visit_expr_path(&mut self, node: &ExprPath) {
        self.free.extend(
            node.path
                .get_ident()
                .into_iter()
                .filter(|p| !self.bound.contains(p))
                .cloned(),
        );
    }

    fn visit_expr_closure(&mut self, cls: &syn::ExprClosure) {
        let mut deeper = self.clone();
        let mut walker = PatVarWalker::default();
        for i in cls.inputs.iter() {
            walker.visit_pat(i);
        }
        deeper.bound.extend(walker.pat_idents);
        deeper.visit_expr(&cls.body);
        self.free.extend(deeper.free);
    }
}

#[derive(Debug, Default)]
pub struct PatVarSealer {
    bound: HashSet<Ident>,
}

impl VisitMut for PatVarSealer {
    fn visit_pat_mut(&mut self, node: &mut Pat) {
        match node {
            Pat::Ident(ident) => {
                if self.bound.contains(&ident.ident) {
                    *node = parse_quote! { _ };
                } else {
                    self.bound.insert(ident.ident.clone());
                }
            }
            p => syn::visit_mut::visit_pat_mut(self, p),
        }
    }
}
