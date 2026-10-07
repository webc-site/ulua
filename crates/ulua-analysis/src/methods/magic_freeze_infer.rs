use alloc::vec::Vec;

use crate::{
  functions::{
    as_mutable_type::as_mutable_type_id, as_mutable_type_pack::as_mutable_type_pack,
    extend_type_pack::extend_type_pack, follow_type, freeze_table::freeze_table, get_type,
  },
  records::{
    arena_handle::{Handle, alias, alias_ref},
    blocked_type::BlockedType,
    magic_function_call_context::MagicFunctionCallContext,
    scope::Scope,
    type_pack::TypePack,
  },
  type_aliases::{type_id::TypeId, type_pack_variant::TypePackVariant, type_variant::TypeVariant},
};
pub fn magic_freeze_infer(context: &MagicFunctionCallContext) -> bool {
  let solver = alias_ref(context.solver.as_ptr());
  let arena = { &mut solver.arena.get_mut() };
  let dfg = solver.dfg;
  let scope: *mut Scope = alias_ref(context.constraint.as_ptr()).scope;

  let call_site = alias_ref(context.call_site.as_ptr());

  // extend_type_pack 已 safe 化（形参全为受检类型）。
  let extended = extend_type_pack(
    arena,
    Handle::from_ptr(solver.builtin_types.as_ptr()),
    context.arguments,
    1,
    Vec::new(),
  );
  let param_types = extended.head;
  if param_types.is_empty() || call_site.args.size == 0 {
    return false;
  }

  let input_type = follow_type::follow(param_types[0]);

  // args.size != 0 由上面的早退保证，[0] 恒在界内，安全切片读取元素指针。
  let target_expr = call_site.args.as_slice()[0];
  let result_def = alias_ref(dfg).get_def_optional(target_expr);
  let result_ty: Option<TypeId> = match result_def {
    Some(def) => alias_ref(scope).lookup_def_id(def),
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
      alias(as_mutable_type_id(result_ty)).ty = TypeVariant::Bound(builtin_types.error_type);
    }
    let result_mut = as_mutable_type_pack(context.result);
    alias(result_mut).ty = TypePackVariant::Bound(builtin_types.error_type_pack);

    return true;
  };
  if let Some(result_ty) = result_ty {
    alias(as_mutable_type_id(result_ty)).ty = TypeVariant::Bound(frozen_type);
  }
  let frozen_pack = arena.add_type_pack_t(TypePack::single(frozen_type));
  let result_mut = as_mutable_type_pack(context.result);
  alias(result_mut).ty = TypePackVariant::Bound(frozen_pack);

  true
}
