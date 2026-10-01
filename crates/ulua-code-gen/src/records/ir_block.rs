use crate::{enums::ir_block_kind::IrBlockKind, records::label::Label};

pub const K_BLOCK_NO_START_PC: u32 = u32::MAX;

/// `IrBlock::flags` 位掩码，对应 cpp/CodeGen/include/Luau/IrData.h 的
/// `kBlockFlagSafeEnvCheck` / `kBlockFlagSafeEnvClear` / `kBlockFlagEntryArgCheck`。
pub(crate) const K_BLOCK_FLAG_SAFE_ENV_CHECK: u8 = 1 << 0;
pub(crate) const K_BLOCK_FLAG_SAFE_ENV_CLEAR: u8 = 1 << 1;
pub(crate) const K_BLOCK_FLAG_ENTRY_ARG_CHECK: u8 = 1 << 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct IrBlock {
  pub kind: IrBlockKind,
  pub flags: u8,
  pub use_count: u16,

  /// 'start' 与 'finish' 定义函数内属于本 block 的指令闭区间
  /// block 构建完成后，'finish' 总指向唯一的那条终止指令
  pub start: u32,
  pub finish: u32,

  pub sortkey: u32,
  pub chainkey: u32,
  pub expected_next_block: u32,

  /// 生成本 block 时对应的字节码 PC 位置
  pub startpc: u32,

  pub label: Label,
}

impl Default for IrBlock {
  fn default() -> Self {
    Self {
      kind: IrBlockKind::Dead,
      flags: 0,
      use_count: 0,
      start: u32::MAX,
      finish: u32::MAX,
      sortkey: u32::MAX,
      chainkey: 0,
      expected_next_block: u32::MAX,
      startpc: K_BLOCK_NO_START_PC,
      label: Label {
        id: 0,
        location: u32::MAX,
      },
    }
  }
}
