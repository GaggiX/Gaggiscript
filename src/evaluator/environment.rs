use crate::evaluator::object::Object;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub type EnvRc<'a> = Rc<RefCell<Environment<'a>>>;

#[derive(PartialEq)]
pub struct Environment<'a> {
    pub vars:  HashMap<String, Object<'a>>,
    pub outer: Option<EnvRc<'a>>
}


pub fn new<'a>() -> EnvRc<'a> {
    Rc::new(RefCell::new(Environment{
        vars:  HashMap::new(),
        outer: None
    }))
}

pub fn new_enclosed_environment(outer: EnvRc) -> EnvRc {
    let env = new();
    env.borrow_mut().outer = Some(outer); 
    env
}

impl<'a> Environment<'a> {

    pub fn get(&self, ident: &str) -> Result<Object<'a>, String> {
        match self.vars.get(ident) {
            Some(i) => Ok(i.clone()),
            None    => match &self.outer {
                Some(e) => Ok(e.borrow_mut().get(ident)?),
                None    => Err(format!("Runtime error: {} not found", ident))
            }
        }
    }

    pub fn set(&mut self, ident: &str, obj: Object<'a>) {
        self.vars.insert(ident.to_string(), obj);
    }

}