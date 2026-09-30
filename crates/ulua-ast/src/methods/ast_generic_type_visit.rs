use crate::{
  records::{ast_generic_type::AstGenericType, ast_visitor::AstVisitor, node_handle::OptNode},
  visit::{AstNodeRefMut, AstVisitable, ast_type_visit_ref},
};

impl_visitable!(AstGenericType, GenericType, |this, visitor| {
  // default_value 是 `Option<NonNull<AstType>>` 可空槽（None 即 cpp 的 nullptr）：
  // 经 `OptNode` 句柄取独占借用后走 safe 引用门面，本调用点不再折 null 哨兵、
  // 不再开 unsafe（null 的构造与解引用只留在 records/node_handle.rs 一处）。
  if let Some(ty) = OptNode::from_non_null(this.default_value).get_mut() {
    ast_type_visit_ref(ty, visitor);
  }
});
