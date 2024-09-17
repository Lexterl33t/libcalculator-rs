

pub enum Node {
    Number(i64),
    BinOp(Box<Node>, char, Box<Node>)
}

impl Node {

    pub fn evaluate(&self) -> i64 {
        match self {
            Node::Number(num) => *num,
            Node::BinOp(lhs,op,  rhs) => {
                match op {
                    '+' => lhs.evaluate() + rhs.evaluate(),
                    '-' => lhs.evaluate() - rhs.evaluate(),
                    '*' => lhs.evaluate() * rhs.evaluate(),
                    '/' => lhs.evaluate() / rhs.evaluate(),
                    _ => 0
                }
            }
        }
    }
}