use crate::enums::ir_block_kind::IrBlockKind;

pub fn get_block_kind_priority(kind: IrBlockKind) -> i32 {
  if kind == IrBlockKind::Fallback {
    return 1;
  }

  if kind == IrBlockKind::ExitSync {
    return 2;
  }

  0
}
