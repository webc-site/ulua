use crate::{
  functions::visit_type_list::visit_type_list,
  records::{ast_stat_declare_function::AstStatDeclareFunction, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_type_pack_visit},
};

impl_visitable!(
  AstStatDeclareFunction,
  StatDeclareFunction,
  |this, visitor| {
    visit_type_list(visitor, &this.params);

    // Safety: ret_types 为 arena 中存活的 AstTypePack 或 null；ast_type_pack_visit 对 null
    // 内部短路（等价旧守卫）。
    unsafe { ast_type_pack_visit(this.ret_types, visitor) };
  }
);
