use crate::enums::ir_cmd::IrCmd;

#[inline]
pub fn is_non_terminating_jump(cmd: IrCmd) -> bool {
  matches!(
    cmd,
    IrCmd::TryNumToIndex
      | IrCmd::TryCallFastgettm
      | IrCmd::CheckFastcallRes
      | IrCmd::CheckTag
      | IrCmd::CheckTruthy
      | IrCmd::CheckReadonly
      | IrCmd::CheckNoMetatable
      | IrCmd::CheckSafeEnv
      | IrCmd::CheckArraySize
      | IrCmd::CheckSlotMatch
      | IrCmd::CheckNodeNoNext
      | IrCmd::CheckNodeValue
      | IrCmd::CheckBufferLen
      | IrCmd::CheckUserdataTag
      | IrCmd::CheckCmpNum
      | IrCmd::CheckCmpInt
      | IrCmd::CheckCmpInt64
      | IrCmd::CheckDivInt64
  )
}
