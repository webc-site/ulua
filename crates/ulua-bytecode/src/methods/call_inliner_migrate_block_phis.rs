use crate::records::call_inliner::CallInliner;

impl<'a> CallInliner<'a> {
  /// cpp `CallInliner::migrateBlockPhis`：把 target 各块锚定的 phi 映射后挂到 caller 对应块。
  /// 独立于 migrateBlocks：phi 可能引用 GETVARARGS 投影，需等 MOVE 在 migrateBlocks 中物化后再映射。
  pub fn migrate_block_phis(&mut self) {
    for i in 0..self.target.blocks.len() {
      let target_phis = self.target.blocks[i].phis.clone();
      let caller_block_idx = self.caller_blocks_size_before_inline as usize + i;
      for phi_op in target_phis {
        let caller_phi_op = self.map_to_caller_op(phi_op);
        self.caller.blocks[caller_block_idx]
          .phis
          .push(caller_phi_op);
      }
    }
  }
}
