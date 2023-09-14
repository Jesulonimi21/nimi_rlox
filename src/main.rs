mod scanner;
mod token;
// mod Expr;
mod expression_visitor;
mod Expr;
mod parser;
mod interprete;
mod stmt;
mod environment;

use std::env;
use std::process::exit;
use std::fs;
use std::io;
use std::io::Write;
use scanner::Scanner;
use token::Token;
use parser::Parser;
use interprete::Interpreter;
// use crate::Expr::{Binary, Grouping, Literal, Unary};\\
use crate::Expr::{Expr::{binary, literal, unary, grouping}, Binary, Unary, Literal, Grouping};
use crate::TokenType::{EOF, PLUS};
// use crate::ExpressionVisitor::AST_PRINTER_VISITOR;

#[derive(Copy, Clone, PartialEq)]
#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // One or two character tokens.
    BANG,
    BangEqual,
    EQUAL,
    EqualEqual,
    GREATER,
    GreaterEqual,
    LESS,
    LessEqual,

    // Literals.
    IDENTIFIER, STRING, NUMBER,

    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
}
static mut HAD_ERROR: bool = false;
fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 2 {
        println!("Jlox script");
        exit(64);
    }else if args.len() == 2 {
        run_file(&args[1]);
    }else{
        run_prompt();
    }
    // let exprBin = binary(Binary{
    //     left:Box::new(literal(Literal{
    //         value: Box::new(String::from("1"))
    //     })) ,
    //     token_operator: Token{token_type: PLUS, lexeme: String::from("+"), literal: Box::new(String::from("+")),line:1},
    //     right: Box::new(literal(Literal{
    //     value: Box::new(String::from("2"))
    // }))
    // });
    //
    // let exprUn = unary(Unary{
    //     token_operator: Token{token_type: PLUS, lexeme: String::from("+"), literal: Box::new(String::from("+")),line:1},
    //     expr: Box::new(exprBin)
    // });
    //
    // let exprGroup = grouping(Grouping{
    //     expr: Box::new(exprUn),
    // });
    // let expr_vis = AST_PRINTER_VISITOR{
    //     expr: Box::new(exprGroup)
    // };
    //
    // expr_vis.print()
}

fn run_file(file_path: &String) {
    let contents = fs::read_to_string(file_path);
    match contents{
        Ok(content)=> {
            let mut interpreter: Interpreter =Interpreter::new();
                            run(String::from(content.trim()), &mut interpreter);
                             unsafe { if HAD_ERROR { exit(64); } }
                             }
        Err(error)=> println!("Could not find file: {}", error),
    }

}
fn run_prompt(){
    let mut interpreter: Interpreter =Interpreter::new();
     loop{
        print!(">");
         io::stdout().flush().expect("Failed to flush stdout");
        let mut line = String::new();
        let line_of_code = io::stdin().read_line(&mut line);
        match line_of_code{
            Err(_x) => break,
            Ok(_x) => { unsafe { HAD_ERROR = false; }


                run(String::from(line.trim()), &mut interpreter) },
        }
    }
}
fn run(source: String, interpreter: &mut Interpreter){
    println!("{}", &source);
    let mut scanner = Scanner::new(source);
    let tokens: &Vec<Token> = scanner.scan_tokens();
    println!("TOKEN LENGTH FROM MAIN: {}", tokens.len());
    let mut parser = Parser {
        tokens,
        current:0
    };
    let statements = parser.parse();
    
    interpreter.interprete(statements);
    // let expr_vis = AST_PRINTER_VISITOR{
    //     expr: Box::new(expr)
    // };

    // expr_vis.print()

    // for i in tokens{
    //     println!("{:?}", i);
    //
    //
    // }

}


fn error_parser(token:&Token, message:String){
    if(token.token_type == EOF){
        report(token.line.clone(), String::from("at end ",),message);
    }else{
        report(token.line.clone(), String::from(format!("at {} ",token.lexeme.clone())),message);
    }
}

fn error(line:usize, message:String){
    report(line, String::new(), message);
}

fn report(line: usize, location:String,message: String ){
    eprintln!("[line {} ] Error: {} : {}", line, location, message);
    unsafe{
        HAD_ERROR = true;
    }
}


//https://plugins.jetbrains.com/plugin/8182-rust/docs/rust-quick-start.html