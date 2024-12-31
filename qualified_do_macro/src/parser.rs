use punctuated::Punctuated;
use syn::{parse::*, *};

use super::types;
use super::types::*;

impl Parse for DoStatement {
    fn parse(input: ParseStream) -> Result<Self> {
        use DoStatement::*;
        if input.peek(Token![return]) {
            Ok(Return(input.parse()?))
        } else if input.peek(Token![let]) {
            Ok(Let(input.parse()?))
        } else if input.peek(keywords::guard) {
            Ok(Guard(input.parse()?))
        } else if input.peek(Token![~]) {
            Ok(Bind(input.parse()?))
            // FIXME: This might be too expensive;
            // consider using a more efficient way of parsing.
        } else if input.fork().parse::<types::Bind>().is_ok() {
            Ok(Bind(input.parse().unwrap()))
        } else {
            Ok(Expr(input.parse()?))
        }
    }
}

impl Parse for Return {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Return {
            return_token: input.parse()?,
            expr: input.parse::<Expr>()?,
        })
    }
}

impl Parse for Guard {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Guard {
            guard_token: input.parse::<keywords::guard>()?,
            cond: input.parse::<Expr>()?,
        })
    }
}

impl Parse for Let {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Let {
            let_token: input.parse::<Token![let]>()?,
            pat: Pat::parse_single(input)?,
            eq_token: input.parse::<Token![=]>()?,
            expr: input.parse::<Expr>()?,
        })
    }
}

impl Parse for Bind {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Bind {
            irrefutable: input.parse()?,
            pat: Pat::parse_single(input)?,
            bind_token: input.parse::<Token![<-]>()?,
            body: input.parse::<Expr>()?,
        })
    }
}

impl Parse for Namespace {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse().map(Namespace)
    }
}

impl Parse for QDo {
    fn parse(input: ParseStream) -> Result<Self> {
        let namespace = input.parse()?;
        let content;
        braced!(content in input);
        let statements = Punctuated::<DoStatement, Token![;]>::parse_terminated(&content)?;
        if statements.is_empty() {
            return Err(Error::new(
                content.span(),
                "expected at least one statement",
            ));
        }
        let trailing_semi = statements.trailing_punct();
        let statements = statements.into_iter().collect();

        Ok(QDo {
            namespace,
            statements,
            trailing_semi,
        })
    }
}
