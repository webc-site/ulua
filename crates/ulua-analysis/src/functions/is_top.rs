use crate::records::{builtin_types::BuiltinTypes, normalized_extern_type::NormalizedExternType};

pub fn is_top(builtin_types: &BuiltinTypes, extern_types: &NormalizedExternType) -> bool {
  if extern_types.extern_types.len() != 1 {
    return false;
  }

  let (first_ty, first_types) = extern_types
    .extern_types
    .iter()
    .next()
    .expect("len() checked above");

  if *first_ty != builtin_types.extern_type {
    return false;
  }

  if !first_types.empty() {
    return false;
  }

  true
}
