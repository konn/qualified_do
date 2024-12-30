use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Expr, ExprPath, Pat, Token};

pub enum DoStatement {
    Return(Return),
    Let(Let),
    Bind(Bind),
    Expr(Expr),
}

impl ToTokens for DoStatement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            DoStatement::Return(r) => r.to_tokens(tokens),
            DoStatement::Let(l) => l.to_tokens(tokens),
            DoStatement::Bind(b) => b.to_tokens(tokens),
            DoStatement::Expr(e) => e.to_tokens(tokens),
        }
    }
}

pub struct Return {
    pub return_token: Token![return],
    pub expr: Expr,
}

impl ToTokens for Return {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.return_token.to_tokens(tokens);
        self.expr.to_tokens(tokens);
    }
}

pub struct Let {
    pub let_token: Token![let],
    pub pat: Pat,
    pub eq_token: Token![=],
    pub expr: Expr,
}

impl ToTokens for Let {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.let_token.to_tokens(tokens);
        self.pat.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        self.expr.to_tokens(tokens);
    }
}

pub struct Bind {
    pub bindee: Pat,
    pub bind_token: Token![<-],
    pub body: Expr,
}

impl ToTokens for Bind {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.bindee.to_tokens(tokens);
        self.bind_token.to_tokens(tokens);
        self.body.to_tokens(tokens);
    }
}

pub struct QDo {
    pub namespace: ExprPath,
    pub statements: Vec<DoStatement>,
    pub trailing_semi: bool,
}
