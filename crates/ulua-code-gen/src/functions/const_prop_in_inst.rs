use core::{ffi::CStr, mem::transmute};

use ulua_common::{
  FFlag::{DebugLuauAbortingChecks, LuauCodegenExtraTableOpts, LuauCodegenLoadPropagateOrigin},
  FInt::{LuauCodeGenLiveSlotReuseLimit, LuauCodeGenReuseSlotLimit},
  enums::luau_builtin_function::LuauBuiltinFunction,
};
use ulua_vm::{
  enums::lua_type::LuaType,
  macros::{getstr::getstr, tsvalue::tsvalue, ttisvector::ttisvector, vvalue::vvalue},
  type_aliases::t_value::TValue,
};

use crate::{
  enums::{ir_cmd::IrCmd, ir_const_kind::IrConstKind, ir_op_kind::IrOpKind},
  functions::{
    compare_ir_utils::compare_f64_f64_ir_condition,
    compare_ir_utils_alt_b::compare_i32_i32_ir_condition, condition_op::condition_op,
    fold_constants::fold_constants, handle_builtin_effects::handle_builtin_effects, is_gco::is_gco,
    kill_ir_utils::kill_ir_function_ir_inst,
    produces_dirty_high_register_bits::produces_dirty_high_register_bits,
    replace_ir_utils::replace_ir_function_ir_op_ir_op,
    replace_ir_utils_alt_b::replace_ir_function_ir_block_u32_ir_inst,
    safe_integer_constant::safe_integer_constant, substitute::substitute,
    substitute_with_truncated_uint::substitute_with_truncated_uint,
    try_get_operand_tag::try_get_operand_tag, try_get_tag_for_typename::try_get_tag_for_typename,
    vm_const_op::vm_const_op, vm_reg_op::vm_reg_op,
  },
  macros::{
    has_op_c::HAS_OP_C, op_a::op_a, op_b::op_b, op_c::op_c, op_d::op_d, op_e::op_e, op_f::op_f,
    op_g::op_g, opt_op_b::opt_op_b, opt_op_c::opt_op_c, opt_op_d::opt_op_d,
  },
  records::{
    array_value_entry::ArrayValueEntry, const_prop_state::ConstPropState, ir_block::IrBlock,
    ir_builder::IrBuilder, ir_data::K_INVALID_INST_IDX, ir_function::IrFunction, ir_inst::IrInst,
    ir_op::IrOp, node_slot_state::NodeSlotState, numbered_instruction::NumberedInstruction,
  },
  type_aliases::ir_ops::IrOps,
};

fn const_prop_make_inst(cmd: IrCmd, ops: &[IrOp]) -> IrInst {
  let mut ir_ops = IrOps::new();
  for &op in ops {
    ir_ops.push(op);
  }

  IrInst {
    cmd,
    ops: ir_ops,
    ..IrInst::default()
  }
}

fn tag_for_vm_const_typename(function: &IrFunction, source: IrOp, for_typeof: bool) -> Option<u8> {
  if source.kind() != IrOpKind::VmConst || function.proto.is_null() {
    return None;
  }

  unsafe {
    let constants = (*function.proto).k;
    if constants.is_null() {
      return None;
    }

    let value = constants.add(vm_const_op(source) as usize);
    if (*value).tt != LuaType::String as i32 {
      return None;
    }

    let string = tsvalue!(value as *const TValue);
    let name = CStr::from_ptr(getstr(string)).to_str().ok()?;
    let tag = try_get_tag_for_typename(name, for_typeof);

    if tag == 0xff { None } else { Some(tag) }
  }
}

fn type_name_tag_comparison(
  function: &IrFunction,
  lhs_op: IrOp,
  rhs_op: IrOp,
) -> Option<(IrOp, u8)> {
  if lhs_op.kind() != IrOpKind::Inst || rhs_op.kind() != IrOpKind::Inst {
    return None;
  }

  let mut lhs = function.instructions.get(lhs_op.index() as usize)?.clone();
  let mut rhs = function.instructions.get(rhs_op.index() as usize)?.clone();

  if lhs.cmd != IrCmd::GetType && lhs.cmd != IrCmd::GetTypeof {
    return None;
  }

  if rhs.cmd != IrCmd::LoadPointer {
    return None;
  }

  let source = op_a(&mut lhs);
  let tag = tag_for_vm_const_typename(function, op_a(&mut rhs), lhs.cmd == IrCmd::GetTypeof)?;

  Some((source, tag))
}

pub fn const_prop_in_inst(
  state: &mut ConstPropState,
  build: &mut IrBuilder,
  function: &mut IrFunction,
  block: &mut IrBlock,
  inst: &mut IrInst,
  index: u32,
) {
  match inst.cmd {
    IrCmd::LoadTag => {
      let source = op_a(inst);
      let tag = state.try_get_tag(source);
      if tag != 0xff {
        let tag_op = build.const_tag(tag);
        substitute(function, inst, tag_op);
      } else if source.kind() == IrOpKind::VmReg {
        if state.substitute_tag_load_with_t_value_data(build, inst) {
          return;
        }

        if LuauCodegenLoadPropagateOrigin.get() {
          state.try_redirect_vm_reg_load_to_t_value_origin(inst);
        }

        state.substitute_or_record_vm_reg_load(inst);
      }
    }
    IrCmd::LoadPointer => {
      let source = op_a(inst);
      if source.kind() == IrOpKind::VmReg {
        if state.substitute_or_record_value_load_with_t_value_data(build, inst) {
          return;
        }

        if LuauCodegenLoadPropagateOrigin.get() {
          state.try_redirect_vm_reg_load_to_t_value_origin(inst);
        }

        state.substitute_or_record_vm_reg_load(inst);
      }
    }
    IrCmd::LoadDouble => {
      let source = op_a(inst);
      let value = state.try_get_value(source);

      if function.as_double_op(value).is_some() {
        substitute(function, inst, value);
      } else if source.kind() == IrOpKind::VmReg {
        if state.substitute_or_record_value_load_with_t_value_data(build, inst) {
          return;
        }

        if LuauCodegenLoadPropagateOrigin.get() {
          state.try_redirect_vm_reg_load_to_t_value_origin(inst);
        }

        state.substitute_or_record_vm_reg_load(inst);
      } else {
        state.substitute_or_record(inst, index);
      }
    }
    IrCmd::LoadInt => {
      let source = op_a(inst);
      let value = state.try_get_value(source);

      if function.as_int_op(value).is_some() {
        substitute(function, inst, value);
      } else if source.kind() == IrOpKind::VmReg {
        if state.substitute_or_record_value_load_with_t_value_data(build, inst) {
          return;
        }

        if LuauCodegenLoadPropagateOrigin.get() {
          state.try_redirect_vm_reg_load_to_t_value_origin(inst);
        }

        state.substitute_or_record_vm_reg_load(inst);
      } else {
        state.substitute_or_record(inst, index);
      }
    }
    IrCmd::LoadInt64 => {
      let source = op_a(inst);
      let value = state.try_get_value(source);

      if function.as_int_64_op(value).is_some() {
        substitute(function, inst, value);
      } else if source.kind() == IrOpKind::VmReg {
        if state.substitute_or_record_value_load_with_t_value_data(build, inst) {
          return;
        }

        if LuauCodegenLoadPropagateOrigin.get() {
          state.try_redirect_vm_reg_load_to_t_value_origin(inst);
        }

        state.substitute_or_record_vm_reg_load(inst);
      } else {
        state.substitute_or_record(inst, index);
      }
    }
    IrCmd::LoadTvalue => {
      let source = op_a(inst);
      if source.kind() == IrOpKind::VmReg {
        if !state.substitute_or_record_vm_reg_load(inst) && !HAS_OP_C!(inst) {
          let tag = state.try_get_tag(source);
          if tag != 0xff {
            let offset = build.const_int(0);
            let tag_op = build.const_tag(tag);
            let mut ops = IrOps::new();
            ops.push(source);
            ops.push(offset);
            ops.push(tag_op);
            replace_ir_function_ir_block_u32_ir_inst(
              function,
              block,
              index,
              IrInst {
                cmd: IrCmd::LoadTvalue,
                ops,
                ..IrInst::default()
              },
            );
          }
        }
      } else if source.kind() == IrOpKind::Inst {
        let source_inst = function.instructions[source.index() as usize].clone();

        if source_inst.cmd == IrCmd::GetSlotNodeAddr {
          if let Some(prev_idx) = state.hash_value_cache.find(&source.index()).copied()
            && prev_idx != K_INVALID_INST_IDX
          {
            let prev = function.instructions[prev_idx as usize].clone();

            if prev.cmd == IrCmd::LoadTvalue {
              if prev.use_count != 0 {
                substitute(
                  function,
                  inst,
                  IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, prev_idx),
                );
              }
            } else if prev.cmd == IrCmd::StoreSplitTvalue {
              state
                .inst_tag
                .try_insert(index, function.tag_op(op_b(prev.clone())));
              state.inst_value.try_insert(index, op_c(prev));
            } else if LuauCodegenExtraTableOpts.get() && prev.cmd == IrCmd::StoreTvalue {
              let arg = function.as_inst_op(op_b(prev.clone()));
              if !arg.is_null() && unsafe { (*arg).use_count } != 0 {
                substitute(function, inst, op_b(prev));
              }
            }

            return;
          }

          *state.hash_value_cache.get_or_insert(source.index()) = index;
        } else if source_inst.cmd == IrCmd::GetArrAddr {
          let mut source_addr = source_inst;
          let offset_op =
            state.get_combined_array_load_offset_op(&mut source_addr, opt_op_b(inst.clone()));

          if let Some(entry) = state
            .array_value_cache
            .iter()
            .find(|entry| entry.pointer == source.index() && entry.offset == offset_op)
            .copied()
            && entry.value != K_INVALID_INST_IDX
          {
            let prev = function.instructions[entry.value as usize].clone();

            if prev.cmd == IrCmd::LoadTvalue {
              if prev.use_count != 0 {
                substitute(
                  function,
                  inst,
                  IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, entry.value),
                );
              }
            } else if prev.cmd == IrCmd::StoreSplitTvalue {
              state
                .inst_tag
                .try_insert(index, function.tag_op(op_b(prev.clone())));
              state.inst_value.try_insert(index, op_c(prev));
            } else if LuauCodegenExtraTableOpts.get() && prev.cmd == IrCmd::StoreTvalue {
              let arg = function.as_inst_op(op_b(prev.clone()));
              if !arg.is_null() && unsafe { (*arg).use_count } != 0 {
                substitute(function, inst, op_b(prev));
              }
            }

            return;
          }

          state.array_value_cache.push(ArrayValueEntry {
            pointer: source.index(),
            offset: offset_op,
            value: index,
          });
        } else {
          state.substitute_or_record(inst, index);
        }
      }
    }
    IrCmd::LoadFloat => {
      let source = op_a(inst);
      if source.kind() == IrOpKind::VmReg {
        let offset = function.int_op(op_b(inst.clone()));

        if let Some(subst) =
          state.find_substitute_component_load_from_store_vector(build, source, offset)
        {
          substitute(function, inst, subst);
          return;
        }

        if let Some(prev_idx_ptr) =
          state.get_previous_versioned_load_index(IrCmd::LoadTvalue, source)
        {
          let mut prev_idx = unsafe { *prev_idx_ptr };
          let prev = function.instructions[prev_idx as usize].clone();

          if prev.cmd == IrCmd::TagVector {
            let mut prev_for_arg = prev.clone();
            let prev_arg = op_a(&mut prev_for_arg);
            if !function.as_inst_op(prev_arg).is_null() {
              prev_idx = prev_arg.index();
            }
          }

          let value = function.instructions[prev_idx as usize].clone();
          let byte_offset = offset as u32;
          crate::macros::codegen_assert::CODEGEN_ASSERT!(byte_offset.is_multiple_of(4));

          let component = byte_offset / 4;
          crate::macros::codegen_assert::CODEGEN_ASSERT!(component <= 3);

          if value.cmd == IrCmd::LoadTvalue {
            let mut value_for_source = value.clone();
            let value_source = op_a(&mut value_for_source);

            if value_source.kind() == IrOpKind::VmConst && !function.proto.is_null() {
              let tv = unsafe { (*function.proto).k.add(vm_const_op(value_source) as usize) };

              if unsafe { ttisvector!(tv as *const TValue) } {
                let v = unsafe { vvalue!(tv as *const TValue) };
                let subst = build.const_double(v[component as usize] as f64);
                substitute(function, inst, subst);
                return;
              }
            } else if value_source.kind() == IrOpKind::VmReg {
              let prev_op = IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, prev_idx);
              if state.try_get_reg_link(prev_op).is_some()
                && let Some(subst) = state.find_substitute_component_load_from_store_vector(
                  build,
                  value_source,
                  offset,
                )
              {
                substitute(function, inst, subst);
                return;
              }
            }
          }

          let mut ops = IrOps::new();
          ops.push(IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, prev_idx));
          ops.push(build.const_int(component as i32));

          replace_ir_function_ir_block_u32_ir_inst(
            function,
            block,
            index,
            IrInst {
              cmd: IrCmd::ExtractVec,
              ops,
              ..IrInst::default()
            },
          );

          state.substitute_or_record(&mut function.instructions[index as usize], index);
          return;
        }

        state.substitute_or_record_vm_reg_load(inst);
      } else {
        state.substitute_or_record(inst, index);
      }
    }
    IrCmd::StoreTag => {
      let target = op_a(inst);
      if target.kind() == IrOpKind::VmReg {
        let mut active_load_cmd = IrCmd::NOP;
        let mut active_load_value = !0u32;
        let value = op_b(inst.clone());
        if value.kind() == IrOpKind::Constant {
          let tag = function.tag_op(value);
          (active_load_cmd, active_load_value) =
            state.get_previous_versioned_load_for_tag(tag, target);

          if state.try_get_tag(target) == tag {
            kill_ir_function_ir_inst(function, inst);
          } else {
            state.save_tag(target, tag);
            if tag == LuaType::Nil as u8 {
              state.invalidate_value(target);
            }
          }
        } else {
          state.invalidate_tag(target);
        }

        if active_load_value != !0u32 {
          let key = state.versioned_vm_reg_load_ir_cmd_ir_op(active_load_cmd, target);
          *state.value_map.get_or_insert(key) = active_load_value;
        }
      }
    }
    IrCmd::StorePointer => {
      let target = op_a(inst);
      if target.kind() == IrOpKind::VmReg {
        let value = op_b(inst.clone());
        if value.kind() == IrOpKind::Inst
          && let Some(prev_idx) =
            state.get_previous_versioned_load_index(IrCmd::LoadPointer, target)
          && unsafe { *prev_idx } == value.index()
        {
          kill_ir_function_ir_inst(function, inst);
          return;
        }

        state.invalidate_value(target);
        if value.kind() == IrOpKind::Inst {
          state.forward_vm_reg_store_to_load(inst, IrCmd::LoadPointer);

          let value_ptr = function.as_inst_op(value);
          if !LuauCodegenExtraTableOpts.get()
            && !value_ptr.is_null()
            && unsafe { (*value_ptr).cmd } == IrCmd::NewTable
            && let Some(info) = state.try_get_register_info(target)
          {
            unsafe {
              let array_size_op = (&(*value_ptr).ops)[0];
              (*info).known_not_readonly_deprecated = true;
              (*info).known_no_metatable_deprecated = true;
              (*info).known_table_array_size_deprecated = function.uint_op(array_size_op) as i32;
            }
          }
        }
      }
    }
    IrCmd::StoreDouble => {
      let target = op_a(inst);
      if target.kind() == IrOpKind::VmReg {
        let value = op_b(inst.clone());
        if value.kind() == IrOpKind::Constant {
          if state.try_get_value(target) == value {
            kill_ir_function_ir_inst(function, inst);
          } else {
            state.save_value(target, value);
          }
        } else {
          if value.kind() == IrOpKind::Inst
            && let Some(prev_idx) =
              state.get_previous_versioned_load_index(IrCmd::LoadDouble, target)
            && unsafe { *prev_idx } == value.index()
          {
            kill_ir_function_ir_inst(function, inst);
            return;
          }

          state.invalidate_value(target);
          state.forward_vm_reg_store_to_load(inst, IrCmd::LoadDouble);
        }
      }
    }
    IrCmd::StoreInt => {
      let stored = op_b(inst.clone());
      let stored_inst = function.as_inst_op(stored);
      if !stored_inst.is_null() && unsafe { (*stored_inst).cmd } == IrCmd::TruncateUint {
        let mut stored_inst_clone = unsafe { (*stored_inst).clone() };
        let replacement = op_a(&mut stored_inst_clone);
        replace_ir_function_ir_op_ir_op(function, &mut inst.ops[1], replacement);
      }

      let target = op_a(inst);
      if target.kind() == IrOpKind::VmReg {
        let value = op_b(inst.clone());
        if value.kind() == IrOpKind::Constant {
          if state.try_get_value(target) == value {
            kill_ir_function_ir_inst(function, inst);
          } else {
            state.save_value(target, value);
          }
        } else {
          if value.kind() == IrOpKind::Inst
            && let Some(prev_idx) = state.get_previous_versioned_load_index(IrCmd::LoadInt, target)
            && unsafe { *prev_idx } == value.index()
          {
            kill_ir_function_ir_inst(function, inst);
            return;
          }

          state.invalidate_value(target);
          state.forward_vm_reg_store_to_load(inst, IrCmd::LoadInt);
        }
      }
    }
    IrCmd::StoreInt64 => {
      let target = op_a(inst);
      if target.kind() == IrOpKind::VmReg {
        let value = op_b(inst.clone());
        if value.kind() == IrOpKind::Constant {
          if state.try_get_value(target) == value {
            kill_ir_function_ir_inst(function, inst);
          } else {
            state.save_value(target, value);
          }
        } else {
          if value.kind() == IrOpKind::Inst
            && let Some(prev_idx) =
              state.get_previous_versioned_load_index(IrCmd::LoadInt64, target)
            && unsafe { *prev_idx } == value.index()
          {
            kill_ir_function_ir_inst(function, inst);
            return;
          }

          state.invalidate_value(target);
          state.forward_vm_reg_store_to_load(inst, IrCmd::LoadInt64);
        }
      }
    }
    IrCmd::StoreVector => {
      let target = op_a(inst);
      if target.kind() == IrOpKind::VmReg {
        state.invalidate_value(target);

        let reg = vm_reg_op(target) as usize;
        let captured_regs = unsafe { &(*state.function).cfg.captured.regs };
        if (captured_regs[reg / 64] & (1u64 << (reg % 64))) == 0 {
          let key = state.versioned_vm_reg_load_ir_cmd_ir_op(IrCmd::LoadFloat, target);
          *state.value_map.get_or_insert(key) = index;
        }
      }
    }
    IrCmd::StoreTvalue => {
      let target = op_a(inst);
      if target.kind() == IrOpKind::VmReg || target.kind() == IrOpKind::Inst {
        let stored = op_b(inst.clone());

        if target.kind() == IrOpKind::VmReg {
          if stored.kind() == IrOpKind::Inst
            && let Some(prev_idx) =
              state.get_previous_versioned_load_index(IrCmd::LoadTvalue, target)
            && unsafe { *prev_idx } == stored.index()
          {
            kill_ir_function_ir_inst(function, inst);
            return;
          }

          state.invalidate_ir_op(target);
        }

        let mut tag = state.try_get_tag(stored);
        if tag == 0xff {
          tag = try_get_operand_tag(function, stored).unwrap_or(0xff);
        }

        let mut value = state.try_get_value(stored);

        if target.kind() == IrOpKind::VmReg {
          if tag != 0xff {
            state.save_tag(target, tag);
          }

          if value.kind() != IrOpKind::None {
            state.save_value(target, value);
          }
        }

        if target.kind() == IrOpKind::Inst {
          let target_inst = function.instructions[target.index() as usize].clone();
          state.invalidate_table_store_location(target_inst, opt_op_c(inst.clone()), tag);
        }

        let mut active_load_cmd = IrCmd::NOP;
        let mut active_load_value = !0u32;

        if tag != 0xff && value.kind() == IrOpKind::None && state.try_get_reg_link(stored).is_some()
        {
          let arg_ptr = function.as_inst_op(stored);
          if !arg_ptr.is_null() {
            let mut arg = unsafe { (*arg_ptr).clone() };
            let arg_source = op_a(&mut arg);
            if arg.cmd == IrCmd::LoadTvalue && arg_source.kind() == IrOpKind::VmReg {
              let (cmd, idx) = state.get_previous_versioned_load_for_tag(tag, arg_source);
              if idx != !0u32 {
                active_load_cmd = cmd;
                active_load_value = idx;
                value = IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, idx);
              }
            }
          }
        }

        let can_split_tvalue_store = if tag == LuaType::Boolean as u8 {
          value.kind() == IrOpKind::Inst || function.as_int_op(value).is_some()
        } else if tag == LuaType::Number as u8 {
          value.kind() == IrOpKind::Inst || function.as_double_op(value).is_some()
        } else if tag == LuaType::Integer as u8 {
          value.kind() == IrOpKind::Inst || function.as_int_64_op(value).is_some()
        } else {
          tag != 0xff && is_gco(tag) && value.kind() == IrOpKind::Inst
        };

        if can_split_tvalue_store {
          let tag_op = build.const_tag(tag);
          let mut ops = IrOps::new();
          ops.push(target);
          ops.push(tag_op);
          ops.push(value);
          if HAS_OP_C!(inst) {
            ops.push(op_c(inst.clone()));
          }

          let replacement = IrInst {
            cmd: IrCmd::StoreSplitTvalue,
            ops,
            ..IrInst::default()
          };
          let replacement_offset = opt_op_d(replacement.clone());

          replace_ir_function_ir_block_u32_ir_inst(function, block, index, replacement);

          if target.kind() == IrOpKind::VmReg && active_load_value != !0u32 {
            let reg = vm_reg_op(target) as usize;
            let versioned_reg = IrOp::ir_op_ir_op_kind_u32(
              IrOpKind::VmReg,
              (reg as u32) | (state.regs[reg].version << 8),
            );
            let mut ops = IrOps::new();
            ops.push(versioned_reg);
            let key = IrInst {
              cmd: active_load_cmd,
              ops,
              ..IrInst::default()
            };
            *state.value_map.get_or_insert(key) = active_load_value;
          }

          if target.kind() == IrOpKind::Inst {
            let target_ptr = &mut function.instructions[target.index() as usize] as *mut IrInst;
            unsafe {
              state.forward_table_store_to_load(&mut *target_ptr, replacement_offset, index);
            }
          }
        } else if target.kind() == IrOpKind::VmReg {
          state.forward_vm_reg_store_to_load(inst, IrCmd::LoadTvalue);
        } else if LuauCodegenExtraTableOpts.get() && target.kind() == IrOpKind::Inst {
          let offset = opt_op_c(inst.clone());
          let target_ptr = &mut function.instructions[target.index() as usize] as *mut IrInst;
          unsafe {
            state.forward_table_store_to_load(&mut *target_ptr, offset, index);
          }
        }
      }
    }
    IrCmd::StoreSplitTvalue => {
      let target = op_a(inst);
      if target.kind() == IrOpKind::VmReg {
        state.invalidate_ir_op(target);

        let tag = function.tag_op(op_b(inst.clone()));
        state.save_tag(target, tag);

        let value = op_c(inst.clone());
        if value.kind() == IrOpKind::Constant {
          state.save_value(target, value);
        }
      } else if target.kind() == IrOpKind::Inst {
        let tag = function.tag_op(op_b(inst.clone()));
        let offset = opt_op_d(inst.clone());
        let target_inst = function.instructions[target.index() as usize].clone();
        state.invalidate_table_store_location(target_inst, offset, tag);

        let target_ptr = &mut function.instructions[target.index() as usize] as *mut IrInst;
        unsafe {
          state.forward_table_store_to_load(&mut *target_ptr, offset, index);
        }
      }
    }
    IrCmd::GetUpvalue => {
      state.substitute_or_record_vm_upvalue_load(inst);
    }
    IrCmd::SetUpvalue => {
      state.forward_vm_upvalue_store_to_load(inst);

      let source = op_b(inst.clone());
      let tag = state.try_get_tag(source);
      if tag != 0xff {
        let tag_op = build.const_tag(tag);
        replace_ir_function_ir_op_ir_op(function, &mut inst.ops[2], tag_op);
      }
    }
    IrCmd::Int64ToNum | IrCmd::IntToNum => {
      state.substitute_or_record(inst, index);
    }
    IrCmd::AddNum | IrCmd::SubNum => {
      let rhs = op_b(inst.clone());
      let rhs = if rhs.kind() == IrOpKind::Constant {
        rhs
      } else {
        state.try_get_value(rhs)
      };

      if let Some(k) = function.as_double_op(rhs) {
        if k == 0.0 && k.is_sign_negative() == (inst.cmd == IrCmd::AddNum) {
          let lhs = op_a(inst);
          substitute(function, inst, lhs);
        } else {
          state.substitute_or_record(inst, index);
        }
      } else {
        state.substitute_or_record(inst, index);
      }
    }
    IrCmd::MulNum => {
      let rhs = op_b(inst.clone());
      let rhs = if rhs.kind() == IrOpKind::Constant {
        rhs
      } else {
        state.try_get_value(rhs)
      };

      if let Some(k) = function.as_double_op(rhs) {
        if k == 1.0 {
          let lhs = op_a(inst);
          substitute(function, inst, lhs);
        } else if k == 2.0 {
          let lhs = op_a(inst);
          let mut ops = IrOps::new();
          ops.push(lhs);
          ops.push(lhs);
          replace_ir_function_ir_block_u32_ir_inst(
            function,
            block,
            index,
            IrInst {
              cmd: IrCmd::AddNum,
              ops,
              ..IrInst::default()
            },
          );
        } else if k == -1.0 {
          let lhs = op_a(inst);
          let mut ops = IrOps::new();
          ops.push(lhs);
          replace_ir_function_ir_block_u32_ir_inst(
            function,
            block,
            index,
            IrInst {
              cmd: IrCmd::UnmNum,
              ops,
              ..IrInst::default()
            },
          );
        } else {
          state.substitute_or_record(inst, index);
        }
      } else {
        state.substitute_or_record(inst, index);
      }
    }
    IrCmd::DivNum => {
      let rhs = op_b(inst.clone());
      let rhs = if rhs.kind() == IrOpKind::Constant {
        rhs
      } else {
        state.try_get_value(rhs)
      };

      if let Some(k) = function.as_double_op(rhs) {
        if k == 1.0 {
          let lhs = op_a(inst);
          substitute(function, inst, lhs);
        } else if k == -1.0 {
          let lhs = op_a(inst);
          let mut ops = IrOps::new();
          ops.push(lhs);
          replace_ir_function_ir_block_u32_ir_inst(
            function,
            block,
            index,
            IrInst {
              cmd: IrCmd::UnmNum,
              ops,
              ..IrInst::default()
            },
          );
        } else {
          let exp = k.log2();
          if k > 0.0 && k.is_finite() && exp.fract() == 0.0 && (-1000.0..=1000.0).contains(&exp) {
            let lhs = op_a(inst);
            let reciprocal = build.const_double(1.0 / k);
            let mut ops = IrOps::new();
            ops.push(lhs);
            ops.push(reciprocal);
            replace_ir_function_ir_block_u32_ir_inst(
              function,
              block,
              index,
              IrInst {
                cmd: IrCmd::MulNum,
                ops,
                ..IrInst::default()
              },
            );
          } else {
            state.substitute_or_record(inst, index);
          }
        }
      } else {
        state.substitute_or_record(inst, index);
      }
    }
    IrCmd::AddFloat | IrCmd::SubFloat => {
      let rhs = op_b(inst.clone());
      let rhs = if rhs.kind() == IrOpKind::Constant {
        rhs
      } else {
        state.try_get_value(rhs)
      };

      if let Some(k) = function.as_double_op(rhs) {
        let kf = k as f32;
        if kf == 0.0 && kf.is_sign_negative() == (inst.cmd == IrCmd::AddFloat) {
          let lhs = op_a(inst);
          substitute(function, inst, lhs);
        } else {
          state.substitute_or_record(inst, index);
        }
      } else {
        state.substitute_or_record(inst, index);
      }
    }
    IrCmd::MulFloat => {
      let rhs = op_b(inst.clone());
      let rhs = if rhs.kind() == IrOpKind::Constant {
        rhs
      } else {
        state.try_get_value(rhs)
      };

      if let Some(k) = function.as_double_op(rhs) {
        let kf = k as f32;
        if kf == 1.0 {
          let lhs = op_a(inst);
          substitute(function, inst, lhs);
        } else if kf == 2.0 {
          let lhs = op_a(inst);
          let mut ops = IrOps::new();
          ops.push(lhs);
          ops.push(lhs);
          replace_ir_function_ir_block_u32_ir_inst(
            function,
            block,
            index,
            IrInst {
              cmd: IrCmd::AddFloat,
              ops,
              ..IrInst::default()
            },
          );
        } else if kf == -1.0 {
          let lhs = op_a(inst);
          let mut ops = IrOps::new();
          ops.push(lhs);
          replace_ir_function_ir_block_u32_ir_inst(
            function,
            block,
            index,
            IrInst {
              cmd: IrCmd::UnmFloat,
              ops,
              ..IrInst::default()
            },
          );
        } else {
          state.substitute_or_record(inst, index);
        }
      } else {
        state.substitute_or_record(inst, index);
      }
    }
    IrCmd::DivFloat => {
      let rhs = op_b(inst.clone());
      let rhs = if rhs.kind() == IrOpKind::Constant {
        rhs
      } else {
        state.try_get_value(rhs)
      };

      if let Some(k) = function.as_double_op(rhs) {
        let kf = k as f32;
        if kf == 1.0 {
          let lhs = op_a(inst);
          substitute(function, inst, lhs);
        } else if kf == -1.0 {
          let lhs = op_a(inst);
          let mut ops = IrOps::new();
          ops.push(lhs);
          replace_ir_function_ir_block_u32_ir_inst(
            function,
            block,
            index,
            IrInst {
              cmd: IrCmd::UnmFloat,
              ops,
              ..IrInst::default()
            },
          );
        } else {
          let exp = kf.log2();
          if kf > 0.0 && kf.is_finite() && exp.fract() == 0.0 && (-1000.0..=1000.0).contains(&exp) {
            let lhs = op_a(inst);
            let reciprocal = build.const_double((1.0f32 / kf) as f64);
            let mut ops = IrOps::new();
            ops.push(lhs);
            ops.push(reciprocal);
            replace_ir_function_ir_block_u32_ir_inst(
              function,
              block,
              index,
              IrInst {
                cmd: IrCmd::MulFloat,
                ops,
                ..IrInst::default()
              },
            );
          } else {
            state.substitute_or_record(inst, index);
          }
        }
      } else {
        state.substitute_or_record(inst, index);
      }
    }
    IrCmd::MinFloat
    | IrCmd::MaxFloat
    | IrCmd::UnmFloat
    | IrCmd::FloorFloat
    | IrCmd::CeilFloat
    | IrCmd::SqrtFloat
    | IrCmd::AbsFloat
    | IrCmd::SignFloat => {
      state.substitute_or_record(inst, index);
    }
    IrCmd::IdivNum
    | IrCmd::MuladdNum
    | IrCmd::ModNum
    | IrCmd::MinNum
    | IrCmd::MaxNum
    | IrCmd::UnmNum
    | IrCmd::FloorNum
    | IrCmd::CeilNum
    | IrCmd::RoundNum
    | IrCmd::SqrtNum
    | IrCmd::AbsNum
    | IrCmd::SignNum
    | IrCmd::SelectNum
    | IrCmd::SelectInt64
    | IrCmd::SelectVec
    | IrCmd::MuladdVec
    | IrCmd::ExtractVec
    | IrCmd::NotAny => {
      state.substitute_or_record(inst, index);
    }
    IrCmd::SelectIfTruthy => {
      let tag = state.try_get_tag(op_a(inst));

      if tag == LuaType::Nil as u8 {
        let replacement = op_c(inst.clone());
        substitute(function, inst, replacement);
      } else if tag != 0xff && tag != LuaType::Boolean as u8 {
        let replacement = op_b(inst.clone());
        substitute(function, inst, replacement);
      }
    }
    IrCmd::UintToNum | IrCmd::UintToFloat => {
      let src = function.as_inst_op(op_a(inst));
      if !src.is_null() {
        let mut src_clone = unsafe { (*src).clone() };
        if src_clone.cmd == IrCmd::TruncateUint {
          let src_source = op_a(&mut src_clone);
          let src_of_src = function.as_inst_op(src_source);
          if !src_of_src.is_null() && unsafe { (*src_of_src).cmd } == IrCmd::NumToUint {
            replace_ir_function_ir_op_ir_op(function, &mut inst.ops[0], src_source);
          }
        }
      }

      state.substitute_or_record(inst, index);
    }
    IrCmd::NumToInt => {
      let src = function.as_inst_op(op_a(inst));
      if !src.is_null() {
        let src_clone = unsafe { (*src).clone() };

        if src_clone.cmd == IrCmd::IntToNum {
          let mut src_clone = src_clone;
          substitute(function, inst, op_a(&mut src_clone));
          return;
        }

        if src_clone.cmd == IrCmd::AddNum {
          if let Some(arg) = function.as_double_op(op_b(src_clone.clone()))
            && arg == 0.0
          {
            let mut src_clone = src_clone.clone();
            replace_ir_function_ir_op_ir_op(function, &mut inst.ops[0], op_a(&mut src_clone));
            state.substitute_or_record(inst, index);
            return;
          }

          let mut src_clone_for_a = src_clone.clone();
          let src_op_a = op_a(&mut src_clone_for_a);
          if let Some(arg) = function.as_double_op(src_op_a)
            && arg == 0.0
          {
            replace_ir_function_ir_op_ir_op(function, &mut inst.ops[0], op_b(src_clone));
            state.substitute_or_record(inst, index);
            return;
          }
        }

        if src_clone.cmd == IrCmd::UintToNum {
          let mut src_clone = src_clone;
          let src_source = op_a(&mut src_clone);
          if src_source.kind() != IrOpKind::Constant {
            substitute_with_truncated_uint(function, block, inst, src_source);
            return;
          }
        }
      }

      state.substitute_or_record(inst, index);
    }
    IrCmd::NumToUint => {
      let src = function.as_inst_op(op_a(inst));
      if !src.is_null() {
        let src_clone = unsafe { (*src).clone() };

        if src_clone.cmd == IrCmd::UintToNum {
          let mut src_clone = src_clone;
          substitute_with_truncated_uint(function, block, inst, op_a(&mut src_clone));
          return;
        }

        if src_clone.cmd == IrCmd::IntToNum {
          let mut src_clone = src_clone.clone();
          let src_source = op_a(&mut src_clone);
          if src_source.kind() != IrOpKind::Constant {
            substitute(function, inst, src_source);
            return;
          }
        }

        if src_clone.cmd == IrCmd::AddNum || src_clone.cmd == IrCmd::SubNum {
          let mut src_clone_a = src_clone.clone();
          let src_a = op_a(&mut src_clone_a);
          let src_b = op_b(src_clone.clone());
          let add_src_1 = function.as_inst_op(src_a);
          let add_num_1 = function.as_double_op(src_a);
          let add_src_2 = function.as_inst_op(src_b);
          let add_num_2 = function.as_double_op(src_b);

          let replacement_cmd = if src_clone.cmd == IrCmd::AddNum {
            IrCmd::AddInt
          } else {
            IrCmd::SubInt
          };

          if !add_src_1.is_null()
            && unsafe { (*add_src_1).cmd } == IrCmd::UintToNum
            && !add_src_2.is_null()
            && unsafe { (*add_src_2).cmd } == IrCmd::UintToNum
          {
            let mut add_src_1_clone = unsafe { (*add_src_1).clone() };
            let mut add_src_2_clone = unsafe { (*add_src_2).clone() };
            let mut ops = IrOps::new();
            ops.push(op_a(&mut add_src_1_clone));
            ops.push(op_a(&mut add_src_2_clone));
            replace_ir_function_ir_block_u32_ir_inst(
              function,
              block,
              index,
              IrInst {
                cmd: replacement_cmd,
                ops,
                ..IrInst::default()
              },
            );
            return;
          } else if let Some(add_num_1) = add_num_1 {
            if safe_integer_constant(add_num_1)
              && !add_src_2.is_null()
              && unsafe { (*add_src_2).cmd } == IrCmd::UintToNum
            {
              let mut add_src_2_clone = unsafe { (*add_src_2).clone() };
              let mut ops = IrOps::new();
              ops.push(build.const_int((add_num_1 as i64 as u32) as i32));
              ops.push(op_a(&mut add_src_2_clone));
              replace_ir_function_ir_block_u32_ir_inst(
                function,
                block,
                index,
                IrInst {
                  cmd: replacement_cmd,
                  ops,
                  ..IrInst::default()
                },
              );
              return;
            }
          } else if !add_src_1.is_null()
            && unsafe { (*add_src_1).cmd } == IrCmd::UintToNum
            && let Some(add_num_2) = add_num_2
            && safe_integer_constant(add_num_2)
          {
            let mut add_src_1_clone = unsafe { (*add_src_1).clone() };
            let mut ops = IrOps::new();
            ops.push(op_a(&mut add_src_1_clone));
            ops.push(build.const_int((add_num_2 as i64 as u32) as i32));
            replace_ir_function_ir_block_u32_ir_inst(
              function,
              block,
              index,
              IrInst {
                cmd: replacement_cmd,
                ops,
                ..IrInst::default()
              },
            );
            return;
          }
        }
      }

      state.substitute_or_record(inst, index);
    }
    IrCmd::TruncateUint => {
      let src = function.as_inst_op(op_a(inst));
      if !src.is_null() && !produces_dirty_high_register_bits(unsafe { (*src).cmd }) {
        let source = op_a(inst);
        substitute(function, inst, source);
      } else {
        state.substitute_or_record(inst, index);
      }
    }
    IrCmd::FloatToNum => {
      state.substitute_or_record(inst, index);
    }
    IrCmd::NumToFloat => {
      let src = function.as_inst_op(op_a(inst));
      if !src.is_null() {
        let src_clone = unsafe { (*src).clone() };
        if src_clone.cmd == IrCmd::FloatToNum {
          let mut src_clone = src_clone;
          substitute(function, inst, op_a(&mut src_clone));
        } else if src_clone.cmd == IrCmd::UintToNum {
          let mut src_clone = src_clone;
          let mut ops = IrOps::new();
          ops.push(op_a(&mut src_clone));
          replace_ir_function_ir_block_u32_ir_inst(
            function,
            block,
            index,
            IrInst {
              cmd: IrCmd::UintToFloat,
              ops,
              ..IrInst::default()
            },
          );
        } else {
          state.substitute_or_record(inst, index);
        }
      } else {
        state.substitute_or_record(inst, index);
      }
    }
    IrCmd::JumpIfTruthy => {
      let tag = state.try_get_tag(op_a(inst));
      if tag != 0xff {
        if tag == LuaType::Nil as u8 {
          let mut ops = IrOps::new();
          ops.push(op_c(inst.clone()));
          replace_ir_function_ir_block_u32_ir_inst(
            function,
            block,
            index,
            IrInst {
              cmd: IrCmd::JUMP,
              ops,
              ..IrInst::default()
            },
          );
        } else if tag != LuaType::Boolean as u8 {
          let mut ops = IrOps::new();
          ops.push(op_b(inst.clone()));
          replace_ir_function_ir_block_u32_ir_inst(
            function,
            block,
            index,
            IrInst {
              cmd: IrCmd::JUMP,
              ops,
              ..IrInst::default()
            },
          );
        }
      }
    }
    IrCmd::JumpIfFalsy => {
      let tag = state.try_get_tag(op_a(inst));
      if tag != 0xff {
        if tag == LuaType::Nil as u8 {
          let mut ops = IrOps::new();
          ops.push(op_b(inst.clone()));
          replace_ir_function_ir_block_u32_ir_inst(
            function,
            block,
            index,
            IrInst {
              cmd: IrCmd::JUMP,
              ops,
              ..IrInst::default()
            },
          );
        } else if tag != LuaType::Boolean as u8 {
          let mut ops = IrOps::new();
          ops.push(op_c(inst.clone()));
          replace_ir_function_ir_block_u32_ir_inst(
            function,
            block,
            index,
            IrInst {
              cmd: IrCmd::JUMP,
              ops,
              ..IrInst::default()
            },
          );
        }
      }
    }
    IrCmd::CmpAny => {
      state.invalidate_user_call();
    }
    IrCmd::CmpSplitTvalue => {
      let tag_a_op = op_a(inst);
      let tag_b_op = op_b(inst.clone());
      let tag_a = if tag_a_op.kind() == IrOpKind::Constant {
        function.tag_op(tag_a_op)
      } else {
        state.try_get_tag(tag_a_op)
      };
      let tag_b = if tag_b_op.kind() == IrOpKind::Constant {
        function.tag_op(tag_b_op)
      } else {
        state.try_get_tag(tag_b_op)
      };

      if tag_a == LuaType::String as u8
        && tag_b == LuaType::String as u8
        && let Some((source, tag)) =
          type_name_tag_comparison(function, op_c(inst.clone()), op_d(inst.clone()))
      {
        let tag_op = build.const_tag(tag);
        let replacement =
          const_prop_make_inst(IrCmd::CmpTag, &[source, tag_op, op_e(inst.clone())]);

        replace_ir_function_ir_block_u32_ir_inst(function, block, index, replacement);
        fold_constants(build, function, block, index);
      }
    }
    IrCmd::JumpEqPointer => {
      if let Some((source, tag)) =
        type_name_tag_comparison(function, op_a(inst), op_b(inst.clone()))
      {
        let tag_op = build.const_tag(tag);
        let replacement = const_prop_make_inst(
          IrCmd::JumpEqTag,
          &[source, tag_op, op_c(inst.clone()), op_d(inst.clone())],
        );

        replace_ir_function_ir_block_u32_ir_inst(function, block, index, replacement);
        fold_constants(build, function, block, index);
      }
    }
    IrCmd::JumpEqTag => {
      let a = op_a(inst);
      let b = op_b(inst.clone());
      let tag_a = if a.kind() == IrOpKind::Constant {
        function.tag_op(a)
      } else {
        state.try_get_tag(a)
      };
      let tag_b = if b.kind() == IrOpKind::Constant {
        function.tag_op(b)
      } else {
        state.try_get_tag(b)
      };

      if tag_a != 0xff && tag_b != 0xff {
        let target = if tag_a == tag_b {
          op_c(inst.clone())
        } else {
          op_d(inst.clone())
        };
        let mut ops = IrOps::new();
        ops.push(target);
        replace_ir_function_ir_block_u32_ir_inst(
          function,
          block,
          index,
          IrInst {
            cmd: IrCmd::JUMP,
            ops,
            ..IrInst::default()
          },
        );
      } else if a == b {
        let mut ops = IrOps::new();
        ops.push(op_c(inst.clone()));
        replace_ir_function_ir_block_u32_ir_inst(
          function,
          block,
          index,
          IrInst {
            cmd: IrCmd::JUMP,
            ops,
            ..IrInst::default()
          },
        );
      }
    }
    IrCmd::JumpCmpInt => {
      let a = op_a(inst);
      let b = op_b(inst.clone());
      let value_a = function.as_int_op(if a.kind() == IrOpKind::Constant {
        a
      } else {
        state.try_get_value(a)
      });
      let value_b = function.as_int_op(if b.kind() == IrOpKind::Constant {
        b
      } else {
        state.try_get_value(b)
      });

      if let (Some(value_a), Some(value_b)) = (value_a, value_b) {
        let target =
          if compare_i32_i32_ir_condition(value_a, value_b, condition_op(op_c(inst.clone()))) {
            op_d(inst.clone())
          } else {
            op_e(inst.clone())
          };
        let mut ops = IrOps::new();
        ops.push(target);
        replace_ir_function_ir_block_u32_ir_inst(
          function,
          block,
          index,
          IrInst {
            cmd: IrCmd::JUMP,
            ops,
            ..IrInst::default()
          },
        );
      }
    }
    IrCmd::JumpCmpNum => {
      let a = op_a(inst);
      let b = op_b(inst.clone());
      let value_a = function.as_double_op(if a.kind() == IrOpKind::Constant {
        a
      } else {
        state.try_get_value(a)
      });
      let value_b = function.as_double_op(if b.kind() == IrOpKind::Constant {
        b
      } else {
        state.try_get_value(b)
      });

      if let (Some(value_a), Some(value_b)) = (value_a, value_b) {
        let target =
          if compare_f64_f64_ir_condition(value_a, value_b, condition_op(op_c(inst.clone()))) {
            op_d(inst.clone())
          } else {
            op_e(inst.clone())
          };
        let mut ops = IrOps::new();
        ops.push(target);
        replace_ir_function_ir_block_u32_ir_inst(
          function,
          block,
          index,
          IrInst {
            cmd: IrCmd::JUMP,
            ops,
            ..IrInst::default()
          },
        );
      }
    }
    IrCmd::CheckTag => {
      let target = op_a(inst);
      let expected = function.tag_op(op_b(inst.clone()));
      let mut tag = state.try_get_tag(target);

      if tag == 0xff {
        let value = state.try_get_value(target);
        if value.kind() == IrOpKind::Constant {
          let constant = function.const_op(value);
          if constant.kind == IrConstKind::Double {
            tag = LuaType::Number as u8;
          } else if constant.kind == IrConstKind::Int64 {
            tag = LuaType::Integer as u8;
          }
        }
      }

      if tag != 0xff {
        if tag == expected {
          kill_ir_function_ir_inst(function, inst);
        } else {
          let mut ops = IrOps::new();
          ops.push(op_c(inst.clone()));
          replace_ir_function_ir_block_u32_ir_inst(
            function,
            block,
            index,
            IrInst {
              cmd: IrCmd::JUMP,
              ops,
              ..IrInst::default()
            },
          );
        }
      } else {
        let lhs = function.as_inst_op(target);
        if !lhs.is_null() {
          let mut lhs_inst = unsafe { (*lhs).clone() };
          let lhs_source = op_a(&mut lhs_inst);
          if lhs_inst.cmd == IrCmd::LoadTag
            && lhs_source.kind() == IrOpKind::VmReg
            && let Some(prev_idx) =
              state.get_previous_versioned_load_index(IrCmd::LoadTvalue, lhs_source)
          {
            state.inst_tag.try_insert(unsafe { *prev_idx }, expected);
          }
        }

        state.update_tag(target, expected);
      }
    }
    IrCmd::NumToInt64 => {
      // INT64_TO_NUM followed by NUM_TO_INT64 of the same source is the identity.
      let src = function.as_inst_op(op_a(inst));
      if !src.is_null() {
        let src_clone = unsafe { (*src).clone() };
        let src_cmd = src_clone.cmd;
        if src_cmd == IrCmd::Int64ToNum {
          let src_op_a = op_a(&mut { src_clone });
          substitute(function, inst, src_op_a);
          return;
        }
        if src_cmd == IrCmd::AddNum {
          let src_op_b = op_b(src_clone.clone());
          if let Some(arg) = function.as_double_op(src_op_b)
            && arg == 0.0
          {
            let src_op_a = op_a(&mut { src_clone.clone() });
            inst.ops[0] = src_op_a;
            state.substitute_or_record(inst, index);
            return;
          }
          let src_op_a = op_a(&mut { src_clone.clone() });
          if let Some(arg) = function.as_double_op(src_op_a)
            && arg == 0.0
          {
            let src_op_b = op_b(src_clone);
            inst.ops[0] = src_op_b;
            state.substitute_or_record(inst, index);
            return;
          }
        }
      }
      state.substitute_or_record(inst, index);
    }
    IrCmd::LoadEnv => {
      if LuauCodegenExtraTableOpts.get() {
        if state.load_env_idx != K_INVALID_INST_IDX {
          substitute(
            function,
            inst,
            IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, state.load_env_idx),
          );
        } else {
          state.load_env_idx = index;
        }
      }
    }
    IrCmd::GetSlotNodeAddr => {
      for i in 0..state.get_slot_node_cache.len() {
        let prev_idx = state.get_slot_node_cache[i].inst_idx;
        let mut prev = function.instructions[prev_idx as usize].clone();

        if op_a(&mut prev) == op_a(inst) && op_c(prev.clone()) == op_c(inst.clone()) {
          let limit = LuauCodeGenLiveSlotReuseLimit.get();

          if state.get_slot_node_cache.len() as i32 > limit {
            let mut cache = state.get_slot_node_cache.clone();
            if state.get_max_internal_overlap(&mut cache, i) > limit {
              return;
            }
          }

          state.get_slot_node_cache[i].finish_pos = state.inst_pos;

          substitute(
            function,
            inst,
            IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, prev_idx),
          );
          return;
        }
      }

      if (state.get_slot_node_cache.len() as i32) < LuauCodeGenReuseSlotLimit.get() {
        state.get_slot_node_cache.push(NumberedInstruction {
          inst_idx: index,
          start_pos: state.inst_pos,
          finish_pos: state.inst_pos,
        });
      }
    }
    IrCmd::GetArrAddr => {
      for prev_idx in state.get_arr_addr_cache.iter().copied() {
        let mut prev = function.instructions[prev_idx as usize].clone();

        if op_a(&mut prev) == op_a(inst) && op_b(prev.clone()) == op_b(inst.clone()) {
          substitute(
            function,
            inst,
            IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, prev_idx),
          );
          return;
        }
      }

      if (state.get_arr_addr_cache.len() as i32) < LuauCodeGenReuseSlotLimit.get() {
        state.get_arr_addr_cache.push(index);
      }
    }
    IrCmd::AddInt
    | IrCmd::SubInt
    | IrCmd::AddInt64
    | IrCmd::SubInt64
    | IrCmd::MulInt64
    | IrCmd::DivInt64
    | IrCmd::IdivInt64
    | IrCmd::CheckDivInt64
    | IrCmd::UdivInt64
    | IrCmd::RemInt64
    | IrCmd::UremInt64
    | IrCmd::ModInt64
    | IrCmd::Sexti8Int
    | IrCmd::Sexti16Int => {
      state.substitute_or_record(inst, index);
    }
    IrCmd::TryNumToIndex => {
      for prev_idx in state.try_num_to_index_cache.iter().copied() {
        let mut prev = function.instructions[prev_idx as usize].clone();

        if op_a(&mut prev) == op_a(inst) {
          substitute(
            function,
            inst,
            IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, prev_idx),
          );
          return;
        }
      }

      if (state.try_num_to_index_cache.len() as i32) < LuauCodeGenReuseSlotLimit.get() {
        state.try_num_to_index_cache.push(index);
      }
    }
    IrCmd::CheckSlotMatch => {
      for el in &mut state.check_slot_match_cache {
        let mut prev = function.instructions[el.pointer as usize].clone();

        if op_a(&mut prev) == op_a(inst) && op_b(prev.clone()) == op_b(inst.clone()) {
          if let Some(info) = state.inst_tag.find(&op_a(inst).index())
            && *info != LuaType::Nil as u8
          {
            el.known_to_not_be_nil = true;
          }

          if el.known_to_not_be_nil {
            kill_ir_function_ir_inst(function, inst);
          } else {
            let mut ops = IrOps::new();
            ops.push(op_a(inst));
            ops.push(op_c(inst.clone()));
            replace_ir_function_ir_block_u32_ir_inst(
              function,
              block,
              index,
              IrInst {
                cmd: IrCmd::CheckNodeValue,
                ops,
                ..IrInst::default()
              },
            );
          }

          el.known_to_not_be_nil = true;
          return;
        }
      }

      if (state.check_slot_match_cache.len() as i32) < LuauCodeGenReuseSlotLimit.get() {
        state.check_slot_match_cache.push(NodeSlotState {
          pointer: index,
          known_to_not_be_nil: true,
        });
      }
    }
    IrCmd::CheckSafeEnv => {
      if state.in_safe_env {
        kill_ir_function_ir_inst(function, inst);
      } else {
        state.in_safe_env = true;
      }
    }
    IrCmd::CheckReadonly => {
      let target = op_a(inst);
      if LuauCodegenExtraTableOpts.get() && target.kind() == IrOpKind::Inst {
        let target_idx = target.index();
        if state.inst_not_readonly.find(&target_idx).is_some() {
          kill_ir_function_ir_inst(function, inst);
          return;
        }
        state.inst_not_readonly.insert(target_idx);
      } else if !LuauCodegenExtraTableOpts.get()
        && let Some(info) = state.try_get_register_info(target)
      {
        unsafe {
          if (*info).known_not_readonly_deprecated {
            kill_ir_function_ir_inst(function, inst);
          } else {
            (*info).known_not_readonly_deprecated = true;
          }
        }
      }
    }
    IrCmd::CheckNoMetatable => {
      let target = op_a(inst);
      if LuauCodegenExtraTableOpts.get() && target.kind() == IrOpKind::Inst {
        let target_idx = target.index();
        if state.inst_no_metatable.find(&target_idx).is_some() {
          kill_ir_function_ir_inst(function, inst);
          return;
        }
        state.inst_no_metatable.insert(target_idx);
      } else if !LuauCodegenExtraTableOpts.get()
        && let Some(info) = state.try_get_register_info(target)
      {
        unsafe {
          if (*info).known_no_metatable_deprecated {
            kill_ir_function_ir_inst(function, inst);
          } else {
            (*info).known_no_metatable_deprecated = true;
          }
        }
      }
    }
    IrCmd::BufferReadi8 => {
      state.substitute_or_record_buffer_load(block, index, inst, 1);
    }
    IrCmd::BufferReadu8 => {
      state.substitute_or_record_buffer_load(block, index, inst, 1);
    }
    IrCmd::BufferWritei8 => {
      let src = function.as_inst_op(op_c(inst.clone()));
      if !src.is_null() {
        let src_inst = unsafe { (*src).clone() };
        let int_src_b = function.as_int_op(opt_op_b(src_inst.clone()));

        if src_inst.cmd == IrCmd::Sexti8Int
          || (src_inst.cmd == IrCmd::BitandUint && int_src_b == Some(0xff))
        {
          let replacement = op_a(&mut src_inst.clone());
          replace_ir_function_ir_op_ir_op(function, &mut inst.ops[2], replacement);
        }
      }

      state.forward_buffer_store_to_load(inst, IrCmd::BufferReadi8, 1);
    }
    IrCmd::BufferReadi16 => {
      state.substitute_or_record_buffer_load(block, index, inst, 2);
    }
    IrCmd::BufferReadu16 => {
      state.substitute_or_record_buffer_load(block, index, inst, 2);
    }
    IrCmd::BufferWritei16 => {
      let src = function.as_inst_op(op_c(inst.clone()));
      if !src.is_null() {
        let src_inst = unsafe { (*src).clone() };
        let int_src_b = function.as_int_op(opt_op_b(src_inst.clone()));

        if src_inst.cmd == IrCmd::Sexti16Int
          || (src_inst.cmd == IrCmd::BitandUint && int_src_b == Some(0xffff))
        {
          let replacement = op_a(&mut src_inst.clone());
          replace_ir_function_ir_op_ir_op(function, &mut inst.ops[2], replacement);
        }
      }

      state.forward_buffer_store_to_load(inst, IrCmd::BufferReadi16, 2);
    }
    IrCmd::BufferReadi32 => {
      state.substitute_or_record_buffer_load(block, index, inst, 4);
    }
    IrCmd::BufferWritei32 => {
      let src = function.as_inst_op(op_c(inst.clone()));
      if !src.is_null() {
        let src_inst = unsafe { (*src).clone() };

        if src_inst.cmd == IrCmd::TruncateUint {
          let replacement = op_a(&mut src_inst.clone());
          replace_ir_function_ir_op_ir_op(function, &mut inst.ops[2], replacement);
        }
      }

      state.forward_buffer_store_to_load(inst, IrCmd::BufferReadi32, 4);
    }
    IrCmd::BufferReadf32 => {
      state.substitute_or_record_buffer_load(block, index, inst, 4);
    }
    IrCmd::BufferWritef32 => {
      state.forward_buffer_store_to_load(inst, IrCmd::BufferReadf32, 4);
    }
    IrCmd::BufferReadf64 => {
      state.substitute_or_record_buffer_load(block, index, inst, 8);
    }
    IrCmd::BufferWritef64 => {
      state.forward_buffer_store_to_load(inst, IrCmd::BufferReadf64, 8);
    }
    IrCmd::BufferReadi64 => {
      state.substitute_or_record_buffer_load(block, index, inst, 8);
    }
    IrCmd::BufferWritei64 => {
      state.forward_buffer_store_to_load(inst, IrCmd::BufferReadi64, 8);
    }
    IrCmd::CheckGc => {
      if state.checked_gc {
        kill_ir_function_ir_inst(function, inst);
      } else {
        state.checked_gc = true;
        state.invalidate_heap_table_data();
      }
    }
    IrCmd::BarrierObj | IrCmd::BarrierTableForward => {
      let value = op_b(inst.clone());
      if value.kind() == IrOpKind::VmReg {
        let tag = state.try_get_tag(value);
        if tag != 0xff && !is_gco(tag) {
          kill_ir_function_ir_inst(function, inst);
        }
      }
    }
    IrCmd::NewTable => {
      if LuauCodegenExtraTableOpts.get() {
        let array_size = function.uint_op(op_a(inst)) as i32;
        state.inst_not_readonly.insert(index);
        state.inst_no_metatable.insert(index);
        state.inst_array_size.try_insert(index, array_size);
      }
    }
    IrCmd::CheckArraySize => {
      let target = op_a(inst);
      let boundary = op_b(inst.clone());
      let boundary_value = if boundary.kind() == IrOpKind::Constant {
        function.as_int_op(boundary)
      } else {
        function.as_int_op(state.try_get_value(boundary))
      };

      if let Some(array_index) = boundary_value {
        if array_index < 0 {
          let mut ops = IrOps::new();
          ops.push(op_c(inst.clone()));
          replace_ir_function_ir_block_u32_ir_inst(
            function,
            block,
            index,
            IrInst {
              cmd: IrCmd::JUMP,
              ops,
              ..IrInst::default()
            },
          );
          return;
        }

        if LuauCodegenExtraTableOpts.get()
          && target.kind() == IrOpKind::Inst
          && let Some(known_array_size) = state.inst_array_size.find(&target.index())
          && *known_array_size >= 0
        {
          if (array_index as u32) < (*known_array_size as u32) {
            kill_ir_function_ir_inst(function, inst);
          } else {
            let mut ops = IrOps::new();
            ops.push(op_c(inst.clone()));
            replace_ir_function_ir_block_u32_ir_inst(
              function,
              block,
              index,
              IrInst {
                cmd: IrCmd::JUMP,
                ops,
                ..IrInst::default()
              },
            );
          }
          return;
        }

        if let Some(info) = state.try_get_register_info(target) {
          unsafe {
            if (*info).known_table_array_size_deprecated >= 0 {
              if (array_index as u32) < ((*info).known_table_array_size_deprecated as u32) {
                kill_ir_function_ir_inst(function, inst);
              } else {
                let mut ops = IrOps::new();
                ops.push(op_c(inst.clone()));
                replace_ir_function_ir_block_u32_ir_inst(
                  function,
                  block,
                  index,
                  IrInst {
                    cmd: IrCmd::JUMP,
                    ops,
                    ..IrInst::default()
                  },
                );
              }
              return;
            }
          }
        }
      }

      for prev_idx in state.check_array_size_cache.iter().copied() {
        let mut prev = function.instructions[prev_idx as usize].clone();

        if op_a(&mut prev) != op_a(inst) {
          continue;
        }

        let prev_boundary = op_b(prev.clone());
        let boundary = op_b(inst.clone());
        let mut same_boundary = prev_boundary == boundary;

        if !same_boundary
          && boundary.kind() == IrOpKind::Constant
          && prev_boundary.kind() == IrOpKind::Constant
          && (function.int_op(boundary) as u32) < (function.int_op(prev_boundary) as u32)
        {
          same_boundary = true;
        }

        if same_boundary {
          kill_ir_function_ir_inst(function, inst);
          return;
        }
      }

      if (state.check_array_size_cache.len() as i32) < LuauCodeGenReuseSlotLimit.get() {
        state.check_array_size_cache.push(index);
      }
    }
    IrCmd::CheckBufferLen => {
      let buffer_offset_op = op_b(inst.clone());
      let buffer_offset = if buffer_offset_op.kind() == IrOpKind::Constant {
        function.as_int_op(buffer_offset_op)
      } else {
        function.as_int_op(state.try_get_value(buffer_offset_op))
      };

      let min_offset = function.int_op(op_c(inst.clone()));
      let max_offset = function.int_op(op_d(inst.clone()));
      crate::macros::codegen_assert::CODEGEN_ASSERT!(min_offset < max_offset);
      let access_size = max_offset - min_offset;
      crate::macros::codegen_assert::CODEGEN_ASSERT!(access_size > 0);

      if let Some(buffer_offset) = buffer_offset
        && (buffer_offset < 0
          || (buffer_offset as u32).wrapping_add(access_size as u32) >= i32::MAX as u32)
      {
        let mut ops = IrOps::new();
        ops.push(op_f(inst.clone()));
        replace_ir_function_ir_block_u32_ir_inst(
          function,
          block,
          index,
          IrInst {
            cmd: IrCmd::JUMP,
            ops,
            ..IrInst::default()
          },
        );
        return;
      }

      for prev_idx in state.check_buffer_len_cache.clone() {
        let prev_ptr = &mut function.instructions[prev_idx as usize] as *mut IrInst;
        let prev = unsafe { &mut *prev_ptr };

        if prev.cmd != IrCmd::CheckBufferLen {
          continue;
        }

        if op_a(prev) == op_a(inst)
          && op_b(prev.clone()) == op_b(inst.clone())
          && op_c(prev.clone()) == op_c(inst.clone())
          && op_d(prev.clone()) == op_d(inst.clone())
        {
          if DebugLuauAbortingChecks.get() {
            let replacement = build.undef();
            replace_ir_function_ir_op_ir_op(function, &mut inst.ops[5], replacement);
          } else {
            kill_ir_function_ir_inst(function, inst);
          }
          return;
        }

        if op_a(prev) == op_a(inst)
          && op_b(inst.clone()).kind() == IrOpKind::Constant
          && op_b(prev.clone()).kind() == IrOpKind::Constant
        {
          let curr_bound = function.int_op(op_b(inst.clone()));
          let prev_bound = function.int_op(op_b(prev.clone()));
          crate::macros::codegen_assert::CODEGEN_ASSERT!(curr_bound >= 0);
          crate::macros::codegen_assert::CODEGEN_ASSERT!(prev_bound >= 0);

          let extra_offset = curr_bound - prev_bound;
          if state.try_merge_and_kill_buffer_length_check(build, block, inst, prev, extra_offset) {
            return;
          }

          continue;
        }

        if state.try_merge_buffer_range_check(build, block, inst, prev) {
          return;
        }
      }

      if (state.check_buffer_len_cache.len() as i32) < LuauCodeGenReuseSlotLimit.get() {
        state.check_buffer_len_cache.push(index);
      }
    }
    IrCmd::AddVec
    | IrCmd::SubVec
    | IrCmd::MulVec
    | IrCmd::DivVec
    | IrCmd::IdivVec
    | IrCmd::DotVec
    | IrCmd::MinVec
    | IrCmd::MaxVec => {
      let a = op_a(inst);
      let a_ptr = function.as_inst_op(a);
      if !a_ptr.is_null() {
        let a_inst = unsafe { (*a_ptr).clone() };
        if a_inst.cmd == IrCmd::TagVector {
          let replacement = op_a(&mut a_inst.clone());
          replace_ir_function_ir_op_ir_op(function, &mut inst.ops[0], replacement);
        }
      }

      let b = op_b(inst.clone());
      let b_ptr = function.as_inst_op(b);
      if !b_ptr.is_null() {
        let b_inst = unsafe { (*b_ptr).clone() };
        if b_inst.cmd == IrCmd::TagVector {
          let replacement = op_a(&mut b_inst.clone());
          replace_ir_function_ir_op_ir_op(function, &mut inst.ops[1], replacement);
        }
      }

      state.substitute_or_record(inst, index);
    }
    IrCmd::UnmVec | IrCmd::FloorVec | IrCmd::CeilVec | IrCmd::AbsVec => {
      let a = op_a(inst);
      let a_ptr = function.as_inst_op(a);
      if !a_ptr.is_null() {
        let a_inst = unsafe { (*a_ptr).clone() };
        if a_inst.cmd == IrCmd::TagVector {
          let replacement = op_a(&mut a_inst.clone());
          replace_ir_function_ir_op_ir_op(function, &mut inst.ops[0], replacement);
        }
      }

      state.substitute_or_record(inst, index);
    }
    IrCmd::FloatToVec | IrCmd::TagVector => {
      state.substitute_or_record(inst, index);
    }
    IrCmd::InvokeLibm => {
      state.substitute_or_record(inst, index);
    }
    IrCmd::DoArith => {
      let target = op_a(inst);
      state.invalidate_ir_op(target);
      state.invalidate_user_call();
    }
    IrCmd::DoLen => {
      let target = op_a(inst);
      state.invalidate_ir_op(target);
      state.invalidate_user_call();
      state.save_tag(target, LuaType::Number as u8);
    }
    IrCmd::GetTable => {
      let target = op_a(inst);
      state.invalidate_ir_op(target);
      state.invalidate_user_call();
    }
    IrCmd::SetTable => {
      state.invalidate_user_call();
    }
    IrCmd::GetCachedImport => {
      let target = op_a(inst);
      state.invalidate_ir_op(target);

      if state.in_safe_env {
        state.invalidate_value_propagation();
      } else {
        state.invalidate_user_call();
      }
    }
    IrCmd::SETLIST => {
      if LuauCodegenExtraTableOpts.get() {
        if let Some(load_idx) =
          state.get_previous_versioned_load_index(IrCmd::LoadPointer, op_b(inst.clone()))
        {
          let load_idx = unsafe { *load_idx };
          if let Some(known_array_size) = state.inst_array_size.find(&load_idx)
            && *known_array_size >= 0
          {
            let replacement = build.const_uint(*known_array_size as u32);
            replace_ir_function_ir_op_ir_op(function, &mut inst.ops[5], replacement);
          }
        }
      } else if let Some(info) = state.try_get_register_info(op_b(inst.clone())) {
        unsafe {
          if (*info).known_table_array_size_deprecated >= 0 {
            let replacement = build.const_uint((*info).known_table_array_size_deprecated as u32);
            replace_ir_function_ir_op_ir_op(function, &mut inst.ops[5], replacement);
          }
        }
      }

      state.invalidate_value_propagation();
      state.invalidate_heap_table_data();
      state.invalidate_heap_buffer_data();
    }
    IrCmd::TableSetnum => {
      state.invalidate_table_array_size();
    }
    IrCmd::CONCAT => {
      let first_reg = vm_reg_op(op_a(inst));
      let count = function.uint_op(op_b(inst.clone())) as i32;
      state.invalidate_register_range(first_reg, count);
      state.invalidate_user_call();
    }
    IrCmd::FallbackGetvarargs => {
      let first_reg = vm_reg_op(op_b(inst.clone()));
      let count = function.int_op(op_c(inst.clone()));
      state.invalidate_register_range(first_reg, count);
    }
    IrCmd::FASTCALL => {
      let bfid =
        unsafe { transmute::<u8, LuauBuiltinFunction>(function.uint_op(op_a(inst)) as u8) };
      let first_return_reg = vm_reg_op(op_b(inst.clone()));
      let nresults = function.int_op(op_d(inst.clone()));

      handle_builtin_effects(state, bfid, first_return_reg as u32, nresults);

      match bfid {
        LuauBuiltinFunction::LBF_MATH_MODF | LuauBuiltinFunction::LBF_MATH_FREXP => {
          let target = IrOp::ir_op_ir_op_kind_u32(IrOpKind::VmReg, first_return_reg as u32);
          state.update_tag(target, LuaType::Number as u8);

          if nresults > 1 {
            let target = IrOp::ir_op_ir_op_kind_u32(IrOpKind::VmReg, (first_return_reg + 1) as u32);
            state.update_tag(target, LuaType::Number as u8);
          }
        }
        _ => {}
      }
    }
    IrCmd::InvokeFastcall => {
      let bfid =
        unsafe { transmute::<u8, LuauBuiltinFunction>(function.uint_op(op_a(inst)) as u8) };
      let first_return_reg = vm_reg_op(op_b(inst.clone())) as u32;
      let nresults = function.int_op(op_g(inst.clone()));
      handle_builtin_effects(state, bfid, first_return_reg, nresults);
    }
    IrCmd::CALL => {
      let first_reg = vm_reg_op(op_a(inst));
      state.invalidate_registers_from(first_reg);
      state.invalidate_user_call();
    }
    IrCmd::FallbackGetglobal => {
      state.invalidate_ir_op(op_b(inst.clone()));
      state.invalidate_user_call();
    }
    IrCmd::FallbackSetglobal | IrCmd::FallbackSettableks => {
      state.invalidate_user_call();
    }
    IrCmd::FallbackGettableks => {
      state.invalidate_ir_op(op_b(inst.clone()));
      state.invalidate_user_call();
    }
    IrCmd::FallbackNamecall => {
      let target = op_b(inst.clone());
      state.invalidate_ir_op(target);
      state.invalidate_ir_op(IrOp::ir_op_ir_op_kind_u32(
        target.kind(),
        target.index() + 1,
      ));
      state.invalidate_user_call();
    }
    IrCmd::FallbackPrepvarargs => {}
    IrCmd::FallbackDupclosure => {
      state.invalidate_ir_op(op_b(inst.clone()));
      state.invalidate_heap_table_data();
    }
    IrCmd::FallbackForgprep => {
      let target = op_b(inst.clone());
      state.invalidate_ir_op(target);
      state.invalidate_ir_op(IrOp::ir_op_ir_op_kind_u32(
        target.kind(),
        target.index() + 1,
      ));
      state.invalidate_ir_op(IrOp::ir_op_ir_op_kind_u32(
        target.kind(),
        target.index() + 2,
      ));
      state.invalidate_user_call();
    }
    _ => {}
  }
}
