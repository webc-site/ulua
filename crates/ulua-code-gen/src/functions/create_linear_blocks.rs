use crate::{
  enums::ir_block_kind::IrBlockKind,
  functions::try_create_linear_block::try_create_linear_block,
  records::{
    const_prop_state::ConstPropState, ir_block::IrBlock, ir_builder::IrBuilder,
    ir_function::IrFunction,
  },
};

pub fn create_linear_blocks(build: &mut IrBuilder) {
  let function: *mut IrFunction = &mut build.function;
  let mut state =
    unsafe { ConstPropState::const_prop_state_const_prop_state(build, &mut *function) };
  let mut visited = unsafe { vec![0u8; (*function).blocks.len()] };

  let original_block_count = unsafe { (*function).blocks.len() };

  for i in 0..original_block_count {
    let block: *mut IrBlock = unsafe { &mut (&mut (*function).blocks)[i] };

    unsafe {
      if (*block).kind == IrBlockKind::Fallback || (*block).kind == IrBlockKind::Dead {
        continue;
      }
    }

    if visited[i] != 0 {
      continue;
    }

    unsafe { try_create_linear_block(build, &mut visited, block, &mut state) };
  }
}
