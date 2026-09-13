//! Node: `cxx:Function:Luau.Analysis:Analysis/src/Type.cpp:271:is_overloaded_function`
//! Source: `Analysis/src/Type.cpp:271-283` (hand-ported)

use crate::{
  functions::{
    flatten_intersection::flatten_intersection, follow_type::follow, get_type_alt_j::get,
  },
  records::{function_type::FunctionType, intersection_type::IntersectionType},
  type_aliases::type_id::TypeId,
};

pub fn is_overloaded_function(ty: TypeId) -> bool {
  if get::<IntersectionType>(follow(ty)).is_none() {
    return false;
  }

  let parts = flatten_intersection(ty);
  parts
    .iter()
    .all(|&part| !get::<FunctionType>(part).is_none())
}
