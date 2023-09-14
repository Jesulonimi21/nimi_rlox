
use crate::Token;
use std::any::Any;
use std::borrow::BorrowMut;
use crate::Expr::Expr::{assign, binary, grouping, literal, unary, variable};
use crate::expression_visitor::Visitor;
pub enum Expr{
    binary(Binary),
    grouping(Grouping),
    unary(Unary),
    literal(Literal),
    variable(Variable),
    assign(Assign)
}


impl Expr{
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T{
        match self{
            binary(x) => { visitor.visit_binary_expr(x) }
            grouping(x) => { visitor.visit_grouping_expr(x) }
            unary(x) => { visitor.visit_unary_expr(x) }
            literal(x) => { visitor.visit_literal_expr(x) }
            variable(x) => {visitor.visit_variable_expr(x)},
            assign(x) => {visitor.visit_assigned_expr(x)}
        }
    }
}



pub struct Binary{
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub token_operator: Token
}

pub struct Grouping{
    pub expr: Box<Expr>
}

pub struct  Unary{
    pub token_operator: Token,
    pub expr: Box<Expr>
}

pub struct Literal{
    pub value:Box<dyn Any>
}

pub struct Variable{
    pub name: Token
}

pub struct Assign{
    pub name: Token,
    pub value: Box<Expr>
}