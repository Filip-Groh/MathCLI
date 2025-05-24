
#[derive(Debug)]
pub enum Token {
    Number(i32),
    Operator(Operator),
    Unknown(char),
    EOF()
}

#[derive(Debug)]
pub enum Operator {
    Addition(),
    Subtraction(),
    Multiplication(),
    Division()
}