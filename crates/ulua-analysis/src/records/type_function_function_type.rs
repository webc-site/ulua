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

impl TypeFunctionFunctionType {
  pub fn new(
    generics: Vec<TypeFunctionTypeId>,
    generic_packs: Vec<TypeFunctionTypePackId>,
    arg_types: TypeFunctionTypePackId,
    ret_types: TypeFunctionTypePackId,
    arg_names: Vec<Option<String>>,
  ) -> Self {
    Self {
      generics,
      generic_packs,
      arg_types,
      ret_types,
      arg_names,
    }
  }
}
