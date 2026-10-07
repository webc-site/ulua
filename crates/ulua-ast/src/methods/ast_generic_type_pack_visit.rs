use crate::{
  records::{
    ast_generic_type_pack::AstGenericTypePack, ast_visitor::AstVisitor, node_handle::OptNode,
  },
  visit::{AstNodeRefMut, AstVisitable, ast_type_pack_visit_ref},
};

impl_visitable!(AstGenericTypePack, GenericTypePack, |this, visitor| {
  // default_value 是 `Option<NonNull<AstTypePack>>` 可空槽（None 即 cpp 的 nullptr）：
  // 经 `OptNode` 句柄取独占借用后走 safe 引用门面，本调用点不再折 null 哨兵、
  // 不再开 unsafe（同 [`crate::methods::ast_generic_type_visit`]）。
  if let Some(pack) = OptNode::from_non_null(this.default_value).get_mut() {
    ast_type_pack_visit_ref(pack, visitor);
  }
});
