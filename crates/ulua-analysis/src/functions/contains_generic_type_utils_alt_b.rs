use core::ffi::c_void;

use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    contains_generics::ContainsGenerics, iterative_type_visitor::IterativeTypeVisitorTrait,
  },
  type_aliases::type_pack_id::TypePackId,
};
pub fn contains_generic(tp: TypePackId, generics: *mut DenseHashSet<*const c_void>) -> bool {
  let mut cg = ContainsGenerics::contains_generics_contains_generics(generics);
  cg.run_type_pack_id(tp);
  cg.found
}
