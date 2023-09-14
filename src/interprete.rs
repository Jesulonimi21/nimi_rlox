use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::environment::Environment;
use crate::Expr::{Assign, Binary, Expr, Grouping, Literal, Unary, Variable};
use crate::Expr::Expr::assign;
use crate::TokenType::*;
use crate::expression_visitor;
use crate::stmt;
use crate::stmt::{BlockExpression, Expression, PrintExpression, Stmt, VarExpression};

pub struct Interpreter{
    pub environment:  Rc<RefCell<Environment>>
}

impl expression_visitor::Visitor<Box<dyn Any>> for Interpreter{
    fn visit_binary_expr( &mut self, expr: &Binary) -> Box<dyn Any> {
        let left_any = self.evaluate(&expr.left);
        let right_any = self.evaluate(&expr.right);
        if let Some(l_val) = left_any.downcast_ref::<f64>(){
            if let Some(r_val) = right_any.downcast_ref::<f64>(){
                return match expr.token_operator.token_type {
                    MINUS => Box::new(l_val - r_val),
                    PLUS => Box::new(l_val + r_val),
                    SLASH => Box::new(l_val / r_val),
                    STAR => Box::new(l_val * r_val),
                    EqualEqual => Box::new(l_val == r_val),
                    GreaterEqual => Box::new(l_val >= r_val),
                    GREATER => Box::new(l_val > r_val),
                    LESS => Box::new(l_val < r_val),
                    LessEqual => Box::new(l_val <= r_val),
                    _ => Box::new(0),
                };
            }
        }else if let Some(l_val) = left_any.downcast_ref::<bool>(){
            if let Some(r_val) = right_any.downcast_ref::<bool>(){
                return match expr.token_operator.token_type {
                    EqualEqual => Box::new(l_val == r_val),
                    GreaterEqual => Box::new(l_val >= r_val),
                    GREATER => Box::new(l_val > r_val),
                    LESS => Box::new(l_val < r_val),
                    LessEqual => Box::new(l_val <= r_val),
                    _ => Box::new(false)
                }
            }
        }

        return Box::new(Option::<String>::None);
    }

    fn visit_unary_expr( &mut self, expr: &Unary) -> Box<dyn Any> {
        let ret_val_any = self.evaluate(&expr.expr);
        if let Some(int_value) = ret_val_any.downcast_ref::<f64>(){
            if expr.token_operator.token_type == MINUS{
                 return Box::new(- int_value)
            }
        }else if let Some(bool_value) = ret_val_any.downcast_ref::<bool>(){
            return Box::new(!bool_value)
        }
        panic!("Contact developer, unexpected error happened");

    }

    fn visit_grouping_expr( &mut self, expr: &Grouping) -> Box<dyn Any> {
        return self.evaluate(&expr.expr)
    }

    fn visit_literal_expr(&mut self, expr: &  Literal) -> Box<dyn Any> {
        if let Some(int_value) = expr.value.downcast_ref::<f64>(){
            return Box::new(int_value.clone());
        }else if let Some(str_value) = expr.value.downcast_ref::<String>(){
            return Box::new(str_value.clone());
        }else if let Some(bool_value) = expr.value.downcast_ref::<bool>(){
            return Box::new(bool_value.clone());
        }
        panic!("Unexpected error, contact developers");
    }

    fn visit_variable_expr( &mut self, expr: &Variable) -> Box<dyn Any> {
        // let anyValue =  self.environment.borrow().get(&expr.name);
        if let Some(optionValue) = self.environment.borrow().get(&expr.name).downcast_ref::<Option<Box<dyn Any>>>(){
            if let Some(value) = optionValue{
                if let Some(int_value) = value.downcast_ref::<f64>(){
                    return Box::new(int_value.clone());
                }else if let Some(str_value) = value.downcast_ref::<String>(){
                    return Box::new(str_value.clone());
                }else if let Some(bool_value) = value.downcast_ref::<bool>(){
                    return Box::new(bool_value.clone());
                }
        }

        }

        panic!("Unexpected error, contact developers; tried to access uninitialized variable");
    }

    fn visit_assigned_expr(&mut self, expr: &Assign) -> Box<dyn Any> {
        let value = self.evaluate(&expr.value);

        let name = &expr.name;
        if let Some(int_value) = self.evaluate(&expr.value).downcast_ref::<f64>() {
            println!("{:?}",int_value);
        }else {  println!("nothing");}
        self.environment.borrow_mut().assign(name.lexeme.clone(), Box::new(Some(value)));
        return self.evaluate(&expr.value);
    }
}

impl stmt::Visitor<()> for Interpreter{
    fn visit_expression(&mut self, expression: &Expression) -> () {
        let result = self.evaluate(&expression.expr);

    }

    fn visit_print_expression(&mut self, printExpression: &PrintExpression) -> () {
        let result = self.evaluate(&printExpression.expr);
        if let Some(int_value) = result.downcast_ref::<f64>() {
            println!("{:?}",int_value);
        } else if let Some(str_value) = result.downcast_ref::<String>() {
            println!("{}",str_value);
        }else if let Some(bool_val) = result.downcast_ref::<bool>() {
            println!("{:?}", bool_val);
        }
    }


    fn visit_variable_statement(&mut self, variable_expression: &VarExpression) -> () {
        let token_name = variable_expression.token.lexeme.clone();

        let mut expr_val = Option::None;
        if let Some(value) = &variable_expression.initializer{
            expr_val = Some(self.evaluate(value));
            if let Some(int_value) = self.evaluate(value).downcast_ref::<f64>() {
                println!("{:?}",int_value);
            }
        }
        self.environment.borrow_mut().define(token_name, Box::new(expr_val))
    }

    fn visit_block_statement(&mut self, block_expression: &BlockExpression) -> () {
        // self.executeBlock(&block_expression.staments, Environment::new_enclosing(&mut self.environment))
        // Save the previous environment

        let mut previous =Rc::clone(&self.environment);

        self.environment = Rc::new(RefCell::new(Environment::new_enclosing(previous.clone())));

        for statement in &block_expression.staments {
            statement.accept(self);
        }

        // Restore the previous environment
        self.environment = previous;
    }

}


impl Interpreter{
    pub fn new()-> Interpreter {

        return Interpreter {
            environment: Rc::new(RefCell::new(Environment::new()))
        };
    }

    pub fn interprete(&mut self, statements: Vec<Stmt>){
       for statement in statements{
           statement.accept(self);
       }
    }

    fn evaluate (&mut self, expr: &Expr)-> Box<dyn Any> {
        return expr.accept( self)
    }

    // fn executeBlock(&mut self, statements: &Vec<Stmt>, environment: Environment){
    //     let previous = self.environment;
    //     self.environment = environment;
    //     for statement in statements{
    //         statement.accept(self);
    //     }
    //     self.environment = previous;
    // }
}