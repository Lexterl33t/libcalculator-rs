#[derive(Debug)]
pub enum Token{
    Number(i64),
    Float(f64),

    Plus,
    Sub,
    Div,
    Mul,

    LBracket,
    RBracket,

    Eof
}