use std::cmp::max;

use ulua_common::enums::luau_opcode::LuauOpcode;

use crate::{
  enums::{bc_block_flag::BcBlockFlag, bc_op_kind::BcOpKind},
  records::{
    bc_block::BcBlock, bc_block_edge::BcBlockEdge, bc_function::VmConst, bc_op::BcOp,
    bc_return::BcReturn, call_inliner::CallInliner,
  },
};

impl<'a> CallInliner<'a> {
  /// cpp `migrateBlocks(BcRef<BcBlock>& nextBlock)`：把目标图的块/边/指令搬进调用者
  /// 预留好的槽位。`next_block_op` 为调用者侧紧跟内联区后的块句柄。
  pub fn migrate_blocks(&mut self, next_block_op: BcOp) -> bool {
    let call_block = self.call_block_op();
    let insn_block_sort_key = self.caller.block_op(call_block).sortkey;
    let insn_block_chain_key = self.caller.block_op(call_block).chainkey;
    let mut max_chain_key = 0;

    for i in 0..self.target.blocks.len() {
      let target_block_sortkey = self.target.blocks[i].sortkey;
      let target_block_ops: Vec<BcOp> = self.target.blocks[i].ops.iter().cloned().collect();

      let caller_block_idx = (self.caller_blocks_size_before_inline + i as u32) as usize;
      let caller_block_op = BcOp::bc_op_bc_op_kind_u32(
        BcOpKind::Block,
        self.caller_blocks_size_before_inline + i as u32,
      );

      if i as u32 == self.target.exit_block.index {
        let caller_block = &mut self.caller.blocks[caller_block_idx];
        caller_block.sortkey = BcBlock::K_BLOCK_NO_START_PC;
        caller_block.flags |= BcBlockFlag::Dead as u8;
        continue;
      }

      {
        let caller_block = &mut self.caller.blocks[caller_block_idx];
        caller_block.sortkey = insn_block_sort_key;
        caller_block.chainkey = insn_block_chain_key + target_block_sortkey;
        max_chain_key = max(caller_block.chainkey, max_chain_key);
      }

      // Migrate successors
      let target_successors = self.target.blocks[i].successors.clone();
      for e in &target_successors {
        if e.target != self.target.exit_block {
          let mapped_target = self.map_block_op(e.target);
          let caller_block = &mut self.caller.blocks[caller_block_idx];
          caller_block.successors.push_back(BcBlockEdge {
            kind: e.kind,
            target: mapped_target,
          });
        }
      }

      // Migrate predecessors
      let target_predecessors = self.target.blocks[i].predecessors.clone();
      for e in &target_predecessors {
        let mapped_target = self.map_block_op(e.target);
        let caller_block = &mut self.caller.blocks[caller_block_idx];
        caller_block.predecessors.push_back(BcBlockEdge {
          kind: e.kind,
          target: mapped_target,
        });
      }

      // Migrate instructions
      for op in target_block_ops {
        let inst_op_code = self.target.inst_op(op).op;
        if inst_op_code == LuauOpcode::LOP_GETVARARGS {
          self.replace_get_var_arg(caller_block_op, op);
        } else if inst_op_code == LuauOpcode::LOP_RETURN {
          // cpp：此处只校验变长返回并登记站点，替换延后到 migrateBlockPhis 之后
          let return_count = BcReturn::<VmConst>::from(self.target, op).return_count();
          if return_count < 0 {
            return false;
          }
          self.return_sites.push((caller_block_op, op));
        } else if inst_op_code != LuauOpcode::LOP_PREPVARARGS {
          // cpp `else if (inst.op != LOP_PREPVARARGS)`：被内联函数的 PREPVARARGS
          // 携带的是 *被内联函数* 的 numparams，搬进调用方就成了非法指令
          //（`validateInstructions` 断言 `LUAU_INSN_A == func.numparams` 会失败，
          // VM 侧也会按错误的实参个数重排 L->top），必须整条丢弃。
          let caller_inst_op = self.map_inst_op(op);
          self.caller.blocks[caller_block_idx].append_instruction(caller_inst_op);
          self.caller.inst_op(caller_inst_op).block = caller_block_op;
        }
      }
    }

    self.caller.block_op(call_block).chainkey = max_chain_key + 1;
    self.caller.block_op(next_block_op).chainkey = max_chain_key + 2;

    true
  }
}
