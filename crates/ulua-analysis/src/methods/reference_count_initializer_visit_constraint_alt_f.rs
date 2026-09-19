use crate::{
  records::{
    reference_count_initializer::ReferenceCountInitializer,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::type_id::TypeId,
};

impl ReferenceCountInitializer {
  pub fn visit_type_id_type_function_instance_type(
    &mut self,
    _ty: TypeId,
    tfit: &TypeFunctionInstanceType,
  ) -> bool {
    unsafe { (*tfit.function.as_ptr()).can_reduce_generics }
  }
}
