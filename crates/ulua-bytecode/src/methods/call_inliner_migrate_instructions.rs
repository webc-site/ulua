use alloc::vec::Vec;

use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_op::BcOp, call_inliner::CallInliner},
};

impl<'a> CallInliner<'a> {
  pub fn migrate_instructions(&mut self) {
    for i in 0..self.target.instructions.len() as u32 {
      let target_insn_op = BcOp::bc_op_bc_op_kind_u32(BcOpKind::Inst, i);
      let caller_insn_op =
        BcOp::bc_op_bc_op_kind_u32(BcOpKind::Inst, self.caller_inst_size_before_inline + i);
      let target_inst_data = self.target.inst_op(target_insn_op).clone();
      let target_ops: Vec<BcOp> = target_inst_data.ops.iter().copied().collect();
      let target_reg = self.target.regs.get(&target_insn_op).copied();
      let is_multi_consumer = match target_inst_data.op {
        LuauOpcode::LOP_SETLIST
        | LuauOpcode::LOP_RETURN
        | LuauOpcode::LOP_CALLFB
        | LuauOpcode::LOP_CALL => {
          let imm_op = target_inst_data.ops[if target_inst_data.op == LuauOpcode::LOP_SETLIST {
            1
          } else {
            0
          }];
          // SAFETY：读 union 字段 value_int 前提是该立即数确为整型
          //（SETLIST/CALL 系列指令的 imm 操作数在图构建时只存整数）。
          unsafe {
            self.target.immediates[imm_op.index as usize]
              .value
              .value_int
              < 0
          }
        }
        _ => false,
      };

      // cpp `if (op == RETURN || op == GETVARARGS || op == PREPVARARGS) continue`：
      // PREPVARARGS 携带被内联函数的 numparams，绝不能被复制进调用方的指令槽。
      if target_inst_data.op == LuauOpcode::LOP_RETURN
        || target_inst_data.op == LuauOpcode::LOP_GETVARARGS
        || target_inst_data.op == LuauOpcode::LOP_PREPVARARGS
      {
        continue;
      }

      LUAU_ASSERT!(target_inst_data.block.kind == BcOpKind::Block);
      let mapped_block = BcOp::bc_op_bc_op_kind_u32(
        BcOpKind::Block,
        self.caller_blocks_size_before_inline + target_inst_data.block.index,
      );
      self.caller.instructions[caller_insn_op.index as usize].op = target_inst_data.op;
      self.caller.instructions[caller_insn_op.index as usize].block = mapped_block;

      // cpp `target.is_vararg && isMultiConsumer(...) && isGetVarArg(ops.back())` 的短路求值：
      // 仅当变参路径成立时才取 `ops.last()`。无输入的操作数（如 LOADNIL，其目标寄存器记在
      // `regs` 而非 `ops`）ops 为空，提前访问会取到不存在的元素。
      let var_arg_tail = self.target.is_vararg
        && is_multi_consumer
        && target_ops.last().is_some_and(|last| {
          last.kind == BcOpKind::Inst
            && self.target.instructions[last.index as usize].op == LuauOpcode::LOP_GETVARARGS
        });

      if var_arg_tail {
        let last = target_ops[target_ops.len() - 1];
        for inp in target_ops {
          if inp != last {
            let mapped = self.map_to_caller_op(inp);
            self.caller.instructions[caller_insn_op.index as usize]
              .ops
              .push_back(mapped);
          } else {
            LUAU_ASSERT!(self.var_arg_moves.contains_key(&inp));
            let moves = self.var_arg_moves.get(&inp).unwrap().clone();
            for move_op in moves {
              self.caller.instructions[caller_insn_op.index as usize]
                .ops
                .push_back(move_op);
            }
          }
        }
        // cpp `makeFixedConsumer(caller, callerInst)`：句柄化后直接把调用方指令的
        // `BcOp` 交给它，内部经 `self.caller` 现取可变视图。
        self.make_fixed_consumer(caller_insn_op);
      } else {
        for inp in target_ops {
          let mapped = self.map_to_caller_op(inp);
          self.caller.instructions[caller_insn_op.index as usize]
            .ops
            .push_back(mapped);
        }
      }

      if let Some(reg) = target_reg {
        let mapped_reg = self.map_to_caller_reg(reg);
        self.caller.regs.insert(caller_insn_op, mapped_reg);
      }
    }
  }
}
