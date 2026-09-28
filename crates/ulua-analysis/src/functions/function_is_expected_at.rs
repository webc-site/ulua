use ulua_ast::records::{ast_node::AstNode, position::Position};

use crate::{
  functions::{
    find_expected_type_at::find_expected_type_at, follow_type, get_type,
    return_first_nonnull_option_of_type::return_first_nonnull_option_of_type,
  },
  records::{
    function_type::FunctionType, intersection_type::IntersectionType, module::Module,
    union_type::UnionType,
  },
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn function_is_expected_at(
  module: &Module,
  node: *mut AstNode,
  position: Position,
) -> Option<bool> {
  // SAFETY: node 来自 ancestry，有效性由调用方契约保证
  let type_at_position = find_expected_type_at(module, unsafe { &*node }, position)?;
  let expected_type = follow_type::follow(type_at_position);

  if get_type::get::<FunctionType>(expected_type).is_some() {
    return Some(true);
  }

  if let Some(itv) = get_type::get::<IntersectionType>(expected_type).as_ref() {
    for part in &itv.parts {
      if get_type::get::<FunctionType>(follow_type::follow(*part)).is_none() {
        return Some(false);
      }
    }
    return Some(true);
  }

  if let Some(utv) = get_type::get::<UnionType>(expected_type).as_ref() {
    return Some(return_first_nonnull_option_of_type::<FunctionType>(utv).is_some());
  }

  Some(false)
}
