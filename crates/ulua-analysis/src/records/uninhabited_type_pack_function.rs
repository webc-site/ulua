use crate::type_aliases::type_pack_id::TypePackId;
#[derive(Debug, Clone, PartialEq)]
pub struct UninhabitedTypePackFunction {
  pub(crate) tp: TypePackId,
}
