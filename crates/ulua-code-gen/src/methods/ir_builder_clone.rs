use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{add_use::add_use, is_pseudo::is_pseudo, kill_ir_utils::kill_ir_function_ir_inst},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_block::K_BLOCK_NO_START_PC, ir_builder::IrBuilder, ir_op::IrOp},
};

impl IrBuilder {
  pub fn clone(&mut self, source_idxs: Vec<u32>, remove_current_terminator: bool) {
    let mut inst_redir: DenseHashMap<u32, u32> = DenseHashMap::new(!0u32);

    for source_idx in source_idxs {
      let source = self.function.blocks[source_idx as usize];

      if remove_current_terminator && self.in_terminated_block {
        let finish = self.function.blocks[self.active_block_idx as usize].finish;
        let term = &mut self.function.instructions[finish as usize] as *mut _;
        unsafe {
          kill_ir_function_ir_inst(&mut self.function, &mut *term);
        }
        self.in_terminated_block = false;
      }

      const K_BLOCK_FLAG_SAFE_ENV_CHECK: u8 = 1 << 0;
      if (source.flags & K_BLOCK_FLAG_SAFE_ENV_CHECK) != 0 {
        CODEGEN_ASSERT!(source.startpc != K_BLOCK_NO_START_PC);
        let exit = self.vm_exit(source.startpc);
        self.inst_ir_cmd_ir_op(IrCmd::CheckSafeEnv, exit);
      }

      for index in source.start..=source.finish {
        CODEGEN_ASSERT!((index as usize) < self.function.instructions.len());
        let mut clone = self.function.instructions[index as usize].clone();

        if is_pseudo(clone.cmd) {
          CODEGEN_ASSERT!(clone.use_count == 0);
          continue;
        }

        for op in clone.ops.as_mut_slice() {
          if op.kind() == IrOpKind::Inst {
            if let Some(&new_index) = inst_redir.find(&op.index()) {
              *op = IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, new_index);
            } else {
              CODEGEN_ASSERT!(false);
            }
          }
        }

        for &op in clone.ops.as_slice() {
          add_use(&mut self.function, op);
        }

        *inst_redir.get_or_insert(index) = self.function.instructions.len() as u32;
        self.inst_ir_cmd_ir_ops(clone.cmd, &clone.ops);
      }
    }
  }
}
