use crate::records::{block::Block, sym_def::SymDef, symbol::Symbol};

pub fn block_set_reaching_definition(block: &mut Block, sym: Symbol, def: *mut SymDef) {
  *block.reaching_definitions.get_or_insert(sym) = def;
}
