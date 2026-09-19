use std::vec;

use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  functions::{
    get_jump_target::get_jump_target, get_op_length::get_op_length, is_fast_call::is_fast_call,
  },
  macros::{
    luau_assert::LUAU_ASSERT, luau_insn_b::luau_insn_b, luau_insn_c::luau_insn_c,
    luau_insn_op::luau_insn_op,
  },
};

use crate::records::bytecode_builder::BytecodeBuilder;

impl BytecodeBuilder {
  /// 校验 MULTRET 序列：产出/消费变长序列的指令必须成对出现，
  /// 且序列内部（含消费指令）不得作为跳转目标。
  pub fn validate_variadic(&self) {
    let mut variadic_seq = false;
    let mut insn_targets = vec![false; self.insns.len()];

    // 第一遍：标记全部跳转目标
    let mut insns = self.insns.iter().copied().enumerate();
    while let Some((i, insn)) = insns.next() {
      let op = LuauOpcode::from(luau_insn_op(insn) as u8);

      let target = get_jump_target(insn, i as u32);

      if target >= 0 && !is_fast_call(op) {
        LUAU_ASSERT!((target as usize) < self.insns.len());
        insn_targets[target as usize] = true;
      }

      // 变步长推进：等价 cpp `i += getOpLength(op)`，跳过当前指令的后续槽位
      for _ in 1..get_op_length(op) as usize {
        insns.next();
      }
    }

    // 第二遍：状态机校验 producer/consumer/neutral 的配对关系
    let mut insns = self.insns.iter().copied().enumerate();
    while let Some((i, insn)) = insns.next() {
      let op = LuauOpcode::from(luau_insn_op(insn) as u8);

      if variadic_seq {
        LUAU_ASSERT!(!insn_targets[i]);
      }

      if op == LuauOpcode::LOP_CALL || op == LuauOpcode::LOP_CALLFB {
        // 注意：CALL 可能结束一个变长序列并同时开始新的序列
        if luau_insn_b(insn) == 0 {
          // 消费指令结束变长序列
          LUAU_ASSERT!(variadic_seq);
          variadic_seq = false;
        } else {
          // CALL 非中性指令，序列内只能是消费指令
          LUAU_ASSERT!(!variadic_seq);
        }

        if luau_insn_c(insn) == 0 {
          // 产出指令开启变长序列
          LUAU_ASSERT!(!variadic_seq);
          variadic_seq = true;
        }
      } else if op == LuauOpcode::LOP_GETVARARGS && luau_insn_b(insn) == 0 {
        // 产出指令开启变长序列
        LUAU_ASSERT!(!variadic_seq);
        variadic_seq = true;
      } else if (op == LuauOpcode::LOP_RETURN && luau_insn_b(insn) == 0)
        || (op == LuauOpcode::LOP_SETLIST && luau_insn_c(insn) == 0)
      {
        // 消费指令结束变长序列
        LUAU_ASSERT!(variadic_seq);
        variadic_seq = false;
      } else if op == LuauOpcode::LOP_FASTCALL || op == LuauOpcode::LOP_FASTPCALL {
        let call_target = (i as i32 + luau_insn_c(insn) as i32 + 1) as usize;
        LUAU_ASSERT!(
          call_target < self.insns.len()
            && LuauOpcode::from(luau_insn_op(self.insns[call_target]) as u8)
              == LuauOpcode::LOP_CALL
        );

        if luau_insn_b(self.insns[call_target]) == 0 {
          // 消费指令链接的 CALL 稍后自行结束序列，此处只校验状态
          LUAU_ASSERT!(variadic_seq);
        } else {
          LUAU_ASSERT!(!variadic_seq);
        }
      } else if op == LuauOpcode::LOP_CLOSEUPVALS
        || op == LuauOpcode::LOP_NAMECALL
        || op == LuauOpcode::LOP_NAMECALLUDATA
        || op == LuauOpcode::LOP_GETIMPORT
        || op == LuauOpcode::LOP_MOVE
        || op == LuauOpcode::LOP_GETUPVAL
        || op == LuauOpcode::LOP_GETGLOBAL
        || op == LuauOpcode::LOP_GETTABLEKS
        || op == LuauOpcode::LOP_COVERAGE
      {
        // 变长序列内的中性指令：不改 L->top
      } else {
        LUAU_ASSERT!(!variadic_seq);
      }

      // 变步长推进：等价 cpp `i += getOpLength(op)`，跳过当前指令的后续槽位
      for _ in 1..get_op_length(op) as usize {
        insns.next();
      }
    }

    LUAU_ASSERT!(!variadic_seq);
  }
}
