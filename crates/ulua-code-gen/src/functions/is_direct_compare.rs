use ulua_common::macros::{
  luau_insn_a::luau_insn_a, luau_insn_b::luau_insn_b, luau_insn_c::luau_insn_c,
  luau_insn_d::luau_insn_d, luau_insn_op::luau_insn_op,
};
use ulua_vm::records::proto::Proto;

use crate::type_aliases::instruction_ir_builder::Instruction;

const LOP_LOADB: u32 = 3;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn is_direct_compare(proto: *const Proto, pc: *const Instruction, i: i32) -> bool {
  let proto = unsafe { &*proto };
  if i + 3 < proto.sizecode {
    let pc_val = unsafe { *pc };
    if luau_insn_d(pc_val) == 2 {
      let load_true = unsafe { *pc.add(2) };
      let load_false = unsafe { *pc.add(3) };

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
