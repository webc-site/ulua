use alloc::{vec, vec::Vec};

use crate::{
  functions::{
    as_mutable_type::as_mutable_type_id, as_mutable_type_pack_alt_d::as_mutable_type_pack,
    extend_type_pack::extend_type_pack, follow_type::follow_type_id, freeze_table::freeze_table,
    get_type_alt_j::get_type_id,
  },
  records::{
    blocked_type::BlockedType, magic_function_call_context::MagicFunctionCallContext, scope::Scope,
    type_pack::TypePack,
  },
  type_aliases::{type_id::TypeId, type_pack_variant::TypePackVariant, type_variant::TypeVariant},
};
pub fn magic_freeze_infer(context: &MagicFunctionCallContext) -> bool {
  let solver = unsafe { context.solver.as_ref() };
  let arena = unsafe { &mut *solver.arena };
  let dfg = solver.dfg;
  let scope: *mut Scope = unsafe { (*context.constraint.as_ptr()).scope };

  let call_site = unsafe { context.call_site.as_ref() };

  let extended = unsafe {
    extend_type_pack(
      arena,
      solver.builtin_types,
      context.arguments,
      1,
      Vec::new(),
    )
  };
  let param_types = extended.head;
  if param_types.is_empty() || call_site.args.size == 0 {
    return false;
  }

  let input_type = follow_type_id(param_types[0]);

  let target_expr = unsafe { *call_site.args.data.add(0) };
  let result_def = unsafe { (*dfg).get_def_optional(target_expr) };
  let result_ty: Option<TypeId> = match result_def {
    Some(def) => unsafe { (*scope).lookup_def_id(def) },
    None => None,
  };

  if let Some(result_ty) = result_ty
    && get_type_id::<BlockedType>(follow_type_id(result_ty)).is_none()
  {
    // If there's an existing result type, but it's _not_ blocked, then
    // we aren't type stating this builtin and should fall back to
    // regular inference.
    return false;
  }

  let frozen_type = freeze_table(input_type, context);

  // At this point: we know for sure that if `resultTy` exists, it is a
  // blocked type, and can safely emplace it.
  let builtin_types = unsafe { &*solver.builtin_types };
  if frozen_type.is_none() {
    if let Some(result_ty) = result_ty {
      unsafe {
        (*as_mutable_type_id(result_ty)).ty = TypeVariant::Bound(builtin_types.error_type);
      }
    }
    let result_mut = as_mutable_type_pack(context.result);
    unsafe {
      (*result_mut).ty = TypePackVariant::Bound(builtin_types.error_type_pack);
    }

    return true;
  }

  let frozen_type = frozen_type.unwrap();
  if let Some(result_ty) = result_ty {
    unsafe {
      (*as_mutable_type_id(result_ty)).ty = TypeVariant::Bound(frozen_type);
    }
  }
  let frozen_pack = arena.add_type_pack_t(TypePack {
    head: vec![frozen_type],
    tail: None,
  });
  let result_mut = as_mutable_type_pack(context.result);
  unsafe {
    (*result_mut).ty = TypePackVariant::Bound(frozen_pack);
  }

  true
}
