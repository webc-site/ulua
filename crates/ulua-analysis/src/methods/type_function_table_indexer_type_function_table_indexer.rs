use crate::{
  records::type_function_table_indexer::TypeFunctionTableIndexer,
  type_aliases::type_function_type_id::TypeFunctionTypeId,
};

impl TypeFunctionTableIndexer {
  pub fn new(key_type: TypeFunctionTypeId, value_type: TypeFunctionTypeId) -> Self {
    Self {
      key_type,
      value_type,
    }
  }
}
