use crate::Expr::Expr;
use crate::stmt::Stmt::{blockExpression, expression, printExpression, varExpression};
use crate::token::Token;


pub enum Stmt{
     expression(Expression),
    printExpression(PrintExpression),
    varExpression(VarExpression),
    blockExpression(BlockExpression)
}

pub trait Visitor<T>{
    fn visit_expression(&mut self, expression: &Expression) -> T;
    fn visit_print_expression(&mut self, print_expression: &PrintExpression) -> T;
    fn visit_variable_statement(&mut self, variable_expression: &VarExpression) -> T;
    fn visit_block_statement(&mut self, block_expression:&BlockExpression) -> T;
}

impl Stmt{
   pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T{
        match self {
            expression(expr)=> visitor.visit_expression(expr),
            printExpression(expr) => visitor.visit_print_expression(expr),
            varExpression(expr) => visitor.visit_variable_statement(expr),
            blockExpression(expr) => visitor.visit_block_statement(expr)
        }
    }
}





pub struct Expression{
    pub expr: Expr
}

pub struct PrintExpression{
    pub expr: Expr
}

pub struct VarExpression{
    pub initializer: Option<Expr>,
    pub token: Token
}

pub struct BlockExpression{
    pub staments: Vec<Stmt>
}

