use crate::{
  enums::unify_result::UnifyResult,
  records::{any_type::AnyType, function_type::FunctionType, unifier_2::Unifier2},
};

impl Unifier2 {
  pub fn unify_function_type_any_type(
    &mut self,
    sub_fn: &FunctionType,
    _super_any: &AnyType,
  ) -> UnifyResult {
    let builtin_types = unsafe { &*self.builtin_types.as_ptr() };
    let arg_result =
      self.unify_type_pack_id_type_pack_id(builtin_types.any_type_pack, sub_fn.arg_types);
    let ret_result =
      self.unify_type_pack_id_type_pack_id(sub_fn.ret_types, builtin_types.any_type_pack);
    arg_result & ret_result
  }
}
