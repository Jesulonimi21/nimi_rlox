// use crate::Expr::{Binary, Expr, Grouping, Literal, Unary};
use crate::Expr::{Expr, Binary, Unary, Literal, Grouping, Variable, Assign};

pub trait Visitor<T>{
     fn visit_binary_expr(&mut self, expr: &Binary) -> T;
     fn visit_unary_expr(&mut self, expr: &Unary) -> T;
     fn visit_grouping_expr(&mut self, expr: &Grouping) -> T;
     fn visit_literal_expr(&mut self, expr: &Literal) -> T;
     fn visit_variable_expr(&mut self, expr: &Variable) -> T;
     fn visit_assigned_expr(&mut self, expr: &Assign) -> T;
}


//
// pub struct AST_PRINTER_VISITOR{
//     pub expr:Box<Expr>
// }
//
// impl Visitor<String> for AST_PRINTER_VISITOR{
//      fn visit_binary_expr(&self, expr: &Binary) -> String {
//           self.print_literal(&expr.token_operator.lexeme,vec![&expr.left, &expr.right] )
//      }
//
//      fn visit_unary_expr(&self, expr: &Unary) -> String {
//           return self.print_literal(&expr.token_operator.lexeme,vec![&expr.expr] )
//      }
//
//      fn visit_grouping_expr(&self, expr: &Grouping) -> String {
//           return self.print_literal(&String::from("group"),vec![&expr.expr] )
//      }
//
//      fn visit_literal_expr(&self, expr: &Literal) -> String {
//           println!("{:?}", &expr.value);
//           return match expr.value.downcast_ref::<f32>(){
//                Some(x) => x.to_string(),
//                None => String::from("")
//           };
//      }
//
//      fn visit_variable_expr(&self, expr: &Variable) -> String {
//           return String::new();
//      }
//
//      fn visit_assigned_expr(&mut self, expr: &Assign) -> String {
//           return String::new();
//      }
// }
//
// impl AST_PRINTER_VISITOR{
//
//      fn new(expr: Box<Expr>)-> Self{
//           return Self{
//                expr
//           }
//      }
//
//      pub fn print(&mut self){
//            self.expr.accept( self);
//      }
//
//      fn print_literal(&mut self, name:&String, exprs: Vec<&Box<Expr>>) -> String{
//           let mut new_str = String::from("");
//           new_str.push_str("(");
//           new_str.push_str(format!("{}  ", name).as_str());
//           for expr in exprs{
//                new_str.push_str(" ");
//                new_str = new_str + expr.accept( self).as_str();
//           }
//           new_str.push_str(")");
//           println!("{}",new_str);
//           new_str
//      }
// }
//
