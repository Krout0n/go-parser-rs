use std::collections::VecDeque;

use crate::literals::integer::IntLit;

pub mod tokenizer;

pub(crate) type Tokens<'a> = VecDeque<Token<'a>>;

#[derive(Debug, PartialEq)]
pub enum Delimiter {
    // '(' ')'
    Paren,
    // '[' ']'
    Bra,
    // '{' '}'
    Cur,
}

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    IntLit(IntLit<'a>),
    Keyword(&'static str),
    LDel(Delimiter),
    RDel(Delimiter),
    Symbol(Symbol),
}

impl<'a> Token<'a> {
    fn len(&self) -> usize {
        match self {
            Self::IntLit(i) => i.len(),
            Token::Keyword(s) => s.len(),
            Token::LDel(_) | Token::RDel(_) => 1,
            Token::Symbol(s) => s.len(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Symbol {
    Plus,
    Minus,
    Aster,
    Slash,
}
