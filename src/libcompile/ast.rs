use std::rc::Rc;
use std::vec::Vec;
use std::collections::HashMap;
#[derive(Clone, RustcEncodable, PartialEq, Eq, Hash, Debug, Copy)]
pub struct Name(pub u32);

#[derive(Default)]
pub struct TreeStorage {
  names: HashMap<Rc<String>, Name>,
  strings: Vec<Rc<String>>,
}

impl TreeStorage {
  pub fn new() -> Self {
    TreeStorage::default()
  }

  pub fn store(&mut self, string: String) -> Name {
    if let Some(&name) = self.names.get(&string) {
      return name;
    }

    let name = Name(self.strings.len() as u32);
    let str_ptr = Rc::new(string);
    self.strings.push(str_ptr.clone());
    self.names.insert(str_ptr.clone(), name);

    name
  }

  pub fn get(&self, name: Name) -> Rc<String> {
    self.strings[name.0 as usize].clone()
  }
}

pub enum ExprKind {
  Lit(Rc<Lit>),
  Call(Rc<Expr>, Vec<Rc<Expr>>),
  If(Rc<Expr>, Rc<Block>, Rc<Expr>),
}

pub struct Expr {
  pub node: ExprKind,
}

pub struct Block {
  pub stmts: Vec<Stmt>,
}

pub enum StmtKind {
  Local(Rc<Local>),
  Expr(Rc<Expr>),
}

pub struct Stmt {
  pub node: StmtKind,
}

pub struct Local {
  pub init: Option<Rc<Expr>>,
}

pub enum LitKind {
  Int(i64),
}

pub struct Lit {
  pub node: LitKind,
}