use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Expr, ExprPath, Pat, Token};

#[derive(Clone)]
pub enum DoStatement {
    Return(Return),
    Let(Let),
    Bind(Bind),
    Expr(Expr),
}

impl DoStatement {
    pub fn binder(&self) -> Option<&Pat> {
        match self {
            DoStatement::Bind(Bind { pat, .. }) => Some(pat),
            DoStatement::Let(Let { pat, .. }) => Some(pat),
            _ => None,
        }
    }

    pub fn body(&self) -> &Expr {
        match self {
            DoStatement::Bind(Bind { body, .. }) => body,
            DoStatement::Let(Let { expr, .. }) => expr,
            DoStatement::Return(Return { expr, .. }) => expr,
            DoStatement::Expr(expr) => expr,
        }
    }
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Bind {
    pub pat: Pat,
    pub bind_token: Token![<-],
    pub body: Expr,
}

impl ToTokens for Bind {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.pat.to_tokens(tokens);
        self.bind_token.to_tokens(tokens);
        self.body.to_tokens(tokens);
    }
}

#[derive(Clone)]
pub struct Namespace(pub ExprPath);

impl ToTokens for Namespace {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

#[derive(Clone)]
pub struct QDo {
    pub namespace: Namespace,
    pub statements: Vec<DoStatement>,
    pub trailing_semi: bool,
}
