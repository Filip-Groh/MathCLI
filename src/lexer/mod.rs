use std::error::Error;
use std::iter::Peekable;
use std::str::Chars;
use crate::token::{Operator, Token};

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    tokens: Vec<Token>
}

impl Lexer<'_> {
    pub fn new(text: &String) -> Lexer {
        Lexer { 
            chars: text.chars().peekable(),
            tokens: Vec::new()
        }
    }
    
    pub fn parse(mut self) -> Result<Vec<Token>, Box<dyn Error>> {
        loop {
            match self.chars.peek() {
                Some(char) => {
                    match char {
                        ' ' | '\r' | '\n' => { self.chars.next(); }
                        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => self.parse_number(),
                        '+' | '-' | '*' | '/' => self.parse_operator(),
                        _ => self.tokens.push(Token::Unknown(self.chars.next().unwrap()))
                    }
                }
                None => break
            }
        }
        
        self.tokens.push(Token::EOF());

        Ok(self.tokens)
    }
    
    fn parse_number(&mut self) {
        let mut current_number = Vec::new();
    
        loop {
            match self.chars.peek() {
                Some(char) => {
                    match char {
                        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => current_number.push(self.chars.next().unwrap()),
                        _ => break
                    }
                }
                None => break
            }
        }
    
        if current_number.len() == 0 { return }
    
        self.tokens.push(Token::Number(String::from_iter(current_number).parse().unwrap()));
    }
    
    fn parse_operator(&mut self) {
        match self.chars.peek() {
            Some(char) => {
                match char {
                    '+' => {
                        self.chars.next(); 
                        self.tokens.push(Token::Operator(Operator::Addition()))
                    }
                    '-' => {
                        self.chars.next();
                        self.tokens.push(Token::Operator(Operator::Subtraction()))
                    }
                    '*' => {
                        self.chars.next();
                        self.tokens.push(Token::Operator(Operator::Multiplication()))
                    }
                    '/' => {
                        self.chars.next();
                        self.tokens.push(Token::Operator(Operator::Division()))
                    }
                    _ => ()
                }
            }
            None => ()
        }
    }
}