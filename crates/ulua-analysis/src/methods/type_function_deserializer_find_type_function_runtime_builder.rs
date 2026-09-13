use crate::{
  records::type_function_deserializer::TypeFunctionDeserializer,
  type_aliases::{type_function_type_id::TypeFunctionTypeId, type_id::TypeId},
};

impl TypeFunctionDeserializer {
  pub fn find_type_function_type_id(&self, ty: TypeFunctionTypeId) -> Option<TypeId> {
    self.types.get(&ty).copied()
  }
}
