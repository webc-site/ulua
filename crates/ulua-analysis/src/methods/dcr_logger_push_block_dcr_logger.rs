use crate::{
  records::{constraint::Constraint, dcr_logger::DcrLogger},
  type_aliases::{constraint_block_target::ConstraintBlockTarget, type_id::TypeId},
};

impl DcrLogger {
  pub fn push_block_not_null_constraint_type_id(
    &mut self,
    constraint: *const Constraint,
    block: TypeId,
  ) {
    self
      .constraint_blocks
      .get_or_insert(constraint)
      .push(ConstraintBlockTarget::V0(block));
  }
}
