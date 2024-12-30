use super::types;
use super::types::*;
use proc_macro2::*;
use quote::{quote, ToTokens};
use std::collections::{HashSet, VecDeque};
use syn::{parse_quote, visit::*, ExprPath};
use syn::{Error, PatIdent};

impl QDo {
    // TODO: Support for MonadFail
    // TODO: Support for ApplicativeDo
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
        let and_then = quote! { #namespace::and_then };
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
                DoStatement::Bind(Bind {
                    pat: bindee, body, ..
                }) => Ok(quote! {
                    #and_then(#body, |#bindee| #acc)
                }),
            })
    }

    pub fn desugar_applicative(self) -> Option<TokenStream> {
        use DoStatement::*;
        let mut statements = self.statements.clone();
        enum Scrutinee {
            Let(syn::Expr),
            Bind(syn::Expr),
            Ret(syn::Expr),
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
                use Scrutinee::*;
                match y {
                    Bind(y) => quote! { #namespace::zip_with(|a, b| (a, b), #x, #y) },
                    Let(y) => quote! { #namespace::fmap(|a| { let b = #y; (a, b) }, #x) },
                    Ret(y) => quote! { #namespace::fmap(|a| { let b = #y; (a, b)}, #x) },
                }
            };
            for stmt in statements {
                let mut call_visitor = ExprVarWalker::default();
                call_visitor.visit_expr(stmt.body());
                if call_visitor.0.intersection(&bound).next().is_some() {
                    return None;
                }
                let binder = stmt.binder().cloned();
                let scrutinee = match stmt {
                    Let(types::Let { expr, .. }) => Scrutinee::Let(expr),
                    Bind(types::Bind { body, .. }) => Scrutinee::Bind(body),
                    Expr(expr) => Scrutinee::Let(expr),
                    Return(types::Return { expr, .. }) => Scrutinee::Ret(expr),
                };
                let mut pat_visitor = PatVarWalker::default();
                let pat = if let Some(pat) = binder {
                    pat_visitor.visit_pat(&pat);
                    bound.extend(pat_visitor.0);
                    pat.clone()
                } else {
                    parse_quote! { _ }
                };
                scrutinees.push_back((scrutinee, pat));
            }
            if let Some((scrut0, pat0)) = scrutinees.pop_front() {
                let (scrutinees, pats): (Vec<_>, Vec<_>) = scrutinees.into_iter().unzip();
                let scrut0 = match scrut0 {
                    Scrutinee::Bind(e) => e.into_token_stream(),
                    Scrutinee::Let(e) => quote! { #namespace::pure(#e) },
                    Scrutinee::Ret(e) => quote! { #namespace::pure(#e) },
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

#[derive(Default)]
struct PatVarWalker(HashSet<Ident>);
impl Visit<'_> for PatVarWalker {
    fn visit_pat_ident(&mut self, node: &PatIdent) {
        self.0.insert(node.ident.clone());
    }
}

#[derive(Default)]
struct ExprVarWalker(HashSet<Ident>);
impl Visit<'_> for ExprVarWalker {
    fn visit_expr_path(&mut self, node: &ExprPath) {
        self.0.extend(node.path.get_ident().into_iter().cloned());
    }
}
