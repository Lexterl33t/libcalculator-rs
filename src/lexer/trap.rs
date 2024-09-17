use super::token::Token;
#[derive(Debug)]
pub enum LexerTrap<'TokenLife> {
    LexingError,
    LexingSuccess(&'TokenLife Vec<Token>)
}