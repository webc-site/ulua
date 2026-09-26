use ulua_common::macros::{
  luau_insn_a::luau_insn_a, luau_insn_b::luau_insn_b, luau_insn_c::luau_insn_c,
  luau_insn_d::luau_insn_d, luau_insn_op::luau_insn_op,
};

use crate::type_aliases::instruction_ir_builder::Instruction;

const LOP_LOADB: u32 = 3;

/// JUMPIFEQ/JUMPXEQK* 直比较捷径判定（cpp IrBuilder.cpp isDirectCompare）。
///
/// `code` 为 `proto.code` 字节码只读切片，`i` 为当前指令下标；越界访问由切片索引
/// panic 兜底（C++ 原实现此处为界内裸读，行为等价）。
pub fn is_direct_compare(code: &[Instruction], i: i32) -> bool {
  if i + 3 < code.len() as i32 {
    let pc_val = code[i as usize];
    if luau_insn_d(pc_val) == 2 {
      let load_true = code[i as usize + 2];
      let load_false = code[i as usize + 3];

      if luau_insn_op(load_true) == LOP_LOADB && luau_insn_op(load_false) == LOP_LOADB {
        let same_target = luau_insn_a(load_true) == luau_insn_a(load_false);
        let zero_and_one = luau_insn_b(load_true) == 0 && luau_insn_b(load_false) == 1;
        let correct_jumps = luau_insn_c(load_true) == 1 && luau_insn_c(load_false) == 0;

        return same_target && zero_and_one && correct_jumps;
      }
    }
  }

  false
}
