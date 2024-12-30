use super::types::*;
use proc_macro2::*;
use quote::quote;
use syn::Error;

impl QDo {
    // TODO: Support for MonadFail
    // TODO: Support for ApplicativeDo
    pub fn desugar(self) -> Result<TokenStream, syn::Error> {
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
                    Ok(quote! { #and_then(#pure(#expr), |_| acc) })
                }
                DoStatement::Let(Let { pat, expr, .. }) => Ok(quote! { {let #pat = #expr; #acc} }),
                DoStatement::Bind(Bind { bindee, body, .. }) => Ok(quote! {
                    #and_then(#body, |#bindee| #acc)
                }),
            })
    }
}
