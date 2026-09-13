use crate::{
  records::{block::Block, symbol::Symbol},
  type_aliases::definition::Definition,
};

pub fn block_set_reaching_definition(block: &mut Block, sym: Symbol, def: *mut Definition) {
  *block.reaching_definitions.get_or_insert(sym) = def;
}
