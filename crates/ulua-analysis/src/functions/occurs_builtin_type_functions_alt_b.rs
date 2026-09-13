//! C++ `bool occurs(TypeId haystack, TypeId needle)`
//! (BuiltinTypeFunctions.cpp:1199-1203): seeds an empty seen-set and delegates
//! to the three-arg recursive `occurs`.
use core::ptr::null;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::occurs_builtin_type_functions::occurs as occurs_with_seen,
  type_aliases::type_id::TypeId,
};
pub fn occurs(haystack: TypeId, needle: TypeId) -> bool {
  let mut seen: DenseHashSet<TypeId> = DenseHashSet::new(null());
  occurs_with_seen(haystack, needle, &mut seen)
}
