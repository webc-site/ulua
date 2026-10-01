use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;

#[derive(Debug, Clone)]
pub struct TypeFunctionTableIndexer {
  pub(crate) key_type: TypeFunctionTypeId,
  pub(crate) value_type: TypeFunctionTypeId,
}
