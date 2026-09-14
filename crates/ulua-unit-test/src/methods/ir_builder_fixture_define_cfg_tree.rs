//! @interface-stub
use alloc::vec::Vec;

use ulua_code_gen::{
  enums::ir_block_kind::IrBlockKind,
  functions::{
    compute_cfg_dominance_tree_children::compute_cfg_dominance_tree_children,
    compute_cfg_immediate_dominators::compute_cfg_immediate_dominators, successors::successors,
  },
};

use crate::records::ir_builder_fixture::IrBuilderFixture;

impl IrBuilderFixture {
  pub fn define_cfg_tree(&mut self, successor_sets: &Vec<Vec<u32>>) {
    for successor_set in successor_sets {
      let block = self.build.block(IrBlockKind::Internal);
      self.build.begin_block(block);

      self
        .build
        .function
        .cfg
        .successors_offsets
        .push(self.build.function.cfg.successors.len() as u32);
      self
        .build
        .function
        .cfg
        .successors
        .extend(successor_set.iter().copied());
    }

    // 单次遍历：按块收集前驱，替代对每个目标块重扫全部后继的 O(n²) 循环
    let block_count = self.build.function.blocks.len();
    let mut preds: Vec<Vec<u32>> = vec![Vec::new(); block_count];
    for k in 0..block_count {
      for succ_idx in successors(&self.build.function.cfg, k as u32) {
        preds[succ_idx as usize].push(k as u32);
      }
    }

    for p in &preds {
      self
        .build
        .function
        .cfg
        .predecessors_offsets
        .push(self.build.function.cfg.predecessors.len() as u32);
      self
        .build
        .function
        .cfg
        .predecessors
        .extend(p.iter().copied());
    }

    compute_cfg_immediate_dominators(&mut self.build.function);
    compute_cfg_dominance_tree_children(&mut self.build.function);
  }
}
