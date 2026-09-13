use ulua_common::{
  functions::{is_jump_d::isJumpD, is_skip_c::isSkipC},
  macros::luau_assert::LUAU_ASSERT,
};

use crate::records::{
  bc_block::BcBlock, bytecode_graph_serializer::BytecodeGraphSerializer, jump_info::JumpInfo,
};

impl<'a> BytecodeGraphSerializer<'a> {
  pub fn patch_jump(&mut self, jump: &JumpInfo) {
    let target = self.func.block_op(jump.target_block);
    LUAU_ASSERT!(target.startpc != BcBlock::K_BLOCK_NO_START_PC);

    if isJumpD(jump.op) {
      let patched = self
        .bcb
        .patch_jump_d(jump.instruction_pc as usize, target.startpc as usize);
      LUAU_ASSERT!(patched);
    } else if isSkipC(jump.op) {
      let patched = self
        .bcb
        .patch_skip_c(jump.instruction_pc as usize, target.startpc as usize);
      LUAU_ASSERT!(patched);
    }
  }
}
