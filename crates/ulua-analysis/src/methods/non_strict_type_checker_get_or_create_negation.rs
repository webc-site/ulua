use crate::{
  records::{negation_type::NegationType, non_strict_type_checker::NonStrictTypeChecker},
  type_aliases::type_id::TypeId,
};

impl NonStrictTypeChecker {
  pub fn get_or_create_negation(&mut self, base_type: TypeId) -> TypeId {
    let cached_result = self.cached_negations.get_or_insert(base_type);
    if cached_result.is_null() {
      *cached_result = unsafe { (*self.arena).add_type(NegationType { ty: base_type }) };
    }
    *cached_result
  }
}
