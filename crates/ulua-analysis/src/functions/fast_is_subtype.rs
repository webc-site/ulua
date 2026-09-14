//! Node: `cxx:Function:Luau.Analysis:Analysis/src/TypeUtils.cpp:561:fast_is_subtype`
//! Source: `Analysis/src/TypeUtils.cpp` (TypeUtils.cpp:561-565)

use crate::{
  enums::relation::Relation, functions::relate_simplify_alt_b::relate_type_id_type_id,
  type_aliases::type_id::TypeId,
};

// A fast approximation of sub_ty <: super_ty
pub fn fast_is_subtype(sub_ty: TypeId, super_ty: TypeId) -> bool {
  let r = relate_type_id_type_id(super_ty, sub_ty);
  r == Relation::Coincident || r == Relation::Superset
}
