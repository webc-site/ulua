use crate::{
  enums::ir_block_kind::IrBlockKind,
  functions::{
    apply_substitutions_ir_utils_alt_b::apply_substitutions_ir_function_ir_inst,
    const_prop_in_inst::const_prop_in_inst, fold_constants::fold_constants,
  },
  records::{
    const_prop_state::ConstPropState, ir_block::IrBlock, ir_builder::IrBuilder,
    ir_function::IrFunction,
  },
};

const K_BLOCK_FLAG_SAFE_ENV_CHECK: u8 = 1 << 0;

pub fn const_prop_in_block(build: &mut IrBuilder, block: &mut IrBlock, state: &mut ConstPropState) {
  let function: *mut IrFunction = &mut build.function;

  if (block.flags & K_BLOCK_FLAG_SAFE_ENV_CHECK) != 0 {
    state.in_safe_env = true;
  }

  for index in block.start..=block.finish {
    unsafe {
      let inst = &mut (&mut (*function).instructions)[index as usize] as *mut _;

      apply_substitutions_ir_function_ir_inst(&mut *function, &mut *inst);
      fold_constants(build, &mut *function, block, index);
      const_prop_in_inst(state, build, &mut *function, block, &mut *inst, index);
    }

    if block.kind == IrBlockKind::Dead {
      break;
    }
  }
}
