use crate::records::ast_attr::AstAttr;

impl AstAttr {
  pub fn as_attr(&mut self) -> *mut AstAttr {
    self as *mut AstAttr
  }
}
