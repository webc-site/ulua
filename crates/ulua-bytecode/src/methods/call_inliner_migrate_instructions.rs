use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_op::BcOp, call_inliner::CallInliner},
};

impl<'a> CallInliner<'a> {
  /// cpp `migrateInstructions()`：把被内联函数的指令逐条落进调用方预留好的槽位。
  ///
  /// 这条路径跑在内联热循环上，所以只读 `BcInst` 的 `Copy` 字段、操作数按下标现取，
  /// 不再 `clone()` 整条指令（含两个 `SmallVector`）或 `collect` 成 `Vec`。
  pub fn migrate_instructions(&mut self) {
    let target_insn_count = self.target.instructions.len() as u32;

    for i in 0..target_insn_count {
      let target_insn_op = BcOp::bc_op_bc_op_kind_u32(BcOpKind::Inst, i);
      let caller_insn_op =
        BcOp::bc_op_bc_op_kind_u32(BcOpKind::Inst, self.caller_inst_size_before_inline + i);
      let tidx = target_insn_op.index as usize;
      let cidx = caller_insn_op.index as usize;

      let op = self.target.instructions[tidx].op;

      // cpp `if (op == RETURN || op == GETVARARGS || op == PREPVARARGS) continue`：
      // 早退必须在任何拷贝/下钻之前；PREPVARARGS 携带被内联函数的 numparams，
      // 绝不能被复制进调用方的指令槽。
      if op == LuauOpcode::LOP_RETURN
        || op == LuauOpcode::LOP_GETVARARGS
        || op == LuauOpcode::LOP_PREPVARARGS
      {
        continue;
      }

      let block = self.target.instructions[tidx].block;
      LUAU_ASSERT!(block.kind == BcOpKind::Block);
      let mapped_block = BcOp::bc_op_bc_op_kind_u32(
        BcOpKind::Block,
        self.caller_blocks_size_before_inline + block.index,
      );
      self.caller.instructions[cidx].op = op;
      self.caller.instructions[cidx].block = mapped_block;

      let ops_len = self.target.instructions[tidx].ops.len();

      // cpp `isMultiConsumer`：只需读那条立即数操作数，不必快照整条 ops 表。
      // RETURN 已在上面早退，故不再列入 match 分支。
      let imm_idx = match op {
        LuauOpcode::LOP_SETLIST => Some(1),
        LuauOpcode::LOP_CALLFB | LuauOpcode::LOP_CALL => Some(0),
        _ => None,
      };
      let is_multi_consumer = match imm_idx {
        Some(idx) => match self.target.instructions[tidx].ops.as_slice().get(idx) {
          Some(imm_op) => {
            // SAFETY：读 union 字段 value_int 前提是该立即数确为整型
            //（SETLIST/CALL 系列指令的 imm 操作数在图构建时只存整数）。
            unsafe {
              self.target.immediates[imm_op.index as usize]
                .value
                .value_int
                < 0
            }
          }
          None => false,
        },
        None => false,
      };

      // cpp `target.is_vararg && isMultiConsumer(...) && isGetVarArg(ops.back())` 的短路求值：
      // 仅当变参路径成立时才取 `ops.last()`。无输入的操作数（如 LOADNIL，其目标寄存器记在
      // `regs` 而非 `ops`）ops 为空，提前访问会取到不存在的元素。
      let var_arg_tail = self.target.is_vararg && is_multi_consumer && ops_len > 0 && {
        let last = self.target.instructions[tidx].ops.as_slice()[ops_len - 1];
        last.kind == BcOpKind::Inst
          && self.target.instructions[last.index as usize].op == LuauOpcode::LOP_GETVARARGS
      };

      if var_arg_tail {
        let last = self.target.instructions[tidx].ops.as_slice()[ops_len - 1];
        for k in 0..ops_len {
          let inp = self.target.instructions[tidx].ops.as_slice()[k];
          if inp != last {
            let mapped = self.map_to_caller_op(inp);
            self.caller.instructions[cidx].ops.push_back(mapped);
          } else {
            // varArgMoves 条目要留到整场内联结束（`mapToCallerOp` 处理 GETVARARGS
            // 投影时还会经 `getVarArgParam` 再读），故既不 take/remove 也不 clone，
            // 按下标把 Copy 元素直接搬过去。cpp 此处是 LUAU_ASSERT + `[]`；
            // release 下断言是 no-op，旧实现的 `unwrap()` 就是 panic 路径。
            debug_assert!(
              self.var_arg_moves.contains_key(&inp),
              "变参尾操作数必须已登记 varArgMoves"
            );
            let caller_ops = &mut self.caller.instructions[cidx].ops;
            if let Some(moves) = self.var_arg_moves.get(&inp) {
              caller_ops.extend(moves.iter().copied());
            }
          }
        }
        // cpp `makeFixedConsumer(caller, callerInst)`：句柄化后直接把调用方指令的
        // `BcOp` 交给它，内部经 `self.caller` 现取可变视图。
        self.make_fixed_consumer(caller_insn_op);
      } else {
        for k in 0..ops_len {
          let inp = self.target.instructions[tidx].ops.as_slice()[k];
          let mapped = self.map_to_caller_op(inp);
          self.caller.instructions[cidx].ops.push_back(mapped);
        }
      }

      if let Some(reg) = self.target.regs.get(&target_insn_op).copied() {
        let mapped_reg = self.map_to_caller_reg(reg);
        self.caller.regs.insert(caller_insn_op, mapped_reg);
      }
    }
  }
}
