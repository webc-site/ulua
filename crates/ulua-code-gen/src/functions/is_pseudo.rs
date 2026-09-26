use crate::enums::ir_cmd::IrCmd;

#[inline]
pub fn is_pseudo(cmd: IrCmd) -> bool {
  // 仅供内部使用、不参与最终 lowering 的指令
  matches!(
    cmd,
    IrCmd::NOP | IrCmd::SUBSTITUTE | IrCmd::MarkUsed | IrCmd::MarkDead
  )
}
