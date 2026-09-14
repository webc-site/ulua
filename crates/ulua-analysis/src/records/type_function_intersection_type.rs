use alloc::vec::Vec;

use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
#[derive(Debug, Clone)]
pub struct TypeFunctionIntersectionType {
  pub(crate) components: Vec<TypeFunctionTypeId>,
}
