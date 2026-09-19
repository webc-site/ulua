use crate::{
  records::{
    type_function_deserializer::TypeFunctionDeserializer,
    type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState,
  },
  type_aliases::{type_function_type_id::TypeFunctionTypeId, type_id::TypeId},
};

pub(crate) fn deserialize_type_function_type_id_type_function_runtime_builder_state(
  ty: TypeFunctionTypeId,
  state: *mut TypeFunctionRuntimeBuilderState,
) -> TypeId {
  let mut deserializer = TypeFunctionDeserializer::default();
  unsafe { deserializer.type_function_deserializer(state) };
  deserializer.deserialize_type_function_type_id(ty)
}
