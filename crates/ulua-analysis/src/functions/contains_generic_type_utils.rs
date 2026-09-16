use core::ffi::c_void;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    contains_generics::ContainsGenerics, iterative_type_visitor::IterativeTypeVisitorTrait,
  },
  type_aliases::type_id::TypeId,
};
pub fn contains_generic_type_id_not_null_dense_hash_set_void(
  ty: TypeId,
  generics: *mut DenseHashSet<*const c_void>,
) -> bool {
  let mut cg = ContainsGenerics::contains_generics_contains_generics(generics);
  cg.run_type_id(ty);
  cg.found
}
