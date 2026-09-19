//! Source: `CodeGen/src/EmitCommonX64.h:88` — `inline constexpr OperandX64
//! sClosure = qword[rsp + kStackOffsetToLocals + 0]` (Closure* cl)。
//! 原先在 emit_return / build_entry / emit_inst_call / check_safe_env 各复制
//! 一份 fn, 收敛于此。

use crate::{
  enums::size_x_64::SizeX64,
  functions::get_full_stack_size::K_STACK_OFFSET_TO_LOCALS,
  records::{operand_x_64::OperandX64, register_x_64::RegisterX64},
};

#[inline]
pub fn s_closure() -> OperandX64 {
  OperandX64::mem(
    SizeX64::Qword,
    RegisterX64::NOREG,
    1,
    RegisterX64::RSP,
    K_STACK_OFFSET_TO_LOCALS as i32,
  )
}
