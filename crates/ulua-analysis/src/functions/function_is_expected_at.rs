use ulua_ast::records::{ast_node::AstNode, position::Position};

use crate::{
  functions::{
    find_expected_type_at::find_expected_type_at, follow_type::follow_type_id,
    get_type_alt_j::get_type_id,
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
  let expected_type = follow_type_id(type_at_position);

  unsafe {
    if !get_type_id::<FunctionType>(expected_type).is_none() {
      return Some(true);
    }

    if let Some(itv) = get_type_id::<IntersectionType>(expected_type).as_ref() {
      for part in &itv.parts {
        if get_type_id::<FunctionType>(follow_type_id(*part)).is_none() {
          return Some(false);
        }
      }
      return Some(true);
    }

    if let Some(utv) = get_type_id::<UnionType>(expected_type).as_ref() {
      return Some(return_first_nonnull_option_of_type::<FunctionType>(utv).is_some());
    }

    Some(false)
  }
}
