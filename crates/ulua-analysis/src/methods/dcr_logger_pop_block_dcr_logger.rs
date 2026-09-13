use crate::{
  records::dcr_logger::DcrLogger,
  type_aliases::{constraint_block_target::ConstraintBlockTarget, type_id::TypeId},
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
}
