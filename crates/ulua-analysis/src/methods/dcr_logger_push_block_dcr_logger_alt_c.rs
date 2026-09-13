use crate::{
  records::{constraint::Constraint, dcr_logger::DcrLogger},
  type_aliases::constraint_block_target::ConstraintBlockTarget,
};

impl DcrLogger {
  pub fn push_block_not_null_constraint_not_null_constraint(
    &mut self,
    constraint: *const Constraint,
    block: *const Constraint,
  ) {
    self
      .constraint_blocks
      .get_or_insert(constraint)
      .push(ConstraintBlockTarget::V2(block));
  }
}
