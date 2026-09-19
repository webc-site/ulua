use crate::records::{compiler::Compiler, reg_scope::RegScope};

impl Compiler {
  pub fn reg_scope_compiler(&mut self) -> RegScope {
    RegScope {
      self_: self as *mut Compiler,
      old_top: self.reg_top,
    }
  }
}
