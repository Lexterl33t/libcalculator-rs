use super::token::Token;
use std::str::Chars;
use std::iter::Peekable;

pub struct Eat;

impl Eat{

    pub fn eat_number(first_char: char, input: &mut Chars<'_>) -> Token {
        let mut number_str = String::new();
        number_str.push(first_char);
        while let Some(ch) = input.clone().next() {
            if ch.is_digit(10) || ch == '-' {
                number_str.push(ch);
                input.next();  // Consomme effectivement le caractère
            } else {
                break;
            }
        }

        // Tente de parser la chaîne en nombre
        if let Ok(number) = number_str.parse::<i64>() {
            Token::Number(number)
        } else {
            Token::Eof
        }
    }
}