use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{
    generic_type::GenericType, type_function_instance_type::TypeFunctionInstanceType,
    type_function_reduction_guesser::TypeFunctionReductionGuesser,
  },
  type_aliases::type_id::TypeId,
};

impl TypeFunctionReductionGuesser {
  pub fn operand_is_assignable(&self, ty: TypeId) -> bool {
    if !get_type_id::<TypeFunctionInstanceType>(ty).is_none() {
      return true;
    }
    if !get_type_id::<GenericType>(ty).is_none() {
      return true;
    }
    if self.cyclic_instances.contains(&ty) {
      return true;
    }
    false
  }
}
