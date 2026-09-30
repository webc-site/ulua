use crate::{
  records::{ast_stat_declare_global::AstStatDeclareGlobal, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_type_visit},
};

impl_visitable!(AstStatDeclareGlobal, StatDeclareGlobal, |this, visitor| {
  // Safety: type_ 是 declare 语句解析时 parse_type 分配的 arena 存活节点；ast_type_visit 契约满足，遍历期由 dispatch 独占写穿。
  unsafe {
    ast_type_visit(this.type_, visitor);
  }
});
