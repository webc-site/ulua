use crate::{
  records::{
    ast_stat_declare_extern_type::AstStatDeclareExternType, ast_visitor::AstVisitor,
    node_handle::OptNode,
  },
  visit::{AstNodeRefMut, AstVisitable, ast_type_visit_ref},
};

impl_visitable!(
  AstStatDeclareExternType,
  StatDeclareExternType,
  |this, visitor| {
    // props 是 TempVector 复制进 arena 的属性数组，元素 ty 槽为 parser 分配的
    // 存活 AstType 或 null（句柄边界折叠跳过）；调用点无 unsafe。
    for prop in this.props.iter() {
      if let Some(ty) = OptNode::from_ptr(prop.ty).get_mut() {
        ast_type_visit_ref(ty, visitor);
      }
    }
  }
);
