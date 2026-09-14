use alloc::vec::Vec;

use ulua_common::{FFlag::LuauCodegenLinearSetupEntryState3, FInt::LuauCodeGenMinLinearBlockPath};

use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{
    collect_direct_block_jump_path::collect_direct_block_jump_path,
    const_prop_in_block::const_prop_in_block, replace_ir_utils::replace_ir_function_ir_op_ir_op,
    setup_block_entry_state_optimize_const_prop::setup_block_entry_state_ir_builder_ir_function_ir_block_const_prop_state,
  },
  macros::op_a::op_a,
  records::{
    const_prop_state::ConstPropState, ir_block::IrBlock, ir_builder::IrBuilder,
    ir_function::IrFunction,
  },
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn try_create_linear_block(
  build: &mut IrBuilder,
  visited: &mut Vec<u8>,
  starting_block: *mut IrBlock,
  state: &mut ConstPropState,
) {
  let function: *mut IrFunction = &mut build.function;

  unsafe {
    let block_idx = (&*function).get_block_index(&*starting_block);
    crate::macros::codegen_assert::CODEGEN_ASSERT!(visited[block_idx as usize] == 0);
    visited[block_idx as usize] = 1;

    let term_inst_idx = (*starting_block).finish;
    let term_inst = (&mut (*function).instructions)[term_inst_idx as usize].clone();

    if term_inst.cmd != IrCmd::JUMP {
      return;
    }

    let target_op = op_a(&mut term_inst.clone());
    if target_op.kind() != IrOpKind::Block {
      return;
    }

    if (&(*function).blocks)[target_op.index() as usize].use_count == 1 {
      return;
    }

    let target_block_idx = target_op.index();
    let path = collect_direct_block_jump_path(&mut *function, visited, starting_block);

    if (path.len() as i32) < LuauCodeGenMinLinearBlockPath.get() {
      return;
    }

    state.clear();

    if LuauCodegenLinearSetupEntryState3.get() {
      setup_block_entry_state_ir_builder_ir_function_ir_block_const_prop_state(
        build,
        &mut *function,
        &*starting_block,
        state,
      );
    }

    const_prop_in_block(build, &mut *starting_block, state);

    if LuauCodegenLinearSetupEntryState3.get() {
      let current_target = {
        let inst = &mut (&mut (*function).instructions)[term_inst_idx as usize].clone();
        op_a(inst).index()
      };

      if (*starting_block).finish != term_inst_idx || current_target != target_block_idx {
        return;
      }

      if (&(*function).blocks)[target_block_idx as usize].use_count == 1 {
        return;
      }
    } else {
      let current_target = {
        let inst = &mut (&mut (*function).instructions)[(*starting_block).finish as usize].clone();
        op_a(inst).index()
      };

      if current_target != target_block_idx {
        crate::macros::codegen_assert::CODEGEN_ASSERT!(false);
        return;
      }
    }

    let starting_sort_key = (*starting_block).sortkey;
    let starting_chain_key = (*starting_block).chainkey;

    let new_block = build.block(IrBlockKind::Linearized);
    visited.push(0);

    build.begin_block(new_block);

    build.function.blocks[new_block.index() as usize].sortkey = starting_sort_key;
    build.function.blocks[new_block.index() as usize].chainkey = starting_chain_key + 1;
    build.function.blocks[block_idx as usize].expected_next_block = new_block.index();

    let op_ptr = {
      let inst = &mut build.function.instructions[term_inst_idx as usize];
      &mut inst.ops.as_mut_slice()[0] as *mut _
    };
    replace_ir_function_ir_op_ir_op(&mut build.function, &mut *op_ptr, new_block);

    build.clone(path.clone(), true);

    if build.function.cfg.r#in.len() == new_block.index() as usize {
      crate::macros::codegen_assert::CODEGEN_ASSERT!(
        build.function.cfg.r#in.len() == build.function.cfg.out.len()
      );
      crate::macros::codegen_assert::CODEGEN_ASSERT!(
        build.function.cfg.r#in.len() == build.function.cfg.def.len()
      );

      build
        .function
        .cfg
        .r#in
        .push(build.function.cfg.r#in[path[0] as usize]);
      build
        .function
        .cfg
        .out
        .push(build.function.cfg.out[*path.last().unwrap() as usize]);
      build.function.cfg.def.push(Default::default());

      let out_vararg_seq = build.function.cfg.out.last().unwrap().vararg_seq;
      let def_idx = build.function.cfg.def.len() - 1;

      for &path_block_idx in &path {
        let path_def = build.function.cfg.def[path_block_idx as usize];
        for i in 0..build.function.cfg.def[def_idx].regs.len() {
          build.function.cfg.def[def_idx].regs[i] |= path_def.regs[i];
        }

        if path_def.vararg_seq && out_vararg_seq {
          build.function.cfg.def[def_idx].vararg_seq = true;
          build.function.cfg.def[def_idx].vararg_start = path_def.vararg_start;
        }
      }

      build
        .function
        .cfg
        .predecessors_offsets
        .push(build.function.cfg.predecessors.len() as u32);
      build.function.cfg.predecessors.push(block_idx);
    }

    let linear_block = &mut build.function.blocks[new_block.index() as usize] as *mut IrBlock;
    const_prop_in_block(build, &mut *linear_block, state);
  }
}
