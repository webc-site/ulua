//! Source: `CodeGen/src/EmitCommonX64.h:89` — `inline constexpr OperandX64 sCode =
//! qword[rsp + kStackOffsetToLocals + 8]` (Instruction* code)。
//! 原先在 7 个 emit/lowering 文件里各复制一份 fn, 收敛于此。

use crate::{
  enums::size_x_64::SizeX64,
  functions::get_full_stack_size::K_STACK_OFFSET_TO_LOCALS,
  records::{operand_x_64::OperandX64, register_x_64::RegisterX64},
};

#[inline]
pub fn s_code() -> OperandX64 {
  OperandX64::mem(
    SizeX64::Qword,
    RegisterX64::NOREG,
    1,
    RegisterX64::RSP,
    K_STACK_OFFSET_TO_LOCALS as i32 + 8,
  )
}
