use crate::{
  records::type_function_deserializer::TypeFunctionDeserializer,
  type_aliases::{
    type_function_type_id::TypeFunctionTypeId, type_function_type_pack_id::TypeFunctionTypePackId,
    type_id::TypeId, type_pack_id::TypePackId,
  },
};

impl TypeFunctionDeserializer {
  pub fn find_type_function_type_id(&self, ty: TypeFunctionTypeId) -> Option<TypeId> {
    self.types.get(&ty).copied()
  }

  pub fn find_type_function_type_pack_id(&self, tp: TypeFunctionTypePackId) -> Option<TypePackId> {
    self.packs.get(&tp).copied()
  }
}
