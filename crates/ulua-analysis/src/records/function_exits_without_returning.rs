use crate::type_aliases::type_pack_id::TypePackId;
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionExitsWithoutReturning {
  pub(crate) expected_return_type: TypePackId,
}
