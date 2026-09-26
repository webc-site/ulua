use crate::{
  records::{constraint::Constraint, dcr_logger::DcrLogger},
  type_aliases::{
    constraint_block_target::ConstraintBlockTarget, type_id::TypeId, type_pack_id::TypePackId,
  },
};

impl DcrLogger {
  pub fn pop_block_type_id(&mut self, block: TypeId) {
    for (_, list) in self.constraint_blocks.iter_mut() {
      list.retain(|target| {
        if let ConstraintBlockTarget::V0(target_block) = target {
          *target_block != block
        } else {
          true
        }
      });
    }
  }

  pub fn pop_block_type_pack_id(&mut self, block: TypePackId) {
    for (_, list) in self.constraint_blocks.iter_mut() {
      list.retain(|target| {
        if let ConstraintBlockTarget::V1(target_block) = target {
          *target_block != block
        } else {
          true
        }
      });
    }
  }

  pub fn pop_block_not_null_constraint(&mut self, block: *const Constraint) {
    for (_, list) in self.constraint_blocks.iter_mut() {
      list.retain(|target| {
        if let ConstraintBlockTarget::V2(target_block) = target {
          *target_block != block
        } else {
          true
        }
      });
    }
  }
}
