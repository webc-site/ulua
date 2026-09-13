use crate::enums::ir_cmd::IrCmd;

#[inline]
pub fn is_block_terminator(cmd: IrCmd) -> bool {
  matches!(
    cmd,
    IrCmd::JUMP
      | IrCmd::JumpIfTruthy
      | IrCmd::JumpIfFalsy
      | IrCmd::JumpEqTag
      | IrCmd::JumpCmpInt
      | IrCmd::JumpEqPointer
      | IrCmd::JumpCmpNum
      | IrCmd::JumpCmpFloat
      | IrCmd::JumpFornLoopCond
      | IrCmd::JumpSlotMatch
      | IrCmd::RETURN
      | IrCmd::FORGLOOP
      | IrCmd::ForgloopFallback
      | IrCmd::ForgprepXnextFallback
      | IrCmd::FallbackForgprep
      | IrCmd::JumpCmpProtoid
  )
}
