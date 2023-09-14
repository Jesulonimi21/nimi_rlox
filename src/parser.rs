use std::borrow::Borrow;
use std::panic;
use std::thread::current;
use crate::Expr::{Assign, Binary, Expr, Grouping, Literal, Unary, Variable};
use crate::Expr::Expr::{binary, grouping, literal, unary, variable};
use crate::token::Token;
use crate::TokenType;
use crate::TokenType::*;
use crate::*;
use crate::stmt::*;
use crate::stmt::Stmt::{blockExpression, varExpression};

pub struct Parser<'a>{
    pub tokens: & 'a Vec<Token>,
    pub current:usize
}

impl Parser<'_>{
    fn new  <'a> (tokens: & 'a Vec<Token>) ->  Parser{
        return Parser{
            tokens,
            current: 0
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt>{
        let mut allStatements:Vec<Stmt> = vec![];
        while !self.isAtEnd() && self.peek().token_type != EOF{
            let statement = self.declaration();
            allStatements.push(statement);
        }
        return allStatements;
    }


    fn declaration(&mut self)-> Stmt{
        if self.match_operator(vec![VAR]){
            return self.varDeclaration();
        }


        return self.statement();
    }


    fn varDeclaration(&mut self) -> Stmt{
        let name = self.consume(IDENTIFIER, String::from("expect variable name"));
        let scoped_name = Parser::cloneToken(name);
        let mut initializer = Option::None;
        if(self.match_operator(vec![EQUAL])){
            initializer = Some(self.expression());
        }
        self.consume(SEMICOLON, String::from("Expect ;"));
        return varExpression(VarExpression{
            token: scoped_name,
            initializer
        })
    }

    fn statement(&mut self)-> Stmt{
        if self.match_operator(vec![PRINT]){
           return  self.print_statement()
        }
        if self.match_operator(vec![LeftBrace]){
            println!("enterred left brace");
            return blockExpression(BlockExpression{ staments:self.block()});
        }
        return  self.expression_statement();
    }

    fn block(&mut self)-> Vec<Stmt>{
        let mut statements:Vec<Stmt> = vec![];
        while !self.check(RightBrace) && !self.isAtEnd(){
            statements.push(self.declaration());
        };
        self.consume(RightBrace, String::from("Expect } after block"));
        return  statements;
    }

    fn print_statement(&mut self) ->Stmt{
        let expr = self.expression();
        self.consume(SEMICOLON, String::from("Expect ;"));
        return Stmt::printExpression(PrintExpression{
            expr
        });
    }

    fn expression_statement(&mut self) ->Stmt {
        let expr = self.expression();
        self.consume(SEMICOLON, String::from("Expect ;"));
        return Stmt::expression(Expression{expr});
    }
    fn expression(&mut self) -> Expr{
        return  self.assignment();
    }

    fn assignment(&mut self) -> Expr{
        let expr = self.equality();
        println!("Is an assignment!");
        if self.match_operator(vec![EQUAL]){
            println!("Is an assignment!!");
            let equal = Parser::cloneToken(self.previous());
            let value = self.assignment();
            match &expr{
                variable(varExpr)=> {
                    let name = Parser::cloneToken(&varExpr.name);
                    return Expr::assign(Assign{
                        name,
                        value: Box::new(value)
                    })
                },
                _ => {error_parser(&equal, String::from("invalid assignment target"))}
            }
        }

        return  expr;
    }

    fn equality(&mut self) -> Expr{
        let mut expr = self.comparison();
        if self.match_operator(vec![EqualEqual]){
            expr = binary(Binary{
                left: Box::new(expr),
                token_operator: Parser::cloneToken(self.previous()),
                right: Box::new(self.equality())
            })
        }
        return expr;
    }

    fn comparison(&mut self) -> Expr{
        let mut expr = self.term();
        if self.match_operator(vec![GREATER, GreaterEqual, LESS, LessEqual]){
            expr = binary(Binary{
                left: Box::new(expr),
                token_operator: Parser::cloneToken(self.previous()),
                right: Box::new(self.comparison())
            })
        }

        return expr;
    }

    fn term(&mut self) -> Expr{
        let mut  expr = self.factor();
        if self.match_operator(vec![PLUS, MINUS]){
            expr = binary(Binary{
                left: Box::new(expr),
                token_operator: Parser::cloneToken(self.previous()),
                right: Box::new(self.term())
            })
        }
        return expr;
    }

    fn factor(&mut self) -> Expr{
        let mut expr = self.unary();

        if self.match_operator(vec![STAR, SLASH]){
            expr = binary(Binary{
                left: Box::new(expr),
                token_operator: Parser::cloneToken(self.previous()),
                right: Box::new(self.factor())
            })
        }
        return expr;
    }

    fn unary(&mut self) -> Expr{
        if self.match_operator(vec![BANG, MINUS]){
            let expr = unary(Unary{
                token_operator: Parser::cloneToken(self.previous()),
                expr: Box::new(self.primary())
            });
            return expr
        }else{
            return self.primary();
        }


    }

    fn primary(&mut self) -> Expr{
        if self.match_operator(vec![TRUE]){
            return literal(Literal{
                value: Box::new(true)
            })
        }
        if self.match_operator(vec![FALSE]){
            return literal(Literal{
                value: Box::new(false)
            })
        }
        if self.match_operator(vec![NIL]){
            let value: Box<Option<f32>> = Box::new(Option::None);
            return literal(Literal{
                    value
            })
        }

        if self.match_operator(vec![NUMBER, STRING]) {
            return  literal(Literal{
                value: if let Some(int_value) = self.previous().literal.downcast_ref::<f64>() {
                    Box::new(int_value.clone())
                } else if let Some(str_value) = self.previous().literal.downcast_ref::<String>() {
                    Box::new(str_value.to_string())
                } else {
                    println!("casted as: wrong value");
                    Box::new(0f64)
                }
            });
        }

        if self.match_operator(vec![LeftParen]){
            let expr = self.expression();
            self.consume(RightParen, String::from("expected )"));
            return grouping(Grouping{
                expr: Box::new(expr)
            })
        }
        if self.match_operator(vec![IDENTIFIER]){
            return variable(Variable{
                name: Parser::cloneToken(self.previous())
            })
        }
        error_parser(self.peek(), String::from("Expect expression"));
        panic!("Expect expression");
    }

    fn consume(&mut self, token_type: TokenType, expectStr: String)-> &Token{
        if self.check(token_type){
            return  self.advance();
        }
       panic::set_hook(Box::new(Parser::handle_panic));
        self.synchronize();
       error_parser(self.peek(), expectStr.clone());

       panic!("{}",expectStr)
    }

    fn check(&mut self, token_type: TokenType) -> bool{
        if self.isAtEnd() { return false; };
        return self.peek().token_type == token_type;
    }

    fn handle_panic(info: &panic::PanicInfo) {
    }

    pub fn cloneToken(token: &Token) -> Token{
        return Token{
            token_type: token.token_type.clone(),
            lexeme: token.lexeme.clone(),
            literal: if let Some(int_value) = token.literal.downcast_ref::<i32>() {
                Box::new(int_value.clone())
            } else if let Some(str_value) = token.literal.downcast_ref::<String>() {
                Box::new(str_value.clone())
            } else {
                // Handle the case when x doesn't match any expected type
                // You can add error handling or default behavior here.
                // For example, return an empty Box.
                Box::new(0i32)
            },
            line: token.line.clone(),
        };
    }


    fn previous(&self) -> &Token{
        return &self.tokens[self.current.clone() - 1];
    }

    fn peek(&self) -> &Token{
        return &self.tokens[self.current.clone()]
    }

    fn match_operator(&mut self, type_tokens: Vec<TokenType>) -> bool{
     if self.isAtEnd() { return false }

      for i in  type_tokens {
          if(self.peek().token_type == i){
              self.advance();
              return  true;
          }
      }
      return false;
     }

    fn advance(&mut self)-> &Token{
        self.current = self.current.clone() + 1;
        return self.previous();
    }

    fn isAtEnd(&mut self)-> bool{
        return self.current == self.tokens.len();
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.isAtEnd(){
            if self.previous().token_type == SEMICOLON{
                return;
            }

            match self.peek().token_type{
                CLASS=> { return; }
                FUN=>{ return; }
                FOR=>{ return; }
                IF=>{ return; }
                WHILE=>{ return; }
                PRINT=>{ return; }
                RETURN=>{ return; }
                _=> {self.advance();}
            };
        }
    }
}