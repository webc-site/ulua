use crate::{
  records::{ast_stat_declare_extern_type::AstStatDeclareExternType, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_type_visit},
};

impl_visitable!(
  AstStatDeclareExternType,
  StatDeclareExternType,
  |this, visitor| {
    for prop in this.props.iter() {
      // Safety: props 是 TempVector 复制进 arena 的 AstTableIndexer/属性数组，元素 ty 槽为 parser 分配的存活 AstType 或 null（dispatch 内短路）；单线程独占遍历。
      unsafe {
        ast_type_visit(prop.ty, visitor);
      }
    }
  }
);
