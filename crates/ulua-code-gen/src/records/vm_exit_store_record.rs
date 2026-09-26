use crate::records::ir_inst::IrInst;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct VmExitStoreRecord {
  pub inst_idx: u32,
  pub backup: IrInst,
}

impl Default for VmExitStoreRecord {
  fn default() -> Self {
    Self {
      inst_idx: 0xffffffff,
      // IrInst 含 SmallVector 与 noreg 寄存器，zeroed 会造出非法堆状态；
      // 安全 Default 即 cpp 默认（NOP、空操作数、noreg）
      backup: IrInst::default(),
    }
  }
}
