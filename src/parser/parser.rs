use crate::{ast::node::Node, lexer::token::Token};

#[derive(Debug)]
pub struct Parser<'TokensLife> {
    tokens: &'TokensLife Vec<Token>
}

impl<'TokensLife> Parser<'TokensLife>{

    pub fn new(tokens: &'TokensLife Vec<Token>) -> Self {
        Self {
            tokens: tokens
        }
    }

    pub fn compile(&self) -> Node {
        
        while let Some(token) = self.tokens.iter().next() {
            println!("{:?}", token);
        }

        Node::Number(1)
    }
}