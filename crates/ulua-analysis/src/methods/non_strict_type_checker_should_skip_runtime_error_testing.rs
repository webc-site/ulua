use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    never_type::NeverType, non_strict_type_checker::NonStrictTypeChecker,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::type_id::TypeId,
};

impl NonStrictTypeChecker {
  pub fn should_skip_runtime_error_testing(&mut self, test: TypeId) -> bool {
    let t = follow_type_id(test);

    !get_type_id::<NeverType>(t).is_none() || !get_type_id::<TypeFunctionInstanceType>(t).is_none()
  }
}
