use crate::{
  records::{ast_type_table::AstTypeTable, ast_visitor::AstVisitor, node_handle::OptNode},
  visit::{AstNodeRefMut, AstVisitable, ast_type_visit_ref},
};

impl_visitable!(AstTypeTable, TypeTable, |this, visitor| {
  // prop.r#type 槽位的句柄化属 records 波次；null 折叠与解引用经 `OptNode`
  // 句柄边界完成，调用点无 unsafe。
  for prop in this.props.iter() {
    if let Some(ty) = OptNode::from_ptr(prop.r#type).get_mut() {
      ast_type_visit_ref(ty, visitor);
    }
  }

  // indexer 已句柄化（node_handle）：空槽折叠进 `get_mut` 的 Option，
  // index/result 两子节点经引用门面递归，全链路无裸指针。
  if let Some(indexer) = this.indexer.get_mut() {
    ast_type_visit_ref(indexer.index_type.get_mut(), visitor);
    ast_type_visit_ref(indexer.result_type.get_mut(), visitor);
  }
});
