use crate::{
  records::{
    type_arena::TypeArena, type_function::TypeFunction,
    type_function_instance_type::TypeFunctionInstanceType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TypeArena {
  pub fn add_type_function_type_function_vector_type_id_vector_type_pack_id(
    &mut self,
    function: &TypeFunction,
    type_arguments: Vec<TypeId>,
    pack_arguments: Vec<TypePackId>,
  ) -> TypeId {
    self.add_type(TypeFunctionInstanceType::new_with_pack_args(
      function,
      type_arguments,
      pack_arguments,
    ))
  }
}
