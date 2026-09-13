use crate::enums::ir_cmd::IrCmd;

#[inline]
pub fn is_pseudo(cmd: IrCmd) -> bool {
  // Instructions that are used for internal needs and are not a part of final lowering
  matches!(
    cmd,
    IrCmd::NOP | IrCmd::SUBSTITUTE | IrCmd::MarkUsed | IrCmd::MarkDead
  )
}
