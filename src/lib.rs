use ast::node::Node;
use lexer::trap::LexerTrap;

mod lexer;
mod parser;
mod ast;

pub struct Calculator;

impl Calculator {

    pub fn eval<'InputLife>(input: &'InputLife String) {
        let mut lex = lexer::lexer::Lexer::new(input);
        let ret_lexer = lex.tokenize().unwrap();
        let tokens = match ret_lexer {
            LexerTrap::LexingSuccess(tokens) => tokens,
            LexerTrap::LexingError => panic!("Lexing error")
        };

        let parser_engine = parser::parser::Parser::new(tokens);
        println!("{:?}", parser_engine);

        
        
    }
}