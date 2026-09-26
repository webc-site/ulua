use crate::{
  functions::{follow_type, get_type},
  records::{
    never_type::NeverType, non_strict_type_checker::NonStrictTypeChecker,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::type_id::TypeId,
};

impl NonStrictTypeChecker {
  pub fn should_skip_runtime_error_testing(&mut self, test: TypeId) -> bool {
    let t = follow_type::follow(test);

    get_type::get::<NeverType>(t).is_some()
      || get_type::get::<TypeFunctionInstanceType>(t).is_some()
  }
}
