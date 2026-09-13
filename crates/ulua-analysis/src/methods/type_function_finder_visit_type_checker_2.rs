use crate::{
  records::{
    type_function_finder::TypeFunctionFinder, type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::type_id::TypeId,
};

impl TypeFunctionFinder {
  pub fn visit_type_id_type_function_instance_type(
    &mut self,
    ty: TypeId,
    _instance: &TypeFunctionInstanceType,
  ) -> bool {
    self.mentioned_functions.insert(ty);
    true
  }
}
