use crate::{
  records::{
    ast_stat_declare_global::AstStatDeclareGlobal, ast_visitor::AstVisitor, node_handle::OptNode,
  },
  visit::{AstNodeRefMut, AstVisitable, ast_type_visit_ref},
};

impl_visitable!(AstStatDeclareGlobal, StatDeclareGlobal, |this, visitor| {
  // type_ 是 declare 语句解析时 parse_type 分配的 arena 存活节点（非空）；
  // 经句柄边界取独占视图后走引用门面，调用点无 unsafe。
  if let Some(ty) = OptNode::from_ptr(this.type_).get_mut() {
    ast_type_visit_ref(ty, visitor);
  }
});
