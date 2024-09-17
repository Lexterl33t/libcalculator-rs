
use super::token::Token;
use super::eat::Eat;
use super::trap::LexerTrap;

#[derive(Debug)]
pub struct Lexer<'InputLife> {
    input: &'InputLife String,
    tokens: Vec<Token>
}

impl<'InputLife> Lexer<'InputLife> {
    
    pub fn new(input: &'InputLife String) -> Self {
        Self{
            input: input,
            tokens: Vec::new()
        }
    }

    pub fn tokenize(&mut self) -> Result<LexerTrap, LexerTrap>{

        let mut input_iter = self.input.chars();

        while let Some(ch) = input_iter.next() {
            println!("{ch}");
            match ch {
                '1'..'9' | '-' => {
                    let number_token = Eat::eat_number(ch, &mut input_iter);
                    self.tokens.push(number_token);
                }
                '+' => {
                    self.tokens.push(Token::Plus);
                    input_iter.next(); 
                }
                '*' => {
                    self.tokens.push(Token::Mul);
                    input_iter.next(); 
                }
                '/' => {
                    self.tokens.push(Token::Div);
                    input_iter.next(); 
                }
                '(' => {
                    self.tokens.push(Token::LBracket);
                    input_iter.next();
                }
                ')' => {
                    self.tokens.push(Token::RBracket);
                    input_iter.next();
                }
                ' ' | '\n' | '\t' => (),
                _ => {
                    return Err(LexerTrap::LexingError)
                }
            }
        }
        
        Ok(LexerTrap::LexingSuccess(&self.tokens))
    }
}