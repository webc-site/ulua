use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    contains_generics::ContainsGenerics, iterative_type_visitor::IterativeTypeVisitorTrait,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

pub fn contains_generic_type_id_not_null_dense_hash_set_void(
  ty: TypeId,
  generics: *mut DenseHashSet<*const ()>,
) -> bool {
  let mut cg = ContainsGenerics::contains_generics_contains_generics(generics);
  cg.run_type_id(ty);
  cg.found
}

pub fn contains_generic(tp: TypePackId, generics: *mut DenseHashSet<*const ()>) -> bool {
  let mut cg = ContainsGenerics::contains_generics_contains_generics(generics);
  cg.run_type_pack_id(tp);
  cg.found
}
