use core::ptr::null_mut;

use crate::{
  functions::is_subtype_normalized_string::is_subtype_normalized_string,
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes,
    internal_error_reporter::InternalErrorReporter, normalized_string_type::NormalizedStringType,
    normalizer::Normalizer, scope::Scope, subtyping::Subtyping, type_arena::TypeArena,
    type_function_runtime::TypeFunctionRuntime,
  },
  type_aliases::type_id::TypeId,
};

pub fn is_subtype_normalized_string_type_normalized_string_type(
  sub_str: &NormalizedStringType,
  super_str: &NormalizedStringType,
) -> bool {
  is_subtype_normalized_string(sub_str, super_str)
}

pub fn is_subtype(
  sub_ty: TypeId,
  super_ty: TypeId,
  arena: Handle<TypeArena>,
  builtin_types: Handle<BuiltinTypes>,
  scope: *mut Scope,
  normalizer: Option<Handle<Normalizer>>,
  type_function_runtime: Handle<TypeFunctionRuntime>,
  reporter: Handle<InternalErrorReporter>,
) -> bool {
  let mut subtyping = Subtyping::subtyping_owned(
    builtin_types,
    arena,
    normalizer.map_or(null_mut(), |n| n.as_ptr()),
    type_function_runtime.as_ptr(),
    reporter.as_ptr(),
  );
  subtyping
    .is_subtype_type_id_type_id_not_null_scope(sub_ty, super_ty, scope)
    .is_subtype
}
