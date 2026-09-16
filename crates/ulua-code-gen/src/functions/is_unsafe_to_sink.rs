use crate::enums::ir_cmd::IrCmd;

#[inline]
pub fn is_unsafe_to_sink(cmd: IrCmd) -> bool {
  matches!(
    cmd,
    IrCmd::LoadTag
      | IrCmd::LoadPointer
      | IrCmd::LoadDouble
      | IrCmd::LoadInt
      | IrCmd::LoadInt64
      | IrCmd::LoadFloat
      | IrCmd::LoadTvalue
      | IrCmd::BufferReadi8
      | IrCmd::BufferReadu8
      | IrCmd::BufferReadi16
      | IrCmd::BufferReadu16
      | IrCmd::BufferReadi32
      | IrCmd::BufferReadi64
      | IrCmd::BufferReadf32
      | IrCmd::BufferReadf64
      | IrCmd::GetUpvalue
      | IrCmd::TableLen
      | IrCmd::GetTypeof
      | IrCmd::TableSetnum
      | IrCmd::CmpAny
      | IrCmd::TryNumToIndex
      | IrCmd::TryCallFastgettm
  )
}
