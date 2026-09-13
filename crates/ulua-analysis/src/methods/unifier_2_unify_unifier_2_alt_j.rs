use crate::{
  enums::unify_result::UnifyResult,
  records::{metatable_type::MetatableType, unifier_2::Unifier2},
};

impl Unifier2 {
  pub fn unify_metatable_type_metatable_type(
    &mut self,
    sub_metatable: &MetatableType,
    super_metatable: &MetatableType,
  ) -> UnifyResult {
    let metatable_result =
      self.unify_type_id_type_id(sub_metatable.metatable, super_metatable.metatable);
    if metatable_result != UnifyResult::Ok {
      return metatable_result;
    }
    self.unify_type_id_type_id(sub_metatable.table, super_metatable.table)
  }
}
