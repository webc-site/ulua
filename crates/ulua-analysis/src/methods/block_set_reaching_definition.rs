use crate::{
  records::{block::Block, symbol::Symbol},
  type_aliases::def_id_control_flow_graph::DefId,
};

pub fn block_set_reaching_definition(block: &mut Block, sym: Symbol, def: DefId) {
  *block.reaching_definitions.get_or_insert(sym) = def;
}
