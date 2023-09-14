use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::token::Token;

pub struct Environment{
    values: HashMap<String, Box<dyn Any>>,
    enclosing:  Option<Rc<RefCell<Environment>>>
}

impl Environment<>{
    pub fn new() -> Self{
        return Self{
            values:HashMap::new(),
            enclosing: Option::None
        }
    }


    pub fn new_enclosing(enclosing_environment: Rc<RefCell<Environment>>) -> Environment{
        return  Environment {
            values:HashMap::new(),
            enclosing: Some(enclosing_environment)
        }
    }

    pub fn define(&mut self, token_name: String, value: Box<dyn Any>){
        self.values.insert(token_name.clone(), value);
        println!("{} stored", token_name);
    }

    pub fn assign(&mut self, token_name: String, value: Box<dyn Any>){
        if self.values.contains_key(token_name.as_str()){
            println!("{} stored", token_name);
            self.values.insert(token_name.clone(), value);
            return;
        }
        if let Some(enclosing_env) = &mut self.enclosing{
             enclosing_env.borrow_mut().assign(token_name.clone(), value);
        }

       println!("not found at all: {}", token_name.clone());
        panic!("undefined variable {}", token_name.clone())


    }


    pub fn get<'a>(& 'a self, token_name: &  Token) -> & 'a Box<dyn Any>{
        println!("requested: {}", token_name.lexeme.clone());
        if self.values.contains_key(token_name.lexeme.as_str()){
            println!("found: {}", token_name.lexeme.clone());
            let value =  self.values.get(token_name.lexeme.as_str());
            if let Some(val) = value{
                  return val;
            }
        }

        if let Some(enclosing_env) = &self.enclosing{
          enclosing_env.borrow_mut().get(token_name);

        }

        println!("not found at all: {}", token_name.lexeme.clone());
        panic!("undefined variable {}", token_name.lexeme)

    }
}