use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    any_type::AnyType, intersection_type::IntersectionType, negation_type::NegationType,
    never_type::NeverType, normalizer::Normalizer, union_type::UnionType,
    unknown_type::UnknownType,
  },
  type_aliases::type_id::TypeId,
};

impl Normalizer {
  pub fn negate(&mut self, mut there: TypeId) -> TypeId {
    self.consume_fuel();

    there = follow_type_id(there);

    if get_type_id::<AnyType>(there).is_some() {
      there
    } else if get_type_id::<UnknownType>(there).is_some() {
      unsafe { (*self.builtin_types).never_type }
    } else if get_type_id::<NeverType>(there).is_some() {
      unsafe { (*self.builtin_types).unknown_type }
    } else if let Some(ntv) = get_type_id::<NegationType>(there) {
      ntv.ty
    } else if let Some(utv) = get_type_id::<UnionType>(there) {
      let mut parts = Vec::new();
      for option in &utv.options {
        parts.push(self.negate(*option));
      }
      unsafe { (*self.arena).add_type(IntersectionType { parts }) }
    } else if let Some(itv) = get_type_id::<IntersectionType>(there) {
      let mut options = Vec::new();
      for part in &itv.parts {
        options.push(self.negate(*part));
      }
      unsafe { (*self.arena).add_type(UnionType { options }) }
    } else {
      there
    }
  }
}
