use crate::{
  records::{ast_type_table::AstTypeTable, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_type_visit},
};

impl_visitable!(AstTypeTable, TypeTable, |this, visitor| {
  for prop in this.props.iter() {
    // Safety: prop.r#type 指向 arena 中存活节点（null 由 dispatch 内部短路）。
    unsafe { ast_type_visit(prop.r#type, visitor) };
  }

  // Safety: indexer 按 arena 契约为 null 或指向存活 AstTableIndexer；
  // as_ref 折叠判空，null 静默跳过（不 panic、不裸解引用）。
  if let Some(indexer) = unsafe { this.indexer.as_ref() } {
    // Safety: 两指针均指向 arena 中存活 AstType（null 由 dispatch 内部短路）。
    unsafe {
      ast_type_visit(indexer.index_type, visitor);
      ast_type_visit(indexer.result_type, visitor);
    }
  }
});
