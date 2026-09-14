use crate::records::{compiler::Compiler, visitor::Visitor};

impl Compiler {
  pub fn visitor_visitor(&mut self) -> Visitor {
    Visitor {
      self_: self as *mut Compiler,
      conflict: [0; 4],
      assigned: [0; 4],
    }
  }
}
