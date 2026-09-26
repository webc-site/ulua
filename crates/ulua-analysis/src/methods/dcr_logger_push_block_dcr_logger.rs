use crate::{
  records::{constraint::Constraint, dcr_logger::DcrLogger},
  type_aliases::{
    constraint_block_target::ConstraintBlockTarget, type_id::TypeId, type_pack_id::TypePackId,
  },
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

  pub fn push_block_not_null_constraint_type_pack_id(
    &mut self,
    constraint: *const Constraint,
    block: TypePackId,
  ) {
    self
      .constraint_blocks
      .get_or_insert(constraint)
      .push(ConstraintBlockTarget::V1(block));
  }

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
