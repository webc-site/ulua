use alloc::{collections::BTreeSet, vec::Vec};
use core::ptr::null;

use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};

use crate::{
  enums::reduction::Reduction,
  functions::{compute_keys_of::compute_keys_of, follow_type::follow_type_id},
  records::{
    singleton_type::SingletonType, string_singleton::StringSingleton,
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult, union_type::UnionType,
  },
  type_aliases::{singleton_variant::SingletonVariant, type_id::TypeId, type_pack_id::TypePackId},
};
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn keyof_function_impl(
  type_params: Vec<TypeId>,
  pack_params: Vec<TypePackId>,
  ctx: *mut TypeFunctionContext,
  is_raw: bool,
) -> TypeFunctionReductionResult {
  let ctx_ref = unsafe { &*ctx };
  if type_params.len() != 1 || !pack_params.is_empty() {
    unsafe {
      (*ctx_ref.ice.as_ptr()).ice_string(
                "keyof type function: encountered a type function instance without the required argument structure",
            )
    };
    LUAU_ASSERT!(false);
  }

  let make_result = |result, reduction_status| TypeFunctionReductionResult {
    result,
    reduction_status,
    blocked_types: Vec::new(),
    blocked_packs: Vec::new(),
    error: None,
    messages: Vec::new(),
  };

  let operand_ty = follow_type_id(type_params[0]);
  let Some(norm_ty) = (unsafe { (*ctx_ref.normalizer.as_ptr()).try_normalize(operand_ty) }) else {
    return make_result(None, Reduction::MaybeOk);
  };

  if norm_ty.has_tables() == norm_ty.has_extern_types() {
    return make_result(None, Reduction::Erroneous);
  }

  if norm_ty.has_tops()
    || norm_ty.has_booleans()
    || norm_ty.has_errors()
    || norm_ty.has_nils()
    || norm_ty.has_numbers()
    || norm_ty.has_strings()
    || norm_ty.has_threads()
    || norm_ty.has_buffers()
    || norm_ty.has_functions()
    || norm_ty.has_tyvars()
  {
    return make_result(None, Reduction::Erroneous);
  }

  let mut keys = BTreeSet::new();

  if norm_ty.has_extern_types() {
    LUAU_ASSERT!(!norm_ty.has_tables());
    let mut seen: DenseHashSet<TypeId> = DenseHashSet::new(null());

    let mut extern_types = norm_ty.extern_types.ordering.iter().copied();
    let Some(first) = extern_types.next() else {
      return make_result(None, Reduction::Erroneous);
    };

    if !compute_keys_of(first, &mut keys, &mut seen, is_raw, unsafe { &*ctx }) {
      return make_result(
        Some(unsafe { ctx_ref.builtins.as_ref().string_type }),
        Reduction::MaybeOk,
      );
    }

    for extern_ty in extern_types {
      seen.clear();
      let mut local_keys = BTreeSet::new();
      if compute_keys_of(extern_ty, &mut local_keys, &mut seen, is_raw, unsafe {
        &*ctx
      }) {
        keys.retain(|key| local_keys.contains(key));
      }
    }
  }

  if norm_ty.has_tables() {
    LUAU_ASSERT!(!norm_ty.has_extern_types());
    let mut seen: DenseHashSet<TypeId> = DenseHashSet::new(null());

    let mut tables = norm_ty.tables.order.iter().copied();
    let Some(first) = tables.next() else {
      return make_result(None, Reduction::Erroneous);
    };

    if !compute_keys_of(first, &mut keys, &mut seen, is_raw, unsafe { &*ctx }) {
      return make_result(
        Some(unsafe { ctx_ref.builtins.as_ref().string_type }),
        Reduction::MaybeOk,
      );
    }

    for table in tables {
      seen.clear();
      let mut local_keys = BTreeSet::new();
      if compute_keys_of(table, &mut local_keys, &mut seen, is_raw, unsafe { &*ctx }) {
        keys.retain(|key| local_keys.contains(key));
      }
    }
  }

  if keys.is_empty() {
    return make_result(
      Some(unsafe { ctx_ref.builtins.as_ref().never_type }),
      Reduction::MaybeOk,
    );
  }

  let mut singletons = Vec::new();
  for key in keys {
    singletons.push(unsafe {
      (*ctx_ref.arena.as_ptr()).add_type(SingletonType::new(SingletonVariant::V1(
        StringSingleton::new(key),
      )))
    });
  }

  if singletons.len() == 1 {
    return make_result(Some(singletons[0]), Reduction::MaybeOk);
  }

  make_result(
    Some(unsafe {
      (*ctx_ref.arena.as_ptr()).add_type(UnionType {
        options: singletons,
      })
    }),
    Reduction::MaybeOk,
  )
}
