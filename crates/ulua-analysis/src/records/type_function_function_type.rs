use alloc::{string::String, vec::Vec};

use crate::type_aliases::{
  type_function_type_id::TypeFunctionTypeId, type_function_type_pack_id::TypeFunctionTypePackId,
};

#[derive(Debug, Clone)]
pub struct TypeFunctionFunctionType {
  pub(crate) generics: Vec<TypeFunctionTypeId>,
  pub(crate) generic_packs: Vec<TypeFunctionTypePackId>,
  pub(crate) arg_types: TypeFunctionTypePackId,
  pub(crate) ret_types: TypeFunctionTypePackId,
  pub(crate) arg_names: Vec<Option<String>>,
}
