use alloc::vec;
use core::ptr::NonNull;

use ulua_ast::{
  records::ast_expr_constant_string::AstExprConstantString, rtti::ast_node_try_as_ptr,
};

use crate::{
  functions::{
    as_mutable_type_pack::as_mutable_type_pack, flatten_type_pack::flatten_type_pack_id,
    parse_pattern_string::parse_pattern_string_bytes,
  },
  records::{
    magic_function_call_context::MagicFunctionCallContext, type_pack::TypePack,
    union_type::UnionType,
  },
  type_aliases::type_pack_variant::TypePackVariant,
};
pub fn magic_match_infer(context: &MagicFunctionCallContext) -> bool {
  let (params, _tail) = flatten_type_pack_id(context.arguments);

  if params.len() < 2 || params.len() > 3 {
    return false;
  }

  // Safety: `context.solver` 是构造该 MagicFunctionCallContext 时以 NonNull 语义
  // 写入的 ConstraintSolver 地址，指向本次 infer 调用栈上存活的求解器。
  let solver = unsafe { context.solver.as_ref() };
  // Safety: `solver.arena` 由求解器构造契约指向其内嵌 TypeArena，非空；本次 infer
  // 期间对该 arena 的唯一可变借用即此 `&mut`，add_type/add_type_pack_t 串行使用。
  let arena = { &mut solver.arena.get_mut() };
  // Safety: `context.call_site` 为派发约束时注入的 NonNull<AstExprCall>，指向
  // parser 拥有、求解全程稳定的当前调用节点，此处仅读 self_/args。
  let call_site = unsafe { context.call_site.as_ref() };

  let pattern_index = if call_site.self_ { 0 } else { 1 };
  let pattern = if call_site.args.size > pattern_index {
    // size > pattern_index 保证下标在界内，改用安全切片取元素指针。
    let expr = call_site.args.as_slice()[pattern_index];
    // Safety: `expr` 是 arena 写入 args 的存活表达式节点（或 null）；try_as_ptr 先
    // 判空再按 class_index 甄别，未命中返回 None 且从不解引用，命中即 repr(C) 基址
    // 重合的存活 AstExprConstantString 只读借用。
    unsafe { ast_node_try_as_ptr::<AstExprConstantString>(expr) }
  } else {
    None
  };

  let Some(pattern) = pattern else {
    return false;
  };

  // Safety: `solver.builtin_types` 为会话级 NotNull 内置类型表，非空且比本次
  // 调用长寿，new_unchecked 的 NonNull 不变量由该契约直接成立；`pattern` 经
  // try_as_ptr 命中即指向存活 AstExprConstantString，其 `value` 为 arena 成对
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

  // Safety: `context.solver.as_ptr()` 取回仍被本调用独占存活的求解器地址；
  // `constraint_solver_unify` 需要 `&mut self`，此刻无其他借用（单线程串行）。
  // `builtin_types` 只读句柄值 string_type，约束指针交由 callee 按其契约解引用。
  unsafe {
    let builtin_types = &solver.builtin_types.get_mut();
    (*context.solver.as_ptr()).constraint_solver_unify(
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
    // Safety: 同首处 unify——solver 地址在本调用内独占，unify 的 `&mut` 借用
    // 即时结束，无第二处并发别名。
    unsafe {
      (*context.solver.as_ptr()).constraint_solver_unify(
        context.constraint.as_ptr(),
        params[2],
        optional_number,
      );
    }
  }

  let return_list = arena.add_type_pack_t(TypePack::from_vec(return_types));
  let result_mut = as_mutable_type_pack(context.result);
  // Safety: `context.result` 是派发本约束时在 arena 中预留的待定 TypePackVar，
  // TypePackId 即其裸地址（as_mutable_type_pack 语义），写入 Bound 尾是求解器
  // 对该占位包的唯一写路径，且此刻无并发读。
  unsafe {
    (*result_mut).ty = TypePackVariant::Bound(return_list);
  }

  true
}
