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
  // Safety: state 由调用方（evaluateTypeAliasCall 等）传入，即 runtime->runtimeBuilder，
  // 指向构造期把 ctx 接线为 NonNull 的存活 builder state，比本次调用长寿；被调 ctor 仅
  // 把 state/runtime 存为裸字段并经 ctx.as_ref() 只读取出 runtime，deserializer 为栈局
  // 对象、其后的 deserialize 只在本调用内使用这些指针，单线程下无别名借用。
  unsafe { deserializer.type_function_deserializer(state) };
  deserializer.deserialize_type_function_type_id(ty)
}
