use alloc::vec::Vec;

use crate::type_aliases::type_id::TypeId;
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TypeFunctionInferenceResult {
  pub operand_inference: Vec<TypeId>,
  pub function_result_inference: TypeId,
}
