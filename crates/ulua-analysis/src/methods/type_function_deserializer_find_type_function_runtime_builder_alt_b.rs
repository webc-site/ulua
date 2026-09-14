use crate::{
  records::type_function_deserializer::TypeFunctionDeserializer,
  type_aliases::{type_function_type_pack_id::TypeFunctionTypePackId, type_pack_id::TypePackId},
};

impl TypeFunctionDeserializer {
  pub fn find_type_function_type_pack_id(&self, tp: TypeFunctionTypePackId) -> Option<TypePackId> {
    self.packs.get(&tp).copied()
  }
}
