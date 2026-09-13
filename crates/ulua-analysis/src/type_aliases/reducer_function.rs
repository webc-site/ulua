extern crate alloc;

use alloc::vec::Vec;

use crate::{
  records::{
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

// C++ (TypeFunction.h:124-125): std::function<TypeFunctionReductionResult<T>(
// T, const std::vector<TypeId>&, const std::vector<TypePackId>&,
// NotNull<TypeFunctionContext>)>. Defaulted to TypeId so the bare
// `ReducerFunction` in TypeFunction matches C++ `ReducerFunction<TypeId>`;
// TypePackFunction instantiates `ReducerFunction<TypePackId>`.
//
// The concrete reducer fns in `functions/*_type_function.rs` all share the
// signature `unsafe fn(T, Vec<TypeId>, Vec<TypePackId>, *mut TypeFunctionContext)
// -> TypeFunctionReductionResult` (by-value vectors, raw ctx pointer dereferenced
// inside the body — clippy `not_unsafe_ptr_arg_deref` requires the fn to be
// marked `unsafe`, mirroring C++ where the unsafety is implicit). The alias
// matches that exactly so the fns are assignable to the `reducer` field of
// `TypeFunction`/`TypePackFunction` without a cast — this is the project's
// MagicFunction-style fn-pointer wiring.
pub type ReducerFunction<T = TypeId> = unsafe fn(
  T,
  Vec<TypeId>,
  Vec<TypePackId>,
  *mut TypeFunctionContext,
) -> TypeFunctionReductionResult;
