use crate::{
  enums::unify_result::UnifyResult,
  records::{any_type::AnyType, function_type::FunctionType, unifier_2::Unifier2},
};

impl Unifier2 {
  pub fn unify_any_type_function_type(
    &mut self,
    _sub_any: &AnyType,
    super_fn: &FunctionType,
  ) -> UnifyResult {
    let builtin_types = unsafe { &*self.builtin_types.as_ptr() };
    let arg_result =
      self.unify_type_pack_id_type_pack_id(super_fn.arg_types, builtin_types.any_type_pack);
    let ret_result =
      self.unify_type_pack_id_type_pack_id(builtin_types.any_type_pack, super_fn.ret_types);
    arg_result & ret_result
  }
}
