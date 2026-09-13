use crate::{enums::ir_block_kind::IrBlockKind, records::label::Label};

pub const K_BLOCK_NO_START_PC: u32 = !0u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct IrBlock {
  pub kind: IrBlockKind,
  pub flags: u8,
  pub use_count: u16,

  /// 'start' and 'finish' define an inclusive range of instructions which belong to this block inside the function
  /// When block has been constructed, 'finish' always points to the first and only terminating instruction
  pub start: u32,
  pub finish: u32,

  pub sortkey: u32,
  pub chainkey: u32,
  pub expected_next_block: u32,

  /// Bytecode PC position at which the block was generated
  pub startpc: u32,

  pub label: Label,
}

impl Default for IrBlock {
  fn default() -> Self {
    Self {
      kind: IrBlockKind::Dead,
      flags: 0,
      use_count: 0,
      start: !0u32,
      finish: !0u32,
      sortkey: !0u32,
      chainkey: 0,
      expected_next_block: !0u32,
      startpc: K_BLOCK_NO_START_PC,
      label: Label {
        id: 0,
        location: !0u32,
      },
    }
  }
}
