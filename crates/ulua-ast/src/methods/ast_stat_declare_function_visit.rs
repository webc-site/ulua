use crate::{
  functions::visit_type_list::visit_type_list,
  records::{ast_stat_declare_function::AstStatDeclareFunction, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_type_pack_visit_ref},
};

impl_visitable!(
  AstStatDeclareFunction,
  StatDeclareFunction,
  |this, visitor| {
    visit_type_list(visitor, &this.params);

    // ret_types 槽已句柄化（declare 文法对缺省返回类型现场补建显式空 pack，恒非空）：
    // get_mut 交出独占子节点引用，引用门面递归。
    ast_type_pack_visit_ref(this.ret_types.get_mut(), visitor);
  }
);
