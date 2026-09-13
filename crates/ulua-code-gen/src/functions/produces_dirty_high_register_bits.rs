use crate::enums::ir_cmd::IrCmd;

#[inline]
pub fn produces_dirty_high_register_bits(cmd: IrCmd) -> bool {
  cmd == IrCmd::NumToUint || cmd == IrCmd::InvokeFastcall || cmd == IrCmd::CmpAny
}
