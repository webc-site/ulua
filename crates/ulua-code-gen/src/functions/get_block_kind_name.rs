use crate::enums::ir_block_kind::IrBlockKind;

pub fn get_block_kind_name(kind: IrBlockKind) -> &'static str {
  match kind {
    IrBlockKind::Bytecode => "bb_bytecode",
    IrBlockKind::Fallback => "bb_fallback",
    IrBlockKind::Internal => "bb",
    IrBlockKind::Linearized => "bb_linear",
    IrBlockKind::ExitSync => "bb_exit",
    IrBlockKind::Dead => "dead",
  }
}
