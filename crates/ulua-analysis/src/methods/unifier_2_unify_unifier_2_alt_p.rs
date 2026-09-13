use crate::{
  enums::unify_result::UnifyResult,
  records::{any_type::AnyType, metatable_type::MetatableType, unifier_2::Unifier2},
};

impl Unifier2 {
  pub fn unify_any_type_metatable_type(
    &mut self,
    _sub_any: &AnyType,
    super_metatable: &MetatableType,
  ) -> UnifyResult {
    let builtin_types = unsafe { &*self.builtin_types.as_ptr() };
    let metatable_result =
      self.unify_type_id_type_id(builtin_types.any_type, super_metatable.metatable);
    if metatable_result != UnifyResult::Ok {
      return metatable_result;
    }
    self.unify_type_id_type_id(builtin_types.any_type, super_metatable.table)
  }
}
