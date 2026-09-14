use alloc::{collections::BTreeMap, string::String};

use crate::{
  records::{
    type_function_property::TypeFunctionProperty,
    type_function_table_indexer::TypeFunctionTableIndexer,
  },
  type_aliases::{type_function_type_id::TypeFunctionTypeId, type_id::TypeId},
};

#[derive(Debug, Clone)]
pub struct TypeFunctionExternType {
  pub(crate) props: BTreeMap<String, TypeFunctionProperty>,
  pub(crate) indexer: Option<TypeFunctionTableIndexer>,
  pub(crate) metatable: Option<TypeFunctionTypeId>,
  pub(crate) read_parent: Option<TypeFunctionTypeId>,
  pub(crate) write_parent: Option<TypeFunctionTypeId>,
  pub(crate) extern_ty: TypeId,
}
