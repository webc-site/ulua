use crate::records::{compiler::Compiler, const_upvalue_visitor::ConstUpvalueVisitor};

impl Compiler {
  pub fn const_upvalue_visitor_const_upvalue_visitor(&mut self) -> ConstUpvalueVisitor {
    ConstUpvalueVisitor {
      self_: self,
      upvals: Vec::new(),
    }
  }
}
