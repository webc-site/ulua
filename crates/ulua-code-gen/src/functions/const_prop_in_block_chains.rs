use crate::{
  enums::ir_block_kind::IrBlockKind,
  functions::const_prop_in_block_chain::const_prop_in_block_chain,
  records::{const_prop_state::ConstPropState, ir_builder::IrBuilder, ir_function::IrFunction},
};

pub fn const_prop_in_block_chains(build: &mut IrBuilder) {
  let function: *mut IrFunction = &mut build.function;
  let mut state =
    unsafe { ConstPropState::const_prop_state_const_prop_state(build, &mut *function) };
  let mut visited = unsafe { vec![0u8; (*function).blocks.len()] };

  unsafe {
    (*function)
      .block_exit_tags
      .resize((*function).blocks.len(), Vec::new());

    for (i, block) in (*function).blocks.iter_mut().enumerate() {
      if block.kind == IrBlockKind::Fallback || block.kind == IrBlockKind::Dead {
        continue;
      }

      if visited[i] != 0 {
        continue;
      }

      const_prop_in_block_chain(build, &mut visited, block, &mut state);
    }
  }
}
