use crate::{
  functions::mark_dead_stores_in_inst::mark_dead_stores_in_inst,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    ir_block::IrBlock, ir_builder::IrBuilder, ir_function::IrFunction, ir_inst::IrInst,
    remove_dead_store_state::RemoveDeadStoreState,
  },
};

// IrData.h: `inline constexpr uint8_t kBlockFlagSafeEnvCheck = 1 << 0;`
const K_BLOCK_FLAG_SAFE_ENV_CHECK: u8 = 1 << 0;

pub fn mark_dead_stores_in_block(
  build: &mut IrBuilder,
  block: &mut IrBlock,
  state: &mut RemoveDeadStoreState,
) {
  let function: *mut IrFunction = &mut build.function;

  // Block might establish a safe environment right at the start and might take a VM exit
  if (block.flags & K_BLOCK_FLAG_SAFE_ENV_CHECK) != 0 {
    state.read_all_regs();
  }

  let start = block.start;
  let finish = block.finish;

  let mut index = start;
  while index <= finish {
    CODEGEN_ASSERT!((index as usize) < unsafe { (*function).instructions.len() });

    let inst_ptr: *mut IrInst = unsafe { &mut (&mut (*function).instructions)[index as usize] };

    mark_dead_stores_in_inst(
      state,
      build,
      unsafe { &mut *function },
      block,
      unsafe { &mut *inst_ptr },
      index,
    );

    index += 1;
  }
}
