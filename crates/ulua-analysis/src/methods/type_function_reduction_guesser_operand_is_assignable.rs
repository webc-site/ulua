use crate::{
  functions::get_type,
  records::{
    generic_type::GenericType, type_function_instance_type::TypeFunctionInstanceType,
    type_function_reduction_guesser::TypeFunctionReductionGuesser,
  },
  type_aliases::type_id::TypeId,
};

impl TypeFunctionReductionGuesser {
  pub fn operand_is_assignable(&self, ty: TypeId) -> bool {
    if get_type::get::<TypeFunctionInstanceType>(ty).is_some() {
      return true;
    }
    if get_type::get::<GenericType>(ty).is_some() {
      return true;
    }
    if self.cyclic_instances.contains(&ty) {
      return true;
    }
    false
  }
}
