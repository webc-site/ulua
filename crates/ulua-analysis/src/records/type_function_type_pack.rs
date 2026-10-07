use alloc::vec::Vec;

use crate::type_aliases::{
  type_function_type_id::TypeFunctionTypeId, type_function_type_pack_id::TypeFunctionTypePackId,
};

#[derive(Debug, Clone)]
pub struct TypeFunctionTypePack {
  pub(crate) head: Vec<TypeFunctionTypeId>,
  pub(crate) tail: Option<TypeFunctionTypePackId>,
}
