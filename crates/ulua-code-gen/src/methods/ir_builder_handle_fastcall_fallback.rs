use ulua_common::macros::luau_insn_c::LUAU_INSN_C;

use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  records::{ir_builder::IrBuilder, ir_op::IrOp},
  type_aliases::instruction_ir_builder::Instruction,
};
impl IrBuilder {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn handle_fastcall_fallback(
    &mut self,
    fallback_or_undef: IrOp,
    pc: *const Instruction,
    i: i32,
  ) {
    let skip = unsafe { LUAU_INSN_C(*pc) } as i32;

    if fallback_or_undef.kind() != IrOpKind::Undef {
      let next = self.block_at_inst((i + skip + 2) as u32);
      self.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
      self.begin_block(fallback_or_undef);

      self.active_fastcall_fallback = true;
      self.fastcall_fallback_return = next;
    } else {
      self.cmd_skip_target = i + skip + 2;
    }
  }
}
