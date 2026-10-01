use crate::{
  records::{ast_type_table::AstTypeTable, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_type_visit, ast_type_visit_ref},
};

impl_visitable!(AstTypeTable, TypeTable, |this, visitor| {
  for prop in this.props.iter() {
    // Safety: prop.r#type 指向 arena 中存活节点（null 由 dispatch 内部短路）；
    // 该槽位的句柄化属后续波次（AstTableProp 消费面更宽）。
    unsafe { ast_type_visit(prop.r#type, visitor) };
  }

  // indexer 已句柄化（node_handle）：空槽折叠进 `get_mut` 的 Option，
  // index/result 两子节点经引用门面递归，全链路无裸指针。
  if let Some(indexer) = this.indexer.get_mut() {
    ast_type_visit_ref(indexer.index_type.get_mut(), visitor);
    ast_type_visit_ref(indexer.result_type.get_mut(), visitor);
  }
});
