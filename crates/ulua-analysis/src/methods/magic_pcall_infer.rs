use alloc::vec;

use crate::{
  functions::{
    as_mutable_type_pack::as_mutable_type_pack, flatten_type_pack::flatten_type_pack_id,
    follow_type, get_type,
  },
  records::{
    function_type::FunctionType, magic_function_call_context::MagicFunctionCallContext,
    type_pack::TypePack,
  },
  type_aliases::type_pack_variant::TypePackVariant,
};

pub fn magic_pcall_infer(ctx: &MagicFunctionCallContext) -> bool {
  let (arg_head, _arg_tail) = flatten_type_pack_id(ctx.arguments);

  if arg_head.is_empty() {
    return false;
  }

  let fn_ty = follow_type::follow(arg_head[0]);
  let Some(fn_ptr) = get_type::get::<FunctionType>(fn_ty) else {
    return false;
  };

  let (fn_return_head, fn_return_tail) = flatten_type_pack_id(fn_ptr.ret_types);
  if !fn_return_head.is_empty() || fn_return_tail.is_some() {
    return false;
  }

  // Safety: `ctx.solver` 是 magic 求解入口以 NonNull 登记的 `ConstraintSolver`，
  // 指向本次 infer 期间存活、覆盖调用全程的求解器；`as_ref` 仅取其只读共享引用。
  let solver = unsafe { ctx.solver.as_ref() };
  // Safety: `solver.builtin_types` 为会话级 NotNull 内置类型表，非空且比本次调用长寿，
  // 此处只读若干 TypeId 值；`solver.arena` 为构造期接线的非空 TypeArena，块地址稳定，
  // 本次 infer 单线程独占，`add_type_pack_t` 的追加是其唯一可变借用路径，无别名冲突。
  let res = {
    let builtin_types = &solver.builtin_types.get_mut();
    solver
      .arena
      .get_mut()
      .add_type_pack_t(TypePack::from_vec(vec![
        builtin_types.boolean_type,
        builtin_types.unknown_type,
      ]))
  };

  let result_mut = as_mutable_type_pack(ctx.result);
  // Safety: `ctx.result` 是 solver 为本约束分配、尚待定版的 arena TypePackVar，
  // TypePackId→*mut 对应 cpp `asMutable` 的 const_cast，本次 infer 独占写其 `ty` 为 Bound。
  unsafe {
    (*result_mut).ty = TypePackVariant::Bound(res);
  }

  true
}
