use alloc::sync::Arc;

use ulua_ast::records::{ast_node::AstNode, position::Position};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::type_correct_kind::TypeCorrectKind,
  functions::{
    check_type_match::check_type_match, find_expected_type_at::find_expected_type_at, first::first,
    follow_type::follow_type_id, get_type_alt_j::get_type_id,
  },
  records::{
    builtin_types::BuiltinTypes, function_type::FunctionType, intersection_type::IntersectionType,
    module::Module, scope::Scope, type_arena::TypeArena,
  },
  type_aliases::type_id::TypeId,
};

/// C++ `static TypeCorrectKind checkTypeCorrectKind(...)` (AutocompleteCore.cpp:209-258).
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn check_type_correct_kind(
  module: &Module,
  type_arena: *mut TypeArena,
  builtin_types: &BuiltinTypes,
  node: *mut AstNode,
  position: Position,
  ty: TypeId,
) -> TypeCorrectKind {
  let ty = follow_type_id(ty);

  LUAU_ASSERT!(module.has_module_scope());

  let module_scope = module.get_module_scope();
  let module_scope_ptr = Arc::as_ptr(&module_scope) as *mut Scope;

  // SAFETY: node 来自 ancestry，有效性由调用方契约保证
  let type_at_position = find_expected_type_at(module, unsafe { &*node }, position);

  let type_at_position = match type_at_position {
    Some(t) => t,
    None => return TypeCorrectKind::None,
  };

  let expected_type = follow_type_id(type_at_position);

  let builtin_types_ptr = builtin_types as *const BuiltinTypes as *mut BuiltinTypes;

  // `checkFunctionType` lambda from C++: suggest functions whose first return
  // type matches the expected type.
  let check_function_type = |ftv: &FunctionType| -> bool {
    if let Some(first_ret_ty) = first(ftv.ret_types, true) {
      return unsafe {
        check_type_match(
          module,
          first_ret_ty,
          expected_type,
          module_scope_ptr,
          type_arena,
          builtin_types_ptr,
        )
      };
    }
    false
  };

  // We also want to suggest functions that return compatible result
  if let Some(ftv) = get_type_id::<FunctionType>(ty) {
    if check_function_type(ftv) {
      return TypeCorrectKind::CorrectFunctionResult;
    }
  } else if let Some(itv) = get_type_id::<IntersectionType>(ty) {
    for &id in &itv.parts {
      let id = follow_type_id(id);

      if let Some(ftv) = get_type_id::<FunctionType>(id)
        && check_function_type(ftv)
      {
        return TypeCorrectKind::CorrectFunctionResult;
      }
    }
  }

  if unsafe {
    check_type_match(
      module,
      ty,
      expected_type,
      module_scope_ptr,
      type_arena,
      builtin_types_ptr,
    )
  } {
    TypeCorrectKind::Correct
  } else {
    TypeCorrectKind::None
  }
}
