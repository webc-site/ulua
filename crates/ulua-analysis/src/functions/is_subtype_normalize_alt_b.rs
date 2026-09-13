//! Source: `Analysis/src/Normalize.cpp:3749-3762` (hand-ported)
//!
//! Free function `bool isSubtype(TypeId, TypeId, NotNull<TypeArena>,
//! NotNull<BuiltinTypes>, NotNull<Scope>, NotNull<Normalizer>,
//! NotNull<TypeFunctionRuntime>, NotNull<InternalErrorReporter>)`.
use crate::{
  records::{
    builtin_types::BuiltinTypes, internal_error_reporter::InternalErrorReporter,
    normalizer::Normalizer, scope::Scope, subtyping::Subtyping, type_arena::TypeArena,
    type_function_runtime::TypeFunctionRuntime,
  },
  type_aliases::type_id::TypeId,
};

pub fn is_subtype(
  sub_ty: TypeId,
  super_ty: TypeId,
  arena: *mut TypeArena,
  builtin_types: *mut BuiltinTypes,
  scope: *mut Scope,
  normalizer: *mut Normalizer,
  type_function_runtime: *mut TypeFunctionRuntime,
  reporter: *mut InternalErrorReporter,
) -> bool {
  let mut subtyping = Subtyping::subtyping_owned(
    builtin_types,
    arena,
    normalizer,
    type_function_runtime,
    reporter,
  );
  subtyping
    .is_subtype_type_id_type_id_not_null_scope(sub_ty, super_ty, scope)
    .is_subtype
}
