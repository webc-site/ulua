use alloc::vec;
use core::ptr::{NonNull, null_mut};

use ulua_ast::{
  records::{ast_expr_constant_string::AstExprConstantString, node_handle::OptNode},
  rtti::ast_node_try_as,
};

use crate::{
  functions::{
    as_mutable_type_pack::as_mutable_type_pack, flatten_type_pack::flatten_type_pack_id,
    parse_pattern_string::parse_pattern_string_bytes,
  },
  records::{
    arena_handle::{alias, alias_ref},
    magic_function_call_context::MagicFunctionCallContext,
    type_pack::TypePack,
    union_type::UnionType,
  },
  type_aliases::type_pack_variant::TypePackVariant,
};
pub fn magic_match_infer(context: &MagicFunctionCallContext) -> bool {
  let (params, _tail) = flatten_type_pack_id(context.arguments);

  if params.len() < 2 || params.len() > 3 {
    return false;
  }

  let solver = alias_ref(context.solver.as_ptr());
  let arena = { &mut solver.arena.get_mut() };
  let call_site = alias_ref(context.call_site.as_ptr());

  let pattern_index = if call_site.self_ { 0 } else { 1 };
  // `args` 槽位仍是裸指针：size 守卫在界内后取元素指针，经句柄门面
  // `OptNode::from_ptr` 把可空性与「缺位」一并折叠为 `Option`，判型下转走
  // 生命周期正确的 [`ast_node_try_as`]，借用半径由本函数局部句柄供给。
  let pattern_node = if call_site.args.size > pattern_index {
    OptNode::from_ptr(call_site.args.as_slice()[pattern_index])
  } else {
    OptNode::from_ptr(null_mut())
  };
  let Some(pattern) = pattern_node
    .get()
    .and_then(|p| ast_node_try_as::<AstExprConstantString>(p))
  else {
    return false;
  };

  // Safety: `solver.builtin_types` 为会话级 NotNull 内置类型表，非空且比本次
  // 调用长寿，new_unchecked 的 NonNull 不变量由该契约直接成立；`pattern` 经
  // try_as 命中即指向存活 AstExprConstantString，其 `value` 为 arena 成对
  // 写入的合法字节区，仅只读解析。
  let return_types = unsafe {
    parse_pattern_string_bytes(
      NonNull::new_unchecked(solver.builtin_types.as_ptr()),
      pattern.value.as_bytes(),
    )
  };

  if return_types.is_empty() {
    return false;
  }

  {
    let builtin_types = &solver.builtin_types.get_mut();
    alias(context.solver.as_ptr()).constraint_solver_unify(
      context.constraint.as_ptr(),
      params[0],
      builtin_types.string_type,
    );
  }

  // Safety: `solver.builtin_types` 指向会话级只读内置类型表，借出的 `&` 仅在本
  // 表达式内读取 nil_type/number_type 两个句柄值；`arena` 的可变借用与上方
  // unify 对 solver 的临时 `&mut` 前后串行，不并存。
  let optional_number = {
    let builtin_types = &solver.builtin_types.get_mut();
    arena.add_type(UnionType {
      options: vec![builtin_types.nil_type, builtin_types.number_type],
    })
  };

  let init_index = if call_site.self_ { 1 } else { 2 };
  if params.len() == 3 && call_site.args.size > init_index {
    alias(context.solver.as_ptr()).constraint_solver_unify(
      context.constraint.as_ptr(),
      params[2],
      optional_number,
    );
  }

  let return_list = arena.add_type_pack_t(TypePack::from_vec(return_types));
  let result_mut = as_mutable_type_pack(context.result);
  alias(result_mut).ty = TypePackVariant::Bound(return_list);

  true
}
