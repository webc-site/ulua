use alloc::{collections::BTreeMap, string::String};

use crate::{
  records::{
    type_function_property::TypeFunctionProperty,
    type_function_table_indexer::TypeFunctionTableIndexer,
  },
  type_aliases::type_function_type_id::TypeFunctionTypeId,
};

#[derive(Debug, Clone)]
pub struct TypeFunctionTableType {
  pub(crate) props: BTreeMap<String, TypeFunctionProperty>,
  pub(crate) indexer: Option<TypeFunctionTableIndexer>,
  pub(crate) metatable: Option<TypeFunctionTypeId>,
}
