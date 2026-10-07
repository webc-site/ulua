use crate::{
  records::{
    arena_handle::Handle, type_function_cloner::TypeFunctionCloner,
    type_function_runtime::TypeFunctionRuntime,
  },
  type_aliases::type_function_type_id::TypeFunctionTypeId,
};

pub fn deep_clone(
  runtime: Handle<TypeFunctionRuntime>,
  ty: TypeFunctionTypeId,
) -> TypeFunctionTypeId {
  let mut cloner = TypeFunctionCloner::new(runtime);
  cloner.clone_type_function_type_id(ty)
}
