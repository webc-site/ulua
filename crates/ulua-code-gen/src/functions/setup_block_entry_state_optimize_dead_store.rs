use crate::{
  functions::propagate_tags_from_predecessors::propagate_tags_from_predecessors,
  records::{
    ir_block::IrBlock, ir_function::IrFunction, remove_dead_store_state::RemoveDeadStoreState,
  },
};

pub fn setup_block_entry_state_ir_function_ir_block_remove_dead_store_state(
  function: &IrFunction,
  block: &IrBlock,
  state: &mut RemoveDeadStoreState,
) {
  propagate_tags_from_predecessors(function, block, state);
}
