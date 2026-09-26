use crate::records::{has_free_type::HasFreeType, type_once_visitor::TypeOnceVisitor};
pub fn has_free_type_has_free_type() {
  let mut _visitor = HasFreeType {
    base: TypeOnceVisitor::new("TypeOnceVisitor".to_string(), true),
    result: false,
  };

  _visitor.has_free_type_has_free_type();
}
