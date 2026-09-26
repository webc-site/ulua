use alloc::string::String;

use crate::type_aliases::constraint_block_target::ConstraintBlockTarget;
#[derive(Debug, Clone)]
pub struct ConstraintBlock {
  pub(crate) target: ConstraintBlockTarget,
  pub(crate) stringification: String,
}
