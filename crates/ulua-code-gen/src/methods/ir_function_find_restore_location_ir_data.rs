use core::mem::zeroed;

use crate::records::{
  ir_block::IrBlock, ir_function::IrFunction, value_restore_location::ValueRestoreLocation,
};

impl IrFunction {
  pub fn find_restore_location_u32_bool(
    &self,
    inst_idx: u32,
    limit_to_current_block: bool,
  ) -> ValueRestoreLocation {
    if inst_idx >= self.value_restore_ops.len() as u32 {
      return ValueRestoreLocation {
        op: unsafe { zeroed() },
        kind: unsafe { zeroed() },
        conversion_cmd: unsafe { zeroed() },
        lazy: false,
      };
    }

    if limit_to_current_block {
      for &block_idx in &self.valid_restore_op_blocks {
        let block: &IrBlock = &self.blocks[block_idx as usize];

        if inst_idx >= block.start && inst_idx <= block.finish {
          return self.value_restore_ops[inst_idx as usize];
        }
      }

      return ValueRestoreLocation {
        op: unsafe { zeroed() },
        kind: unsafe { zeroed() },
        conversion_cmd: unsafe { zeroed() },
        lazy: false,
      };
    }

    self.value_restore_ops[inst_idx as usize]
  }
}
