use crate::lexer::Lexer;

mod user_input;
mod lexer;
mod token;

fn main() {
    loop {
        println!("Input: ");
        let input = user_input::input_line();
        let tokens = Lexer::new(&input).parse().unwrap();
        if tokens.len() == 1 {break}
        println!("Tokens: {:?}", tokens);
    }
}
