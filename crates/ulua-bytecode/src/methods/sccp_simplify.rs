use std::vec::Vec;

use ulua_common::{
  macros::luau_assert::LUAU_ASSERT,
  records::{dense_hash_set::DenseHashSet, small_vector::SmallVector},
};

use crate::{
  enums::{bc_block_flag::BcBlockFlag, bc_op_kind::BcOpKind},
  records::{
    bc_op::BcOp,
    sccp::{Sccp, VmConstOps},
  },
};

impl<I: VmConstOps + ?Sized> Sccp<'_, '_, I> {
  /// cpp `Sccp::simplifyPhis`：消解平凡 phi（全部操作数相同），使用点重定向到唯一操作数。
  pub(crate) fn simplify_phis(&mut self) {
    let visited_blocks: Vec<BcOp> = self.state.flow_worklist_set.iter().cloned().collect();

    for block_op in visited_blocks {
      LUAU_ASSERT!(block_op.kind == BcOpKind::Block);
      let mut phi_idx = 0;
      while let Some(&phi_op) = self.func.block(block_op).operator_deref().phis.get(phi_idx) {
        LUAU_ASSERT!(phi_op.kind == BcOpKind::Phi);

        let phi_ops: SmallVector<_, 4> = self.func.phi(phi_op).operator_deref().ops.clone();
        let Some(&unique) = phi_ops.first() else {
          phi_idx += 1;
          continue;
        };
        if !phi_ops.as_slice().iter().all(|&op| op == unique) {
          phi_idx += 1;
          continue;
        }

        // 把使用点重定向到唯一操作数
        let redirect = |ops: &mut SmallVector<BcOp, 4>| {
          for op in ops.as_mut_slice() {
            if *op == phi_op {
              *op = unique;
            }
          }
        };

        for use_op in self.state.uses_of(phi_op).to_vec() {
          match use_op.kind {
            BcOpKind::Inst => redirect(&mut self.func.inst_op(use_op).ops),
            BcOpKind::Phi => redirect(&mut self.func.phi_op(use_op).ops),
            _ => {}
          }
          self.state.record_use(unique, use_op);
        }

        // phi 已消解：出块、清空反向边（remove 后同位下个 phi 前移，不递增）
        self.func.block_op(block_op).phis.remove(phi_idx);
        self.state.op_uses.insert(phi_op, Vec::new());
      }
    }
  }

  /// cpp `Sccp::updateBlockUses`：按 entry 前向可达性标记死块，并回填 useCount。
  pub(crate) fn update_block_uses(&mut self) {
    // 以 entry 出发的前向可达性（而非 blockUses，其结果依赖工作表顺序）标记死块
    let mut reachable: DenseHashSet<u32> = DenseHashSet::new(u32::MAX);
    let entry_idx = self.func.entry_block.index;
    let exit_idx = self.func.exit_block.index;
    reachable.insert(entry_idx);
    reachable.insert(exit_idx);

    let mut worklist: Vec<u32> = vec![entry_idx];
    while let Some(idx) = worklist.pop() {
      for edge in self.func.blocks[idx as usize].successors.as_slice() {
        let succ_idx = edge.target.index;
        LUAU_ASSERT!(edge.target.kind == BcOpKind::Block);
        if !reachable.contains(&succ_idx) {
          reachable.insert(succ_idx);
          worklist.push(succ_idx);
        }
      }
    }

    let block_infos: Vec<(u32, bool)> = self
      .func
      .blocks
      .iter()
      .enumerate()
      .map(|(idx, _)| {
        let idx = idx as u32;
        (
          self.state.block_uses.get_or_insert(idx).len() as u32,
          !reachable.contains(&idx),
        )
      })
      .collect();

    for (block, (use_count, unreachable)) in self.func.blocks.iter_mut().zip(block_infos) {
      block.use_count = use_count;
      if unreachable {
        block.flags |= BcBlockFlag::Dead as u8;
      }
    }
  }
}
