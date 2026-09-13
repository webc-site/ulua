use core::mem::size_of;

use crate::{
  enums::{
    include_ir_prefix::IncludeIrPrefix, ir_block_kind::IrBlockKind, ir_cmd::IrCmd,
    ir_op_kind::IrOpKind,
  },
  functions::{
    any_argument_match::any_argument_match, get_next_block::get_next_block, is_pseudo::is_pseudo,
    jit_rng_random::jit_rng_random,
    to_string_detailed_ir_dump::to_string_detailed as to_string_detailed_inst,
    to_string_detailed_ir_dump_alt_b::to_string_detailed as to_string_detailed_block,
    to_string_ir_dump_alt_f::to_string_string_bytecode_types_c_char as to_string_bytecode_types,
  },
  records::{
    assembly_builder_a_64::AssemblyBuilderA64,
    assembly_builder_x_64::AssemblyBuilderX64,
    assembly_options::AssemblyOptions,
    bytecode_types::LBC_TYPE_ANY,
    ir_block::{IrBlock, K_BLOCK_NO_START_PC},
    ir_function::IrFunction,
    ir_lowering_a_64::IrLoweringA64,
    ir_lowering_x_64::IrLoweringX64,
    ir_op::IrOp,
    ir_to_string_context::IrToStringContext,
    label::Label,
  },
};

const K_BLOCK_FLAG_SAFE_ENV_CHECK: u8 = 1 << 0;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lower_impl_x_64(
  build: &mut AssemblyBuilderX64,
  lowering: &mut IrLoweringX64,
  function: &mut IrFunction,
  sorted_blocks: &[u32],
  bytecodeid: i32,
  options: &AssemblyOptions,
) -> bool {
  unsafe {
    let mut bc_locations = vec![u32::MAX; function.instructions.len() + 1];

    for (i, item) in function
      .bc_mapping
      .iter()
      .enumerate()
      .take(function.bc_mapping.len())
    {
      let ir_location = item.ir_location;

      if ir_location != u32::MAX {
        bc_locations[ir_location as usize] = i as u32;
      }
    }

    let output_enabled = options.include_assembly || options.include_ir;

    let mut text_size = build.text.len();
    let mut code_size = build.get_code_size();
    let mut seen_fallback = false;

    let mut dummy = IrBlock {
      start: u32::MAX,
      ..IrBlock::default()
    };

    debug_assert!(sorted_blocks[0] == 0);
    debug_assert!(function.entry_block == 0);

    for (i, &block_index) in sorted_blocks.iter().enumerate() {
      let block_ptr = function.blocks.as_mut_ptr().add(block_index as usize);

      if (*block_ptr).kind == IrBlockKind::Dead {
        continue;
      }

      debug_assert!((*block_ptr).start != u32::MAX);
      debug_assert!((*block_ptr).finish != u32::MAX);
      debug_assert!(
        !seen_fallback
          || (*block_ptr).kind == IrBlockKind::Fallback
          || (*block_ptr).kind == IrBlockKind::ExitSync
      );

      if ((*block_ptr).kind == IrBlockKind::Fallback || (*block_ptr).kind == IrBlockKind::ExitSync)
        && !seen_fallback
      {
        text_size = build.text.len();
        code_size = build.get_code_size();
        seen_fallback = true;
      }

      if options.include_ir {
        if options.include_ir_prefix == IncludeIrPrefix::Yes {
          build.log_append(format_args!("# "));
        }

        let mut ctx = IrToStringContext {
          result: &mut build.text,
          blocks: &function.blocks,
          constants: &function.constants,
          cfg: &function.cfg,
          vm_exit_info: &function.vm_exit_info,
          proto: function.proto.cast(),
        };
        to_string_detailed_block(
          &mut ctx,
          &*block_ptr,
          block_index,
          options.include_use_info,
          options.include_cfg_info,
          options.include_reg_flow_info,
        );
      }

      function.valid_restore_op_blocks.push(block_index);

      build.set_label_label(&mut (*block_ptr).label);

      if block_index == function.entry_block {
        function.entry_location = build.get_label_offset(&(*block_ptr).label);
      }

      lowering.start_block(&*block_ptr);

      let next_block_ptr = {
        let next_block = get_next_block(function, sorted_blocks, &mut dummy, i);
        next_block as *mut IrBlock
      };

      if (*block_ptr).expected_next_block != u32::MAX {
        debug_assert!(
          function.get_block_index(&*next_block_ptr) == (*block_ptr).expected_next_block
        );
      }

      if ((*block_ptr).flags & K_BLOCK_FLAG_SAFE_ENV_CHECK) != 0 {
        if options.include_ir {
          if options.include_ir_prefix == IncludeIrPrefix::Yes {
            build.log_append(format_args!("# "));
          }

          build.log_append(format_args!(
            "  implicit CHECK_SAFE_ENV exit({})\n",
            (*block_ptr).startpc
          ));
        }

        debug_assert!((*block_ptr).startpc != K_BLOCK_NO_START_PC);
        lowering.check_safe_env(
          IrOp {
            kind_and_index: IrOpKind::VmExit as u32 | ((*block_ptr).startpc << IrOp::INDEX_SHIFT),
          },
          IrLoweringX64::K_INVALID_INST_IDX,
          &*next_block_ptr,
        );
      }

      for index in (*block_ptr).start..=(*block_ptr).finish {
        debug_assert!((index as usize) < function.instructions.len());

        let bc_location = bc_locations[index as usize];

        if output_enabled && options.annotator.is_some() && bc_location != u32::MAX {
          if let Some(annotator) = options.annotator {
            annotator(
              options.annotator_context,
              &mut build.text,
              bytecodeid,
              bc_location as i32,
            );
          }

          let bc_types = function.get_bytecode_types_at(bc_location as i32);

          if bc_types.result != LBC_TYPE_ANY
            || bc_types.a != LBC_TYPE_ANY
            || bc_types.b != LBC_TYPE_ANY
            || bc_types.c != LBC_TYPE_ANY
          {
            to_string_bytecode_types(
              &mut build.text,
              &bc_types,
              options.compilation_options.userdata_types,
            );

            build.log_append(format_args!("\n"));
          }
        }

        if bc_location != u32::MAX {
          let label = if index == (*block_ptr).start {
            (*block_ptr).label
          } else {
            let mut label = Label::default();
            build.set_label(&mut label);
            label
          };

          function.bc_mapping[bc_location as usize].asm_location = build.get_label_offset(&label);
        }

        let inst_ptr = function.instructions.as_mut_ptr().add(index as usize);

        if is_pseudo((*inst_ptr).cmd) {
          if let Some(hint) = function.find_store_location_hint(index) {
            lowering.regs.curr_inst_idx = index;
            lowering.value_tracker.process_store_location_hint(hint);
            lowering.regs.curr_inst_idx = IrLoweringX64::K_INVALID_INST_IDX;
          }

          debug_assert!((*inst_ptr).use_count == 0);
          continue;
        }

        debug_assert!((*inst_ptr).last_use == 0 || (*inst_ptr).use_count != 0);

        if options.include_ir {
          if options.include_ir_prefix == IncludeIrPrefix::Yes {
            build.log_append(format_args!("# "));
          }

          let mut ctx = IrToStringContext {
            result: &mut build.text,
            blocks: &function.blocks,
            constants: &function.constants,
            cfg: &function.cfg,
            vm_exit_info: &function.vm_exit_info,
            proto: function.proto.cast(),
          };
          to_string_detailed_inst(
            &mut ctx,
            &*block_ptr,
            block_index,
            &mut *inst_ptr,
            index,
            options.include_use_info,
          );
        }

        lowering.lower_inst(&mut *inst_ptr, index, &*next_block_ptr);

        if lowering.has_error() {
          for item in sorted_blocks.iter().skip(i + 1) {
            let abandoned_ptr = function.blocks.as_mut_ptr().add(*item as usize);

            build.set_label_label(&mut (*abandoned_ptr).label);
          }

          lowering.ir_lowering_x_64_finish_function();

          return false;
        }
      }

      lowering.finish_block(&*block_ptr, &*next_block_ptr);

      if function.jit_rng_state != 0 {
        let term_inst_ptr = function
          .instructions
          .as_ptr()
          .add((*block_ptr).finish as usize);
        let next_start = (*next_block_ptr).start;

        let block_falls_through = any_argument_match(&*term_inst_ptr, |op| {
          op.kind() == IrOpKind::Block && function.blocks[op.index() as usize].start == next_start
        });

        if !(block_falls_through
          && (*term_inst_ptr).cmd == IrCmd::JUMP
          && (*next_block_ptr).use_count == 1)
        {
          let max_nop_bytes = if block_falls_through { 4 } else { 8 };
          let nop_bytes = jit_rng_random(&mut function.jit_rng_state) % max_nop_bytes;

          if nop_bytes > 0 {
            build.nop(nop_bytes);
          }
        }
      }

      if options.include_ir && options.include_ir_prefix == IncludeIrPrefix::Yes {
        build.log_append(format_args!("#\n"));
      }

      if (*block_ptr).expected_next_block == u32::MAX {
        function.valid_restore_op_blocks.clear();
      }
    }

    if !seen_fallback {
      text_size = build.text.len();
      code_size = build.get_code_size();
    }

    lowering.ir_lowering_x_64_finish_function();

    if output_enabled && !options.include_outlined_code && text_size < build.text.len() {
      build.text.truncate(text_size);

      if options.include_assembly {
        build.log_append(format_args!(
          "; skipping {} bytes of outlined code\n",
          build
            .get_code_size()
            .wrapping_sub(code_size)
            .wrapping_mul(size_of::<u8>() as u32)
        ));
      }
    }

    true
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lower_impl_a_64(
  build: &mut AssemblyBuilderA64,
  lowering: &mut IrLoweringA64,
  function: &mut IrFunction,
  sorted_blocks: &[u32],
  bytecodeid: i32,
  options: &AssemblyOptions,
) -> bool {
  unsafe {
    let mut bc_locations = vec![u32::MAX; function.instructions.len() + 1];

    for (i, item) in function
      .bc_mapping
      .iter()
      .enumerate()
      .take(function.bc_mapping.len())
    {
      let ir_location = item.ir_location;

      if ir_location != u32::MAX {
        bc_locations[ir_location as usize] = i as u32;
      }
    }

    let output_enabled = options.include_assembly || options.include_ir;

    let mut text_size = build.text.len();
    let mut code_size = build.get_code_size();
    let mut seen_fallback = false;

    let mut dummy = IrBlock {
      start: u32::MAX,
      ..IrBlock::default()
    };

    debug_assert!(sorted_blocks[0] == 0);
    debug_assert!(function.entry_block == 0);

    for (i, &block_index) in sorted_blocks.iter().enumerate() {
      let block_ptr = function.blocks.as_mut_ptr().add(block_index as usize);

      if (*block_ptr).kind == IrBlockKind::Dead {
        continue;
      }

      debug_assert!((*block_ptr).start != u32::MAX);
      debug_assert!((*block_ptr).finish != u32::MAX);
      debug_assert!(
        !seen_fallback
          || (*block_ptr).kind == IrBlockKind::Fallback
          || (*block_ptr).kind == IrBlockKind::ExitSync
      );

      if ((*block_ptr).kind == IrBlockKind::Fallback || (*block_ptr).kind == IrBlockKind::ExitSync)
        && !seen_fallback
      {
        text_size = build.text.len();
        code_size = build.get_code_size();
        seen_fallback = true;
      }

      if options.include_ir {
        if options.include_ir_prefix == IncludeIrPrefix::Yes {
          build.log_append(format_args!("# "));
        }

        let mut ctx = IrToStringContext {
          result: &mut build.text,
          blocks: &function.blocks,
          constants: &function.constants,
          cfg: &function.cfg,
          vm_exit_info: &function.vm_exit_info,
          proto: function.proto.cast(),
        };
        to_string_detailed_block(
          &mut ctx,
          &*block_ptr,
          block_index,
          options.include_use_info,
          options.include_cfg_info,
          options.include_reg_flow_info,
        );
      }

      function.valid_restore_op_blocks.push(block_index);

      build.set_label_label(&mut (*block_ptr).label);

      if block_index == function.entry_block {
        function.entry_location = build.get_label_offset(&(*block_ptr).label);
      }

      lowering.ir_lowering_a_64_start_block(&*block_ptr);

      let next_block_ptr = {
        let next_block = get_next_block(function, sorted_blocks, &mut dummy, i);
        next_block as *mut IrBlock
      };

      if (*block_ptr).expected_next_block != u32::MAX {
        debug_assert!(
          function.get_block_index(&*next_block_ptr) == (*block_ptr).expected_next_block
        );
      }

      if ((*block_ptr).flags & K_BLOCK_FLAG_SAFE_ENV_CHECK) != 0 {
        if options.include_ir {
          if options.include_ir_prefix == IncludeIrPrefix::Yes {
            build.log_append(format_args!("# "));
          }

          build.log_append(format_args!(
            "  implicit CHECK_SAFE_ENV exit({})\n",
            (*block_ptr).startpc
          ));
        }

        debug_assert!((*block_ptr).startpc != K_BLOCK_NO_START_PC);
        lowering.ir_lowering_a_64_check_safe_env(
          IrOp {
            kind_and_index: IrOpKind::VmExit as u32 | ((*block_ptr).startpc << IrOp::INDEX_SHIFT),
          },
          IrLoweringA64::K_INVALID_INST_IDX,
          &*next_block_ptr,
        );
      }

      for index in (*block_ptr).start..=(*block_ptr).finish {
        debug_assert!((index as usize) < function.instructions.len());

        let bc_location = bc_locations[index as usize];

        if output_enabled && options.annotator.is_some() && bc_location != u32::MAX {
          if let Some(annotator) = options.annotator {
            annotator(
              options.annotator_context,
              &mut build.text,
              bytecodeid,
              bc_location as i32,
            );
          }

          let bc_types = function.get_bytecode_types_at(bc_location as i32);

          if bc_types.result != LBC_TYPE_ANY
            || bc_types.a != LBC_TYPE_ANY
            || bc_types.b != LBC_TYPE_ANY
            || bc_types.c != LBC_TYPE_ANY
          {
            to_string_bytecode_types(
              &mut build.text,
              &bc_types,
              options.compilation_options.userdata_types,
            );

            build.log_append(format_args!("\n"));
          }
        }

        if bc_location != u32::MAX {
          let label = if index == (*block_ptr).start {
            (*block_ptr).label
          } else {
            build.set_label()
          };

          function.bc_mapping[bc_location as usize].asm_location = build.get_label_offset(&label);
        }

        let inst_ptr = function.instructions.as_mut_ptr().add(index as usize);

        if is_pseudo((*inst_ptr).cmd) {
          if let Some(hint) = function.find_store_location_hint(index) {
            lowering.regs.curr_inst_idx = index;
            lowering.value_tracker.process_store_location_hint(hint);
            lowering.regs.curr_inst_idx = IrLoweringA64::K_INVALID_INST_IDX;
          }

          debug_assert!((*inst_ptr).use_count == 0);
          continue;
        }

        debug_assert!((*inst_ptr).last_use == 0 || (*inst_ptr).use_count != 0);

        if options.include_ir {
          if options.include_ir_prefix == IncludeIrPrefix::Yes {
            build.log_append(format_args!("# "));
          }

          let mut ctx = IrToStringContext {
            result: &mut build.text,
            blocks: &function.blocks,
            constants: &function.constants,
            cfg: &function.cfg,
            vm_exit_info: &function.vm_exit_info,
            proto: function.proto.cast(),
          };
          to_string_detailed_inst(
            &mut ctx,
            &*block_ptr,
            block_index,
            &mut *inst_ptr,
            index,
            options.include_use_info,
          );
        }

        lowering.ir_lowering_a_64_lower_inst(&mut *inst_ptr, index, &*next_block_ptr);

        if lowering.ir_lowering_a_64_has_error() {
          for item in sorted_blocks.iter().skip(i + 1) {
            let abandoned_ptr = function.blocks.as_mut_ptr().add(*item as usize);

            build.set_label_label(&mut (*abandoned_ptr).label);
          }

          lowering.ir_lowering_a_64_finish_function();

          return false;
        }
      }

      lowering.ir_lowering_a_64_finish_block(&*block_ptr, &*next_block_ptr);

      if function.jit_rng_state != 0 {
        let term_inst_ptr = function
          .instructions
          .as_ptr()
          .add((*block_ptr).finish as usize);
        let next_start = (*next_block_ptr).start;

        let block_falls_through = any_argument_match(&*term_inst_ptr, |op| {
          op.kind() == IrOpKind::Block && function.blocks[op.index() as usize].start == next_start
        });

        if !(block_falls_through
          && (*term_inst_ptr).cmd == IrCmd::JUMP
          && (*next_block_ptr).use_count == 1)
        {
          let max_nop_bytes = if block_falls_through { 4 } else { 8 };
          let nop_bytes = jit_rng_random(&mut function.jit_rng_state) % max_nop_bytes;

          if nop_bytes > 0 {
            build.nop(nop_bytes);
          }
        }
      }

      if options.include_ir && options.include_ir_prefix == IncludeIrPrefix::Yes {
        build.log_append(format_args!("#\n"));
      }

      if (*block_ptr).expected_next_block == u32::MAX {
        function.valid_restore_op_blocks.clear();
      }
    }

    if !seen_fallback {
      text_size = build.text.len();
      code_size = build.get_code_size();
    }

    lowering.ir_lowering_a_64_finish_function();

    if output_enabled && !options.include_outlined_code && text_size < build.text.len() {
      build.text.truncate(text_size);

      if options.include_assembly {
        build.log_append(format_args!(
          "; skipping {} bytes of outlined code\n",
          build
            .get_code_size()
            .wrapping_sub(code_size)
            .wrapping_mul(size_of::<u32>() as u32)
        ));
      }
    }

    true
  }
}
