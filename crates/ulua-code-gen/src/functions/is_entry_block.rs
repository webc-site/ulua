use crate::{enums::ir_block_kind::IrBlockKind, records::ir_block::IrBlock};

pub fn is_entry_block(block: &IrBlock) -> bool {
  block.use_count == 0 && block.kind != IrBlockKind::Dead
}
