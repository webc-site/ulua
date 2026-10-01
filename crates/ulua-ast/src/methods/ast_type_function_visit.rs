use crate::{
  functions::visit_type_list::visit_type_list,
  records::{ast_type_function::AstTypeFunction, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_type_pack_visit_ref},
};

impl_visitable!(AstTypeFunction, TypeFunction, |this, visitor| {
  visit_type_list(visitor, &this.arg_types);

  // return_types 槽已句柄化（parseReturnType/补建空 pack 恒非空）：get_mut 交出
  // 独占子节点引用，引用门面递归。
  ast_type_pack_visit_ref(this.return_types.get_mut(), visitor);
});
