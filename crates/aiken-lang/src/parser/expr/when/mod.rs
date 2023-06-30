use chumsky::prelude::*;

mod clause;
mod guard;

pub use clause::parser as clause;
pub use guard::parser as guard;

use crate::{
    expr::UntypedExpr,
    parser::{error::ParseError, token::Token},
};

pub fn parser(
    r: Recursive<'_, Token, UntypedExpr, ParseError>,
) -> impl Parser<Token, UntypedExpr, Error = ParseError> + '_ {
    just(Token::When)
        // TODO: If subject is empty we should return ParseErrorType::ExpectedExpr,
        .ignore_then(r.clone().map(Box::new))
        .then_ignore(just(Token::Is))
        .then_ignore(just(Token::LeftBrace))
        // TODO: If clauses are empty we should return ParseErrorType::NoCaseClause
        .then(clause(r).repeated())
        .then_ignore(just(Token::RightBrace))
        .map_with_span(|(subject, clauses), span| UntypedExpr::When {
            location: span,
            subject,
            clauses,
        })
}
