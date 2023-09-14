use std::any::Any;
use std::collections::HashMap;
use std::process::exit;
use std::thread::current;
use crate::token::Token;
use crate::TokenType;
use crate::TokenType::*;
use crate::*;


// static mut start:u64= 0;
// static mut current:u64=0;
static mut line:u64 = 1;

pub struct  Scanner{
    pub source: String,
    pub tokens: Vec<Token>,
    start:usize,
    line:usize,
    current:usize,

}


impl Scanner{
     const start:u64= 0;
    pub fn new(source:String) -> Self{
       return Self{source, tokens: vec![], start:0,line:0, current:0 }
    }
    pub fn scan_tokens(&mut self) -> &Vec<Token>{
        while ! self.is_at_end(){
            self.start = self.current;
            self.scan_token();
        }

        let last_token = Token{token_type: EOF, lexeme: String::from(""), literal: Box::new(NIL), line: self.line};
        self.tokens.push(last_token);
        &self.tokens
    }

    fn scan_token(&mut self){
        let c = self.advance();
        match c{
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(COMMA),
            '.' => self.add_token(DOT),
            '-' => self.add_token(MINUS),
            '+' => self.add_token(PLUS),
            ';' => self.add_token(SEMICOLON),
            '*' => self.add_token(STAR),
            '!' => if self.match_next('='){self.add_token(BangEqual)}else{self.add_token(BANG)}
            '=' => if self.match_next('='){self.add_token(EqualEqual)}else{self.add_token(EQUAL)}
            '<' => if self.match_next('='){self.add_token(LessEqual)}else{self.add_token(LESS)}
            '>' => if self.match_next('='){self.add_token(GreaterEqual)}else{self.add_token(GREATER)}
            '"' => { self.string(); }
            '/' => if(self.match_next('/')){while(self.peek() != '\n') {self.advance();}}else{self.add_token(SLASH);}
            ' ' =>{}
            '\r' => {}
            '\r' => {}
            '\t' => {}
            '\n' => {self.line = self.line + 1}
            _ => if self.isDigit(c.clone()){ self.number();
                    } else if self.is_alpha(c){
                        self.identifier();
                    } else { error(self.line, String::from(format!("Unexpected character {:?}", c))) },
        }
    }

    fn identifier(&mut self){
        while(self.is_alpha_numeric(self.peek())){
            self.advance();
        }

        let mut my_tye_map: HashMap<&str, TokenType>= HashMap::new();
        my_tye_map.insert("and", AND);
        my_tye_map.insert("class", CLASS);
        my_tye_map.insert("else", ELSE);
        my_tye_map.insert("false", FALSE);
        my_tye_map.insert("for", FOR);
        my_tye_map.insert("if", IF);
        my_tye_map.insert("nil", NIL);
        my_tye_map.insert("or", OR);
        my_tye_map.insert("print", PRINT);
        my_tye_map.insert("var", VAR);
        my_tye_map.insert("return", RETURN);
        my_tye_map.insert("while", WHILE);
        my_tye_map.insert("super", SUPER);
        my_tye_map.insert("this", THIS);
        my_tye_map.insert("true", TRUE);

        let text = &self.source[self.start..self.current];
        let type_ = my_tye_map.get(text);
        if let Some(_type) = type_{
            self.add_token(_type.clone());
        }else{
            self.add_token(IDENTIFIER)
        }

    }
    fn is_alpha_numeric(&self, c: char)-> bool{
        return (self.is_alpha(c.clone()) || self.isDigit(c)) && c != '\0' && c!= '\n' && c != ' '&& c!= '(' && c!=')'&& c!=';'&& c!='=';
    }

    fn is_alpha(&self, c:char) -> bool{
        return c >= 'a' || c <= 'z' || c >= 'A' || c <= 'Z' || c == '_' ;
    }

    fn string(&mut self){
        while self.peek() != '"' && !self.is_at_end(){
            if self.peek() == '\n'{
                self.line = self.line + 1;
            }
            self.advance();
        }

        if self.is_at_end(){
            error(self.line, String::from(format!("unexpected character: {}", self.source.chars().nth(self.current).unwrap())))
        }

        self.advance();
        let string_iteral = String::from(&self.source[(self.start + 1)..(self.current -1)]);
        self.add_token_internal(STRING, Box::new(string_iteral));


    }



    fn number(&mut self){
        while self.isDigit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.isDigit(self.peek_next()){
            self.advance();
            while self.isDigit(self.peek()){
                self.advance();
            }
        }

        let f64_str = &self.source[self.start..self.current];
        self.add_token_internal(NUMBER, Box::new(f64_str.parse::<f64>().unwrap()))


    }
    fn peek_next(&self) ->char{
        return  self.source.chars().nth(self.current + 1).unwrap();
    }

    fn isDigit(& self, c: char)-> bool {
        return c >= '0' && c <= '9';
    }
    fn peek(&self)-> char{
        if self.is_at_end(){ return  '\0'}
        return  self.source.chars().nth(self.current).unwrap()
    }

    fn match_next(&mut self, c: char) -> bool{
        if self.is_at_end() {return false;}

        if self.source.chars().nth(self.current).unwrap() != c{
            return false;
        }
        self.current = self.current + 1;
        return  true


    }

    fn add_token(&mut self, token_type: TokenType){
        self.add_token_internal(token_type, Box::new(NIL))
    }

    fn add_token_internal(&mut self, token_type: TokenType, _literal: Box<dyn Any>){
        let text = &self.source[self.start..self.current];
        let new_token = Token{token_type, lexeme: String::from(text), literal: _literal, line: self.line};
        self.tokens.push(new_token);
    }

    fn advance(&mut self) -> char{
        let new_source = self.source.clone();
        let present_character = new_source.chars().nth(self.current);
        self.current = self.current + 1;
        present_character.unwrap()

    }


    fn is_at_end(&self) -> bool{
        self.current >= self.source.len()
    }
}
