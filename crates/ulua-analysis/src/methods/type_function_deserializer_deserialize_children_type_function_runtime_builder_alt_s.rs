use crate::records::{
  generic_type_pack::GenericTypePack, type_function_deserializer::TypeFunctionDeserializer,
  type_function_generic_type_pack::TypeFunctionGenericTypePack,
};

impl TypeFunctionDeserializer {
  pub fn deserialize_children_type_function_generic_type_pack_generic_type_pack(
    &mut self,
    _v2: *mut TypeFunctionGenericTypePack,
    _v1: *mut GenericTypePack,
  ) {
  }
}
