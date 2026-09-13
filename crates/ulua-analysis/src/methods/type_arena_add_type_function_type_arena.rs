use crate::{
  records::{type_arena::TypeArena, type_function::TypeFunction},
  type_aliases::type_id::TypeId,
};

impl TypeArena {
  pub fn add_type_function_type_function_initializer_list_type_id(
    &mut self,
    function: &TypeFunction,
    types: &[TypeId],
  ) -> TypeId {
    let pack_arguments = Vec::new();
    self.add_type_function_type_function_vector_type_id_vector_type_pack_id(
      function,
      types.to_vec(),
      pack_arguments,
    )
  }
}
