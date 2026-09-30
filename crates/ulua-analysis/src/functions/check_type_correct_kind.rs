use core::ptr::NonNull;

use ulua_ast::records::{ast_node::AstNode, position::Position};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::type_correct_kind::TypeCorrectKind,
  functions::{
    check_type_match::check_type_match, find_expected_type_at::find_expected_type_at, first::first,
    follow_type, get_type,
  },
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, function_type::FunctionType,
    intersection_type::IntersectionType, module::Module, type_arena::TypeArena,
  },
  type_aliases::type_id::TypeId,
};

/// C++ `static TypeCorrectKind checkTypeCorrectKind(...)` (AutocompleteCore.cpp:209-258).
///
/// 前提由调用点（AutocompleteCore 查询链）构造保证：`type_arena` 是本次调用栈内
/// 可供下游 Normalizer/Subtyping 写入的类型 arena 句柄；`node` 为 ancestry 末位的
/// AST arena 存活节点指针（cpp `nodes.back()` 直 deref 同位，此处经 checked
/// `NonNull` 物化，null 输入由 UB 收敛为 panic）；`ty` 为同一 arena 中可 follow
/// 的存活 TypeId；`builtin_types` 为进程级长寿单例。
pub fn check_type_correct_kind(
  module: &Module,
  type_arena: Handle<TypeArena>,
  builtin_types: &BuiltinTypes,
  node: *mut AstNode,
  position: Position,
  ty: TypeId,
) -> TypeCorrectKind {
  let ty = follow_type::follow(ty);

  LUAU_ASSERT!(module.has_module_scope());

  let module_scope = module.get_module_scope();

  // SAFETY: node 为 ancestry 末位、parser bump arena（地址不移动）中存活的非空
  // 节点，比本调用长寿；共享借用只供 find_expected_type_at 只读遍历，期间无并发
  // 可变借用，单线程独占。
  let node = unsafe {
    NonNull::new(node)
      .expect("node 非空（cpp nodes.back() 直 deref 同位）")
      .as_ref()
  };

  let type_at_position = find_expected_type_at(module, node, position);

  let type_at_position = match type_at_position {
    Some(t) => t,
    None => return TypeCorrectKind::None,
  };

  let expected_type = follow_type::follow(type_at_position);

  let builtin_types_handle = Handle::from_ref(builtin_types);

  // `checkFunctionType` lambda from C++: suggest functions whose first return
  // type matches the expected type.
  let check_function_type = |ftv: &FunctionType| -> bool {
    if let Some(first_ret_ty) = first(ftv.ret_types, true) {
      // first_ret_ty/expected_type 均为 arena 存活 TypeId；module_scope 局部分身
      // 保活至本调用返回；type_arena 与 builtin_types 句柄原样透传本函数入口
      // 参数（文档已确立独占与长寿），被调方写入仅落在 arena 堆块与自身
      // normalizer 缓存，对 BuiltinTypes 结构只读；单线程串行。
      return check_type_match(
        module,
        first_ret_ty,
        expected_type,
        &module_scope,
        type_arena,
        builtin_types_handle,
      );
    }
    false
  };

  // We also want to suggest functions that return compatible result
  if let Some(ftv) = get_type::get::<FunctionType>(ty) {
    if check_function_type(ftv) {
      return TypeCorrectKind::CorrectFunctionResult;
    }
  } else if let Some(itv) = get_type::get::<IntersectionType>(ty) {
    for &id in &itv.parts {
      let id = follow_type::follow(id);

      if let Some(ftv) = get_type::get::<FunctionType>(id)
        && check_function_type(ftv)
      {
        return TypeCorrectKind::CorrectFunctionResult;
      }
    }
  }

  // 同闭包内首处调用——ty/expected_type 均已 follow 为 arena 存活 TypeId；
  // module_scope 局部分身保活；type_arena/builtin_types 句柄入口原样透传。
  if check_type_match(
    module,
    ty,
    expected_type,
    &module_scope,
    type_arena,
    builtin_types_handle,
  ) {
    TypeCorrectKind::Correct
  } else {
    TypeCorrectKind::None
  }
}
