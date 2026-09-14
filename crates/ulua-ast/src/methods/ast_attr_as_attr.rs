use crate::records::ast_attr::AstAttr;

impl AstAttr {
  pub fn as_attr(&mut self) -> *mut AstAttr {
    self as *mut AstAttr
  }
}

#[inline]
pub fn ast_attr_as_attr(this: &mut AstAttr) -> *mut AstAttr {
  this.as_attr()
}
