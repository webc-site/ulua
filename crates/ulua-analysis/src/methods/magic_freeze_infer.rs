use alloc::vec::Vec;

use crate::{
  functions::{
    as_mutable_type::as_mutable_type_id, as_mutable_type_pack::as_mutable_type_pack,
    extend_type_pack::extend_type_pack, follow_type, freeze_table::freeze_table, get_type,
  },
  records::{
    arena_handle::Handle, blocked_type::BlockedType,
    magic_function_call_context::MagicFunctionCallContext, scope::Scope, type_pack::TypePack,
  },
  type_aliases::{type_id::TypeId, type_pack_variant::TypePackVariant, type_variant::TypeVariant},
};
pub fn magic_freeze_infer(context: &MagicFunctionCallContext) -> bool {
  // Safety: `context.solver` 是派发本约束时以 NotNull 语义注入的求解器地址，
  // 指向调用栈内存活、覆盖本次 infer 全程的 ConstraintSolver。
  let solver = unsafe { context.solver.as_ref() };
  // Safety: `solver.arena` 构造期指向 solver 内嵌 TypeArena，本次 infer 的
  // 类型/类型包分配是它的唯一可变借用路径。
  let arena = { &mut solver.arena.get_mut() };
  let dfg = solver.dfg;
  // Safety: `context.constraint` 指向 solver 当前正在求解的存活 Constraint
  // 节点，仅读取其 `scope` 字段值（一个 arena 持有的 *mut Scope）。
  let scope: *mut Scope = unsafe { (*context.constraint.as_ptr()).scope };

  // Safety: `context.call_site` 指向 parser 拥有、求解期间稳定的当前
  // AstExprCall，本函数只读其 args。
  let call_site = unsafe { context.call_site.as_ref() };

  // Safety: `extend_type_pack` 要求各裸指针入参有效——`arena` 为上一步
  // 独占借出的 &mut TypeArena；`solver.builtin_types` 为会话级 NotNull
  // 内置类型表；`context.arguments` 是本次调用已知的存活 TypePackId。
  let extended = unsafe {
    extend_type_pack(
      arena,
      Handle::from_ptr(solver.builtin_types.as_ptr()),
      context.arguments,
      1,
      Vec::new(),
    )
  };
  let param_types = extended.head;
  if param_types.is_empty() || call_site.args.size == 0 {
    return false;
  }

  let input_type = follow_type::follow(param_types[0]);

  // args.size != 0 由上面的早退保证，[0] 恒在界内，安全切片读取元素指针。
  let target_expr = call_site.args.as_slice()[0];
  // Safety: `solver.dfg` 在求解启动时接线指向本次调用期间稳定存活的
  // DataFlowGraph（magic infer 仅在 ConstraintSolver 求解流程内被派发），
  // `get_def_optional` 只按表达式指针查 astDefs，不写图。
  let result_def = unsafe { (*dfg).get_def_optional(target_expr) };
  let result_ty: Option<TypeId> = match result_def {
    // Safety: `scope` 是从存活 Constraint 读出的当前作用域指针，求解期间稳定，
    // `lookup_def_id` 为 &self 只读，返回的 TypeId 指向 arena 驻留节点。
    Some(def) => unsafe { (*scope).lookup_def_id(def) },
    None => None,
  };

  if let Some(result_ty) = result_ty
    && get_type::get::<BlockedType>(follow_type::follow(result_ty)).is_none()
  {
    // If there's an existing result type, but it's _not_ blocked, then
    // we aren't type stating this builtin and should fall back to
    // regular inference.
    return false;
  }

  let frozen_type = freeze_table(input_type, context);

  // At this point: we know for sure that if `resultTy` exists, it is a
  // blocked type, and can safely emplace it.
  // Safety: `solver.builtin_types` 为会话级 NotNull 内置类型表，比本次调用
  // 长寿且此处只读若干 TypeId 值。
  let builtin_types = { &solver.builtin_types.get_mut() };
  // 双写合一：原 `is_none()` 块 + 块后 `unwrap()` 收为 let-else，None 走
  // error 兜底块、Some 直接绑定，逐分支行为等价。
  let Some(frozen_type) = frozen_type else {
    if let Some(result_ty) = result_ty {
      // Safety: 上面已确认 result_ty 存在且为 Blocked 节点，指向 arena 内
      // 该 TypeId 的存活 Type；Type→*mut 对应 cpp `asMutable` 的 const_cast，
      // 此刻 magic infer 独占把其变体覆盖为 Bound。
      unsafe {
        (*as_mutable_type_id(result_ty)).ty = TypeVariant::Bound(builtin_types.error_type);
      }
    }
    let result_mut = as_mutable_type_pack(context.result);
    // Safety: `context.result` 是待定版的 arena TypePackVar，const_cast 后由
    // 本次 infer 独占写 Bound(error_type_pack)，写点前后无并发读旧值。
    unsafe {
      (*result_mut).ty = TypePackVariant::Bound(builtin_types.error_type_pack);
    }

    return true;
  };
  if let Some(result_ty) = result_ty {
    // Safety: 同 error 分支——result_ty 为已确认 Blocked 的存活 arena Type，
    // const_cast 后独占写 Bound(frozen_type)。
    unsafe {
      (*as_mutable_type_id(result_ty)).ty = TypeVariant::Bound(frozen_type);
    }
  }
  let frozen_pack = arena.add_type_pack_t(TypePack::single(frozen_type));
  let result_mut = as_mutable_type_pack(context.result);
  // Safety: `context.result` 为待定版 arena TypePackVar，const_cast 后由本次
  // infer 独占写 Bound(frozen_pack)。
  unsafe {
    (*result_mut).ty = TypePackVariant::Bound(frozen_pack);
  }

  true
}
