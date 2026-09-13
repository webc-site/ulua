use alloc::boxed::Box;

use crate::{
  functions::propagate_tags_from_predecessors::propagate_tags_from_predecessors,
  records::{
    ir_block::IrBlock, ir_function::IrFunction, remove_dead_store_state::RemoveDeadStoreState,
    store_reg_info::StoreRegInfo,
  },
};

pub fn setup_block_entry_state_ir_function_ir_block_remove_dead_store_state(
  function: &IrFunction,
  block: &IrBlock,
  state: &mut RemoveDeadStoreState,
) {
  let info_ptr: *mut [StoreRegInfo; 256] = &mut state.info;

  let get: Box<dyn Fn(usize) -> u8> = {
    let p = info_ptr;
    Box::new(move |i: usize| -> u8 { unsafe { (*p)[i].known_tag } })
  };

  let set: Box<dyn Fn(usize, u8)> = {
    let p = info_ptr;
    Box::new(move |i: usize, tag: u8| unsafe {
      (*p)[i].known_tag = tag;
    })
  };

  propagate_tags_from_predecessors(function, block, get, set);
}
