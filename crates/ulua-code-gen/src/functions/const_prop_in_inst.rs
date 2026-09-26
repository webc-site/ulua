use ulua_common::{
  enums::luau_builtin_function::LuauBuiltinFunction,
  fflag::{DebugLuauAbortingChecks, LuauCodegenSubstituteReplacements},
  fint::{LuauCodeGenLiveSlotReuseLimit, LuauCodeGenReuseSlotLimit, LuauCodeGenReuseUdataTagLimit},
};
use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{
    compare_ir_utils::{compare_f64_f64_ir_condition, compare_int},
    condition_op::condition_op,
    fold_constants::fold_constants,
    handle_builtin_effects::handle_builtin_effects,
    is_gco::is_gco,
    kill_ir_utils::kill_ir_function_ir_inst_at,
    produces_dirty_high_register_bits::produces_dirty_high_register_bits,
    proto_constant_string::{proto_constant_vector_component, with_constant_string_name},
    reg_bitset::reg_bit_test,
    replace_ir_utils::{
      replace_ir_function_ir_block_u32_ir_inst, replace_ir_function_ir_op_ir_op_at,
    },
    safe_integer_constant::safe_integer_constant,
    substitute::substitute_at,
    substitute_with_truncated_uint::substitute_with_truncated_uint_at,
    try_get_operand_tag::try_get_operand_tag,
    try_get_tag_for_typename::try_get_tag_for_typename,
    vm_const_op::vm_const_op,
    vm_reg_op::vm_reg_op,
  },
  macros::{
    codegen_assert::CODEGEN_ASSERT, has_op_c::HAS_OP_C,
    ir_operand::{
      op_a_ref, op_b, op_c_ref, op_d_ref, op_e_ref, op_f_ref, op_g_ref, opt_op_b_ref, opt_op_c_ref,
      opt_op_d_ref,
    },
    op_a::op_a, op_b_ref::op_b_ref,
  },
  records::{
    array_value_entry::ArrayValueEntry,
    const_prop_state::ConstPropState,
    ir_builder::ConstantMap,
    ir_const::IrConst,
    ir_data::{K_INVALID_INST_IDX, K_UNKNOWN_TAG},
    ir_function::IrFunction,
    ir_inst::IrInst,
    ir_op::IrOp,
    node_slot_state::NodeSlotState,
    numbered_instruction::NumberedInstruction,
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

/// 当前指令只读视图：以索引即时定位，避免与 `function` 的整体借用重叠。
#[inline]
fn cur_ref(function: &IrFunction, index: u32) -> &IrInst {
  &function.instructions[index as usize]
}

/// 当前指令可变视图：单槽位再借用，与 `function` 的可变借用顺序交割（同一表达式内不同时出现）。
#[inline]
fn cur_mut(function: &mut IrFunction, index: u32) -> &mut IrInst {
  &mut function.instructions[index as usize]
}

/// 生成双操作数整数运算替换指令（NumToUint 的 Add/Sub 组合分支共用）
fn emit_int_arith(
  function: &mut IrFunction,
  block_idx: u32,
  index: u32,
  cmd: IrCmd,
  a: IrOp,
  b: IrOp,
) {
  let mut ops = IrOps::new();
  ops.push(a);
  ops.push(b);
  replace_ir_function_ir_block_u32_ir_inst(
    function,
    block_idx,
    index,
    IrInst {
      cmd,
      ops,
      ..IrInst::default()
    },
  );
}

/// 操作数是 `TagVector` 指令时，返回其内层源操作数
#[inline]
fn tag_vector_source(function: &IrFunction, op: IrOp) -> Option<IrOp> {
  function
    .as_inst_op_ref(op)
    .filter(|src| src.cmd == IrCmd::TagVector)
    .map(op_a_ref)
}

/// 无条件跳转替换：JUMP 只带单个目标操作数
fn replace_with_jump(function: &mut IrFunction, block_idx: u32, index: u32, target: IrOp) {
  replace_ir_function_ir_block_u32_ir_inst(
    function,
    block_idx,
    index,
    const_prop_make_inst(IrCmd::JUMP, &[target]),
  );
}

/// 常量优先取值：非常量走已记录的值映射
#[inline]
fn resolve_value(state: &mut ConstPropState, op: IrOp) -> IrOp {
  if op.kind() == IrOpKind::Constant {
    op
  } else {
    state.try_get_value(op)
  }
}

/// VmReg 加载前奏三件套：先以 T_VALUE 数据尝试替换（`$t_value_data` 为该加载
/// 对应的 T_VALUE 数据替换谓词方法名，区分 tag/value 两类），命中即 return 短路；
/// 否则重定向至 T_VALUE origin 后记录 vm_reg 加载。各调用点均丢弃返回值，三处同构。
macro_rules! vm_reg_prelude {
  ($state:expr, $function:expr, $index:expr, $t_value_data:ident) => {
    if $state.$t_value_data(cur_mut($function, $index)) {
      return;
    }
    $state.try_redirect_vm_reg_load_to_t_value_origin(cur_mut($function, $index));
    $state.substitute_or_record_vm_reg_load(cur_mut($function, $index));
  };
}

/// 算术替换：生成 `cmd(ops...)` 替换当前指令，SUBSTITUTE_REPLACEMENTS 开启时记录替换链
fn substitute_with(
  state: &mut ConstPropState,
  function: &mut IrFunction,
  block_idx: u32,
  index: u32,
  cmd: IrCmd,
  ops: &[IrOp],
) {
  replace_ir_function_ir_block_u32_ir_inst(
    function,
    block_idx,
    index,
    const_prop_make_inst(cmd, ops),
  );

  if LuauCodegenSubstituteReplacements.get() {
    state.substitute_or_record(cur_mut(function, index), index);
  }
}

/// Buffer 整数写：源为符号扩展/掩码/截断的逆变换时用其源操作数替换，再前向。
/// `is_inverse` 标识该位宽的逆变换谓词（i8/i16 查 Sexti/Bitand，i32 查 TruncateUint）
fn forward_buffer_int_store(
  state: &mut ConstPropState,
  function: &mut IrFunction,
  index: u32,
  is_inverse: fn(&IrInst, Option<i32>) -> bool,
  load_cmd: IrCmd,
  size: u8,
) {
  let replacement = function
    .as_inst_op_ref(op_c_ref(cur_ref(function, index)))
    .filter(|src_inst| is_inverse(src_inst, function.as_int_op(opt_op_b_ref(src_inst))))
    .map(op_a_ref);

  if let Some(replacement) = replacement {
    replace_ir_function_ir_op_ir_op_at(function, index, 2, replacement);
  }

  state.forward_buffer_store_to_load(cur_mut(function, index), load_cmd, size);
}

fn tag_for_vm_const_typename(function: &IrFunction, source: IrOp, for_typeof: bool) -> Option<u8> {
  if source.kind() != IrOpKind::VmConst {
    return None;
  }

  // `Proto`/`k`/TValue 裸指针解引用收口到边界门面 with_constant_string_name（§2）：
  // 空 proto/空 k/非 string 常量/非法 UTF-8 均短路为 None，与散点版判定逐项一致
  let tag = with_constant_string_name(function.proto, vm_const_op(source) as u32, |name| {
    try_get_tag_for_typename(name, for_typeof)
  })?;

  if tag == K_UNKNOWN_TAG {
    None
  } else {
    Some(tag)
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

  // function 此处仅只读借用：直接用引用读取操作数，无需克隆指令
  let lhs = function.instructions.get(lhs_op.index() as usize)?;
  let rhs = function.instructions.get(rhs_op.index() as usize)?;

  if lhs.cmd != IrCmd::GetType && lhs.cmd != IrCmd::GetTypeof {
    return None;
  }

  if rhs.cmd != IrCmd::LoadPointer {
    return None;
  }

  let source = op_a_ref(lhs);
  let tag = tag_for_vm_const_typename(function, op_a_ref(rhs), lhs.cmd == IrCmd::GetTypeof)?;

  Some((source, tag))
}

/// 数值类加载（LoadDouble/LoadInt/LoadInt64）：常量值已可用则直接替换，
/// VmReg 源走 T_VALUE 数据/重定向/记录链，其余记录
#[inline]
fn load_numeric(
  state: &mut ConstPropState,
  function: &mut IrFunction,
  index: u32,
  is_value: fn(&mut IrFunction, IrOp) -> bool,
) {
  let source = op_a(cur_mut(function, index));
  let value = state.try_get_value(source);

  if is_value(function, value) {
    substitute_at(function, index, value);
  } else if source.kind() == IrOpKind::VmReg {
    vm_reg_prelude!(
      state,
      function,
      index,
      substitute_or_record_value_load_with_t_value_data
    );
  } else {
    state.substitute_or_record(cur_mut(function, index), index);
  }
}

/// 数值类存储（StoreDouble/StoreInt/StoreInt64）：同值常量重复存储 kill，
/// Inst 源做版本化加载对消，否则失效并前向。
/// 注意：函数体即整个 match，此处 return 等价于退出 const_prop_in_inst。
#[inline]
fn store_numeric(
  state: &mut ConstPropState,
  function: &mut IrFunction,
  index: u32,
  load_cmd: IrCmd,
) {
  let target = op_a(cur_mut(function, index));
  if target.kind() == IrOpKind::VmReg {
    let value = op_b_ref(cur_ref(function, index));
    if value.kind() == IrOpKind::Constant {
      if state.try_get_value(target) == value {
        kill_ir_function_ir_inst_at(function, index);
      } else {
        state.save_value(target, value);
      }
    } else {
      if value.kind() == IrOpKind::Inst
        && let Some(prev_idx) = state.get_previous_versioned_load_index(load_cmd, target)
        && prev_idx == value.index()
      {
        kill_ir_function_ir_inst_at(function, index);
        return;
      }

      state.invalidate_value(target);
      state.forward_vm_reg_store_to_load(cur_mut(function, index), load_cmd);
    }
  }
}

/// 节点/数组加载复用：prev 为 LoadTvalue 且活跃则替换为其结果，
/// StoreSplit/StoreTvalue 则前向记录 tag/value（cpp OptimizeConstProp.cpp:1769）
fn reuse_node_load(
  state: &mut ConstPropState,
  function: &mut IrFunction,
  index: u32,
  prev_idx: u32,
) {
  let prev = function.instructions[prev_idx as usize].clone();

  if prev.cmd == IrCmd::LoadTvalue {
    if prev.use_count != 0 {
      substitute_at(
        function,
        index,
        IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, prev_idx),
      );
    }
  } else if prev.cmd == IrCmd::StoreSplitTvalue {
    state
      .inst_tag
      .try_insert(index, function.tag_op(op_b_ref(&prev)));
    state.inst_value.try_insert(index, op_c_ref(&prev));
  } else if prev.cmd == IrCmd::StoreTvalue {
    // cpp 无条件前向复用节点存储值（OptimizeConstProp.cpp:1769）
    let alive = function
      .as_inst_op_ref(op_b_ref(&prev))
      .is_some_and(|arg| arg.use_count != 0);
    if alive {
      substitute_at(function, index, op_b_ref(&prev));
    }
  }
}

pub fn const_prop_in_inst(
  state: &mut ConstPropState,
  function: &mut IrFunction,
  map: &mut ConstantMap,
  block_idx: u32,
  index: u32,
) {
  match cur_ref(function, index).cmd {
    IrCmd::LoadTag => {
      let source = op_a(cur_mut(function, index));
      let tag = state.try_get_tag(source);
      if tag != K_UNKNOWN_TAG {
        let tag_op = function.const_tag(map, tag);
        substitute_at(function, index, tag_op);
      } else if source.kind() == IrOpKind::VmReg {
        vm_reg_prelude!(
          state,
          function,
          index,
          substitute_tag_load_with_t_value_data
        );
      }
    }
    IrCmd::LoadPointer => {
      let source = op_a(cur_mut(function, index));
      if source.kind() == IrOpKind::VmReg {
        vm_reg_prelude!(
          state,
          function,
          index,
          substitute_or_record_value_load_with_t_value_data
        );
      }
    }
    IrCmd::LoadDouble => {
      load_numeric(state, function, index, |f, v| f.as_double_op(v).is_some());
    }
    IrCmd::LoadInt => {
      load_numeric(state, function, index, |f, v| f.as_int_op(v).is_some());
    }
    IrCmd::LoadInt64 => {
      load_numeric(state, function, index, |f, v| f.as_int_64_op(v).is_some());
    }
    IrCmd::LoadTvalue => {
      let source = op_a(cur_mut(function, index));
      if source.kind() == IrOpKind::VmReg {
        if !state.substitute_or_record_vm_reg_load(cur_mut(function, index))
          && !HAS_OP_C!(cur_ref(function, index))
        {
          let tag = state.try_get_tag(source);
          if tag != K_UNKNOWN_TAG {
            let offset = function.const_int(map, 0);
            let tag_op = function.const_tag(map, tag);
            replace_ir_function_ir_block_u32_ir_inst(
              function,
              block_idx,
              index,
              const_prop_make_inst(IrCmd::LoadTvalue, &[source, offset, tag_op]),
            );
          }
        }
      } else if source.kind() == IrOpKind::Inst {
        let source_inst = function.instructions[source.index() as usize].clone();

        if source_inst.cmd == IrCmd::GetSlotNodeAddr {
          if let Some(prev_idx) = state.hash_value_cache.find(&source.index()).copied()
            && prev_idx != K_INVALID_INST_IDX
          {
            reuse_node_load(state, function, index, prev_idx);
            return;
          }

          *state.hash_value_cache.get_or_insert(source.index()) = index;
        } else if source_inst.cmd == IrCmd::GetArrAddr {
          let source_addr = source_inst;
          let offset_op = state.get_combined_array_load_offset_op(
            &source_addr,
            opt_op_b_ref(cur_mut(function, index)),
          );

          if let Some(entry) = state
            .array_value_cache
            .iter()
            .find(|entry| entry.pointer == source.index() && entry.offset == offset_op)
            .copied()
            && entry.value != K_INVALID_INST_IDX
          {
            reuse_node_load(state, function, index, entry.value);
            return;
          }

          state.array_value_cache.push(ArrayValueEntry {
            pointer: source.index(),
            offset: offset_op,
            value: index,
          });
        } else {
          state.substitute_or_record(cur_mut(function, index), index);
        }
      }
    }
    IrCmd::LoadFloat => {
      let source = op_a(cur_mut(function, index));
      if source.kind() == IrOpKind::VmReg {
        let offset = function.int_op(op_b_ref(cur_ref(function, index)));

        if let Some(subst) = state.find_substitute_component_load_from_store_vector(source, offset)
        {
          substitute_at(function, index, subst);
          return;
        }

        if let Some(mut prev_idx) =
          state.get_previous_versioned_load_index(IrCmd::LoadTvalue, source)
        {
          let prev = function.instructions[prev_idx as usize].clone();

          if prev.cmd == IrCmd::TagVector {
            let prev_arg = op_a_ref(&prev);
            if function.as_inst_op_ref(prev_arg).is_some() {
              prev_idx = prev_arg.index();
            }
          }

          let value = function.instructions[prev_idx as usize].clone();
          let byte_offset = offset as u32;
          CODEGEN_ASSERT!(byte_offset.is_multiple_of(4));

          let component = byte_offset / 4;
          CODEGEN_ASSERT!(component <= 3);

          if value.cmd == IrCmd::LoadTvalue {
            let value_source = op_a_ref(&value);

            if value_source.kind() == IrOpKind::VmConst {
              // `Proto::k` vector 常量分量读取收口到边界门面（§2），
              // 空 proto/空 k/非 vector 常量短路为 None 后走通用 LoweredLoadVectorComponent 路径
              let vector_component = proto_constant_vector_component(
                function.proto,
                vm_const_op(value_source) as u32,
                component as usize,
              );

              if let Some(component_value) = vector_component {
                let subst = function.const_double(map, component_value);
                substitute_at(function, index, subst);
                return;
              }
            } else if value_source.kind() == IrOpKind::VmReg {
              let prev_op = IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, prev_idx);
              if state.try_get_reg_link(prev_op).is_some()
                && let Some(subst) =
                  state.find_substitute_component_load_from_store_vector(value_source, offset)
              {
                substitute_at(function, index, subst);
                return;
              }
            }
          }

          let mut ops = IrOps::new();
          ops.push(IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, prev_idx));
          ops.push(function.const_int(map, component as i32));

          replace_ir_function_ir_block_u32_ir_inst(
            function,
            block_idx,
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

        state.substitute_or_record_vm_reg_load(cur_mut(function, index));
      } else {
        state.substitute_or_record(cur_mut(function, index), index);
      }
    }
    IrCmd::StoreTag => {
      let target = op_a(cur_mut(function, index));
      if target.kind() == IrOpKind::VmReg {
        let mut active_load_cmd = IrCmd::NOP;
        let mut active_load_value = !0u32;
        let value = op_b_ref(cur_ref(function, index));
        if value.kind() == IrOpKind::Constant {
          let tag = function.tag_op(value);
          (active_load_cmd, active_load_value) =
            state.get_previous_versioned_load_for_tag(tag, target);

          if state.try_get_tag(target) == tag {
            kill_ir_function_ir_inst_at(function, index);
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
      let target = op_a(cur_mut(function, index));
      if target.kind() == IrOpKind::VmReg {
        let value = op_b_ref(cur_ref(function, index));
        if value.kind() == IrOpKind::Inst
          && let Some(prev_idx) =
            state.get_previous_versioned_load_index(IrCmd::LoadPointer, target)
          && prev_idx == value.index()
        {
          kill_ir_function_ir_inst_at(function, index);
          return;
        }

        state.invalidate_value(target);
        if value.kind() == IrOpKind::Inst {
          state.forward_vm_reg_store_to_load(cur_mut(function, index), IrCmd::LoadPointer);
        }
      }
    }
    IrCmd::StoreDouble => store_numeric(state, function, index, IrCmd::LoadDouble),
    IrCmd::StoreInt => {
      let stored = op_b_ref(cur_ref(function, index));
      // 借用在 if-let 语句末结束（IrOp 为 Copy），不与下方 &mut 调用冲突
      let replacement = function
        .as_inst_op_ref(stored)
        .filter(|src_inst| src_inst.cmd == IrCmd::TruncateUint)
        .map(op_a_ref);
      if let Some(replacement) = replacement {
        replace_ir_function_ir_op_ir_op_at(function, index, 1, replacement);
      }

      store_numeric(state, function, index, IrCmd::LoadInt);
    }
    IrCmd::StoreInt64 => store_numeric(state, function, index, IrCmd::LoadInt64),
    IrCmd::StoreVector => {
      let target = op_a(cur_mut(function, index));
      if target.kind() == IrOpKind::VmReg {
        state.invalidate_value(target);

        let reg = vm_reg_op(target) as usize;
        let captured_regs = &state.function_ref().cfg.captured.regs;
        if !reg_bit_test(captured_regs, reg) {
          let key = state.versioned_vm_reg_load_ir_cmd_ir_op(IrCmd::LoadFloat, target);
          *state.value_map.get_or_insert(key) = index;
        }
      }
    }
    IrCmd::StoreTvalue => {
      let target = op_a(cur_mut(function, index));
      if matches!(target.kind(), IrOpKind::VmReg | IrOpKind::Inst) {
        let stored = op_b_ref(cur_ref(function, index));

        if target.kind() == IrOpKind::VmReg {
          if stored.kind() == IrOpKind::Inst
            && let Some(prev_idx) =
              state.get_previous_versioned_load_index(IrCmd::LoadTvalue, target)
            && prev_idx == stored.index()
          {
            kill_ir_function_ir_inst_at(function, index);
            return;
          }

          state.invalidate_ir_op(target);
        }

        let mut tag = state.try_get_tag(stored);
        if tag == K_UNKNOWN_TAG {
          tag = try_get_operand_tag(function, stored).unwrap_or(K_UNKNOWN_TAG);
        }

        let mut value = state.try_get_value(stored);

        if target.kind() == IrOpKind::VmReg {
          if tag != K_UNKNOWN_TAG {
            state.save_tag(target, tag);
          }

          if value.kind() != IrOpKind::None {
            state.save_value(target, value);
          }
        }

        if target.kind() == IrOpKind::Inst {
          let target_inst = function.instructions[target.index() as usize].clone();
          state.invalidate_table_store_location(
            target_inst,
            opt_op_c_ref(cur_mut(function, index)),
            tag,
          );
        }

        let mut active_load_cmd = IrCmd::NOP;
        let mut active_load_value = !0u32;

        if tag != K_UNKNOWN_TAG
          && value.kind() == IrOpKind::None
          && state.try_get_reg_link(stored).is_some()
          && let Some(arg) = function.as_inst_op_ref(stored)
        {
          let arg_source = op_a_ref(arg);
          if arg.cmd == IrCmd::LoadTvalue && arg_source.kind() == IrOpKind::VmReg {
            let (cmd, idx) = state.get_previous_versioned_load_for_tag(tag, arg_source);
            if idx != !0u32 {
              active_load_cmd = cmd;
              active_load_value = idx;
              value = IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, idx);
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
          tag != K_UNKNOWN_TAG && is_gco(tag) && value.kind() == IrOpKind::Inst
        };

        if can_split_tvalue_store {
          let tag_op = function.const_tag(map, tag);
          let mut ops = IrOps::new();
          ops.push(target);
          ops.push(tag_op);
          ops.push(value);
          if HAS_OP_C!(cur_ref(function, index)) {
            ops.push(op_c_ref(cur_ref(function, index)));
          }

          let replacement = IrInst {
            cmd: IrCmd::StoreSplitTvalue,
            ops,
            ..IrInst::default()
          };
          let replacement_offset = opt_op_d_ref(&replacement);

          replace_ir_function_ir_block_u32_ir_inst(function, block_idx, index, replacement);

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
            state.forward_table_store_to_load(
              &function.instructions[target.index() as usize],
              replacement_offset,
              index,
            );
          }
        } else if target.kind() == IrOpKind::VmReg {
          state.forward_vm_reg_store_to_load(cur_mut(function, index), IrCmd::LoadTvalue);
        } else if target.kind() == IrOpKind::Inst {
          // cpp 无条件前向记录表节点存储值（OptimizeConstProp.cpp:2079）
          let offset = opt_op_c_ref(cur_mut(function, index));
          state.forward_table_store_to_load(
            &function.instructions[target.index() as usize],
            offset,
            index,
          );
        }
      }
    }
    IrCmd::StoreSplitTvalue => {
      let target = op_a(cur_mut(function, index));
      if target.kind() == IrOpKind::VmReg {
        state.invalidate_ir_op(target);

        let tag = function.tag_op(op_b_ref(cur_ref(function, index)));
        state.save_tag(target, tag);

        let value = op_c_ref(cur_ref(function, index));
        if value.kind() == IrOpKind::Constant {
          state.save_value(target, value);
        }
      } else if target.kind() == IrOpKind::Inst {
        let tag = function.tag_op(op_b_ref(cur_ref(function, index)));
        let offset = opt_op_d_ref(cur_mut(function, index));
        let target_inst = function.instructions[target.index() as usize].clone();
        state.invalidate_table_store_location(target_inst, offset, tag);

        state.forward_table_store_to_load(
          &function.instructions[target.index() as usize],
          offset,
          index,
        );
      }
    }
    IrCmd::GetUpvalue => {
      state.substitute_or_record_vm_upvalue_load(cur_mut(function, index));
    }
    IrCmd::SetUpvalue => {
      state.forward_vm_upvalue_store_to_load(cur_mut(function, index));

      let source = op_b_ref(cur_ref(function, index));
      let tag = state.try_get_tag(source);
      if tag != K_UNKNOWN_TAG {
        let tag_op = function.const_tag(map, tag);
        replace_ir_function_ir_op_ir_op_at(function, index, 2, tag_op);
      }
    }
    IrCmd::Int64ToNum | IrCmd::IntToNum => {
      state.substitute_or_record(cur_mut(function, index), index);
    }
    IrCmd::AddNum | IrCmd::SubNum => {
      let rhs = resolve_value(state, op_b_ref(cur_ref(function, index)));

      if let Some(k) = function.as_double_op(rhs) {
        if k == 0.0 && k.is_sign_negative() == (cur_ref(function, index).cmd == IrCmd::AddNum) {
          let lhs = op_a(cur_mut(function, index));
          substitute_at(function, index, lhs);
        } else {
          state.substitute_or_record(cur_mut(function, index), index);
        }
      } else {
        state.substitute_or_record(cur_mut(function, index), index);
      }
    }
    IrCmd::MulNum => {
      let rhs = resolve_value(state, op_b_ref(cur_ref(function, index)));

      if let Some(k) = function.as_double_op(rhs) {
        let lhs = op_a(cur_mut(function, index));
        if k == 1.0 {
          substitute_at(function, index, lhs);
        } else if k == 2.0 {
          // x * 2 → x + x
          substitute_with(
            state,
            function,
            block_idx,
            index,
            IrCmd::AddNum,
            &[lhs, lhs],
          );
        } else if k == -1.0 {
          substitute_with(state, function, block_idx, index, IrCmd::UnmNum, &[lhs]);
        } else {
          state.substitute_or_record(cur_mut(function, index), index);
        }
      } else {
        state.substitute_or_record(cur_mut(function, index), index);
      }
    }
    IrCmd::DivNum => {
      let rhs = resolve_value(state, op_b_ref(cur_ref(function, index)));

      if let Some(k) = function.as_double_op(rhs) {
        let lhs = op_a(cur_mut(function, index));
        if k == 1.0 {
          substitute_at(function, index, lhs);
        } else if k == -1.0 {
          substitute_with(state, function, block_idx, index, IrCmd::UnmNum, &[lhs]);
        } else {
          // k 为 2 的幂且缩放后指数界内 → x / k ≡ x * (1/k)
          let exp = k.log2();
          if k > 0.0 && k.is_finite() && exp.fract() == 0.0 && (-1001.0..=999.0).contains(&exp) {
            let reciprocal = function.const_double(map, 1.0 / k);
            substitute_with(
              state,
              function,
              block_idx,
              index,
              IrCmd::MulNum,
              &[lhs, reciprocal],
            );
          } else {
            state.substitute_or_record(cur_mut(function, index), index);
          }
        }
      } else {
        state.substitute_or_record(cur_mut(function, index), index);
      }
    }
    IrCmd::AddFloat | IrCmd::SubFloat => {
      let rhs = resolve_value(state, op_b_ref(cur_ref(function, index)));

      if let Some(k) = function.as_double_op(rhs) {
        let kf = k as f32;
        if kf == 0.0 && kf.is_sign_negative() == (cur_ref(function, index).cmd == IrCmd::AddFloat) {
          let lhs = op_a(cur_mut(function, index));
          substitute_at(function, index, lhs);
        } else {
          state.substitute_or_record(cur_mut(function, index), index);
        }
      } else {
        state.substitute_or_record(cur_mut(function, index), index);
      }
    }
    IrCmd::MulFloat => {
      let rhs = resolve_value(state, op_b_ref(cur_ref(function, index)));

      if let Some(k) = function.as_double_op(rhs) {
        let kf = k as f32;
        let lhs = op_a(cur_mut(function, index));
        if kf == 1.0 {
          substitute_at(function, index, lhs);
        } else if kf == 2.0 {
          // x * 2f → x + x
          substitute_with(
            state,
            function,
            block_idx,
            index,
            IrCmd::AddFloat,
            &[lhs, lhs],
          );
        } else if kf == -1.0 {
          substitute_with(state, function, block_idx, index, IrCmd::UnmFloat, &[lhs]);
        } else {
          state.substitute_or_record(cur_mut(function, index), index);
        }
      } else {
        state.substitute_or_record(cur_mut(function, index), index);
      }
    }
    IrCmd::DivFloat => {
      let rhs = resolve_value(state, op_b_ref(cur_ref(function, index)));

      if let Some(k) = function.as_double_op(rhs) {
        let kf = k as f32;
        let lhs = op_a(cur_mut(function, index));
        if kf == 1.0 {
          substitute_at(function, index, lhs);
        } else if kf == -1.0 {
          substitute_with(state, function, block_idx, index, IrCmd::UnmFloat, &[lhs]);
        } else {
          // kf 为 2 的幂且缩放后指数界内 → x / kf ≡ x * (1/kf)
          let exp = kf.log2();
          if kf > 0.0 && kf.is_finite() && exp.fract() == 0.0 && (-1001.0..=999.0).contains(&exp) {
            let reciprocal = function.const_double(map, (1.0f32 / kf) as f64);
            substitute_with(
              state,
              function,
              block_idx,
              index,
              IrCmd::MulFloat,
              &[lhs, reciprocal],
            );
          } else {
            state.substitute_or_record(cur_mut(function, index), index);
          }
        }
      } else {
        state.substitute_or_record(cur_mut(function, index), index);
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
      state.substitute_or_record(cur_mut(function, index), index);
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
      state.substitute_or_record(cur_mut(function, index), index);
    }
    IrCmd::SelectIfTruthy => {
      let tag = state.try_get_tag(op_a(cur_mut(function, index)));

      if tag == LuaType::Nil as u8 {
        let replacement = op_c_ref(cur_ref(function, index));
        substitute_at(function, index, replacement);
      } else if tag != K_UNKNOWN_TAG && tag != LuaType::Boolean as u8 {
        let replacement = op_b_ref(cur_ref(function, index));
        substitute_at(function, index, replacement);
      }
    }
    IrCmd::UintToNum | IrCmd::UintToFloat => {
      // TruncateUint(NumToUint(x)) ≡ x：借用在语句末结束（IrOp 为 Copy）
      let cur_a = op_a(cur_mut(function, index));
      let replacement = function
        .as_inst_op_ref(cur_a)
        .filter(|src_inst| src_inst.cmd == IrCmd::TruncateUint)
        .and_then(|src_inst| {
          let src_source = op_a_ref(src_inst);
          function
            .as_inst_op_ref(src_source)
            .filter(|src_of_src| src_of_src.cmd == IrCmd::NumToUint)
            .map(|_| src_source)
        });

      if let Some(replacement) = replacement {
        replace_ir_function_ir_op_ir_op_at(function, index, 0, replacement);
      }

      state.substitute_or_record(cur_mut(function, index), index);
    }
    IrCmd::NumToInt => {
      let cur_a = op_a(cur_mut(function, index));
      if let Some(src) = function.as_inst_op_ref(cur_a) {
        // 快照一次以提前结束对 function 的不可变借用；后续均为只读访问
        let src_clone = src.clone();

        if src_clone.cmd == IrCmd::IntToNum {
          substitute_at(function, index, op_a_ref(&src_clone));
          return;
        }

        if src_clone.cmd == IrCmd::AddNum {
          if let Some(arg) = function.as_double_op(op_b_ref(&src_clone))
            && arg == 0.0
          {
            replace_ir_function_ir_op_ir_op_at(function, index, 0, op_a_ref(&src_clone));
            state.substitute_or_record(cur_mut(function, index), index);
            return;
          }

          let src_op_a = op_a_ref(&src_clone);
          if let Some(arg) = function.as_double_op(src_op_a)
            && arg == 0.0
          {
            replace_ir_function_ir_op_ir_op_at(function, index, 0, op_b_ref(&src_clone));
            state.substitute_or_record(cur_mut(function, index), index);
            return;
          }
        }

        if src_clone.cmd == IrCmd::UintToNum {
          let src_source = op_a_ref(&src_clone);
          if src_source.kind() != IrOpKind::Constant {
            let truncated =
              substitute_with_truncated_uint_at(function, block_idx, index, src_source);

            if truncated && LuauCodegenSubstituteReplacements.get() {
              state.substitute_or_record(cur_mut(function, index), index);
            }

            return;
          }
        }
      }

      state.substitute_or_record(cur_mut(function, index), index);
    }
    IrCmd::NumToUint => {
      let cur_a = op_a(cur_mut(function, index));
      if let Some(src) = function.as_inst_op_ref(cur_a) {
        // 快照一次以提前结束对 function 的不可变借用；后续均为只读访问
        let src_clone = src.clone();

        if src_clone.cmd == IrCmd::UintToNum {
          let truncated =
            substitute_with_truncated_uint_at(function, block_idx, index, op_a_ref(&src_clone));

          if truncated && LuauCodegenSubstituteReplacements.get() {
            state.substitute_or_record(cur_mut(function, index), index);
          }

          return;
        }

        if src_clone.cmd == IrCmd::IntToNum {
          let src_source = op_a_ref(&src_clone);
          if src_source.kind() != IrOpKind::Constant {
            substitute_at(function, index, src_source);
            return;
          }
        }

        if matches!(src_clone.cmd, IrCmd::AddNum | IrCmd::SubNum) {
          let src_a = op_a_ref(&src_clone);
          let src_b = op_b_ref(&src_clone);

          // 快照两个操作数的指令/常量视图；指令引用在 clone 后即不再使用
          let add_inst_1 = function.as_inst_op_ref(src_a);
          let add_num_1 = function.as_double_op(src_a);
          let add_inst_2 = function.as_inst_op_ref(src_b);
          let add_num_2 = function.as_double_op(src_b);

          let replacement_cmd = if src_clone.cmd == IrCmd::AddNum {
            IrCmd::AddInt
          } else {
            IrCmd::SubInt
          };

          if let (Some(inst_1), Some(inst_2)) = (add_inst_1, add_inst_2)
            && inst_1.cmd == IrCmd::UintToNum
            && inst_2.cmd == IrCmd::UintToNum
          {
            let (a, b) = (op_a_ref(inst_1), op_a_ref(inst_2));
            emit_int_arith(function, block_idx, index, replacement_cmd, a, b);

            if !LuauCodegenSubstituteReplacements.get() {
              return;
            }
          } else if let Some(add_num_1) = add_num_1 {
            if safe_integer_constant(add_num_1)
              && let Some(inst_2) = add_inst_2
              && inst_2.cmd == IrCmd::UintToNum
            {
              let b = op_a_ref(inst_2);
              let a = function.const_int(map, (add_num_1 as i64 as u32) as i32);
              emit_int_arith(function, block_idx, index, replacement_cmd, a, b);

              if !LuauCodegenSubstituteReplacements.get() {
                return;
              }
            }
          } else if let Some(inst_1) = add_inst_1
            && inst_1.cmd == IrCmd::UintToNum
            && let Some(add_num_2) = add_num_2
            && safe_integer_constant(add_num_2)
          {
            let a = op_a_ref(inst_1);
            let b = function.const_int(map, (add_num_2 as i64 as u32) as i32);
            emit_int_arith(function, block_idx, index, replacement_cmd, a, b);

            if !LuauCodegenSubstituteReplacements.get() {
              return;
            }
          }
        }
      }

      state.substitute_or_record(cur_mut(function, index), index);
    }
    IrCmd::TruncateUint => {
      let cur_a = op_a(cur_mut(function, index));
      let dirty = function
        .as_inst_op_ref(cur_a)
        .is_some_and(|src| produces_dirty_high_register_bits(src.cmd));
      if !dirty {
        let source = op_a(cur_mut(function, index));
        substitute_at(function, index, source);
      } else {
        state.substitute_or_record(cur_mut(function, index), index);
      }
    }
    IrCmd::FloatToNum => {
      state.substitute_or_record(cur_mut(function, index), index);
    }
    IrCmd::NumToFloat => {
      // 借用在 match 前结束（cmd 与 op_a 均为 Copy）
      let cur_a = op_a(cur_mut(function, index));
      match function
        .as_inst_op_ref(cur_a)
        .map(|src_inst| (src_inst.cmd, op_a_ref(src_inst)))
      {
        Some((IrCmd::FloatToNum, source)) => substitute_at(function, index, source),
        Some((IrCmd::UintToNum, src_source)) => {
          replace_ir_function_ir_block_u32_ir_inst(
            function,
            block_idx,
            index,
            const_prop_make_inst(IrCmd::UintToFloat, &[src_source]),
          );

          if LuauCodegenSubstituteReplacements.get() {
            state.substitute_or_record(cur_mut(function, index), index);
          }
        }
        _ => state.substitute_or_record(cur_mut(function, index), index),
      }
    }
    IrCmd::JumpIfTruthy => {
      let tag = state.try_get_tag(op_a(cur_mut(function, index)));
      if tag != K_UNKNOWN_TAG {
        if tag == LuaType::Nil as u8 {
          replace_with_jump(
            function,
            block_idx,
            index,
            op_c_ref(cur_ref(function, index)),
          );
        } else if tag != LuaType::Boolean as u8 {
          replace_with_jump(
            function,
            block_idx,
            index,
            op_b_ref(cur_ref(function, index)),
          );
        }
      }
    }
    IrCmd::JumpIfFalsy => {
      let tag = state.try_get_tag(op_a(cur_mut(function, index)));
      if tag != K_UNKNOWN_TAG {
        if tag == LuaType::Nil as u8 {
          replace_with_jump(
            function,
            block_idx,
            index,
            op_b_ref(cur_ref(function, index)),
          );
        } else if tag != LuaType::Boolean as u8 {
          replace_with_jump(
            function,
            block_idx,
            index,
            op_c_ref(cur_ref(function, index)),
          );
        }
      }
    }
    IrCmd::CmpAny => {
      state.invalidate_user_call();
    }
    IrCmd::CmpSplitTvalue => {
      let tag_a_op = op_a(cur_mut(function, index));
      let tag_b_op = op_b_ref(cur_ref(function, index));
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
        && let Some((source, tag)) = type_name_tag_comparison(
          function,
          op_c_ref(cur_ref(function, index)),
          op_d_ref(cur_ref(function, index)),
        )
      {
        let tag_op = function.const_tag(map, tag);
        let replacement = const_prop_make_inst(
          IrCmd::CmpTag,
          &[source, tag_op, op_e_ref(cur_ref(function, index))],
        );

        replace_ir_function_ir_block_u32_ir_inst(function, block_idx, index, replacement);
        fold_constants(function, map, block_idx, index);
      }
    }
    IrCmd::JumpEqPointer => {
      let cur_a = op_a(cur_mut(function, index));
      let cur_b = op_b_ref(cur_ref(function, index));
      if let Some((source, tag)) = type_name_tag_comparison(function, cur_a, cur_b) {
        let tag_op = function.const_tag(map, tag);
        let replacement = const_prop_make_inst(
          IrCmd::JumpEqTag,
          &[
            source,
            tag_op,
            op_c_ref(cur_ref(function, index)),
            op_d_ref(cur_ref(function, index)),
          ],
        );

        replace_ir_function_ir_block_u32_ir_inst(function, block_idx, index, replacement);
        fold_constants(function, map, block_idx, index);
      }
    }
    IrCmd::JumpEqTag => {
      let a = op_a(cur_mut(function, index));
      let b = op_b_ref(cur_ref(function, index));
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

      if tag_a != K_UNKNOWN_TAG && tag_b != K_UNKNOWN_TAG {
        let target = if tag_a == tag_b {
          op_c_ref(cur_ref(function, index))
        } else {
          op_d_ref(cur_ref(function, index))
        };
        replace_with_jump(function, block_idx, index, target);
      } else if a == b {
        replace_with_jump(
          function,
          block_idx,
          index,
          op_c_ref(cur_ref(function, index)),
        );
      }
    }
    IrCmd::JumpCmpInt => {
      let a = op_a(cur_mut(function, index));
      let b = op_b_ref(cur_ref(function, index));
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
        let target = if compare_int(
          value_a,
          value_b,
          condition_op(op_c_ref(cur_ref(function, index))),
        ) {
          op_d_ref(cur_ref(function, index))
        } else {
          op_e_ref(cur_ref(function, index))
        };
        replace_with_jump(function, block_idx, index, target);
      }
    }
    IrCmd::JumpCmpInt64 => {
      let a = op_a(cur_mut(function, index));
      let b = op_b_ref(cur_ref(function, index));
      let value_a = function.as_int_64_op(if a.kind() == IrOpKind::Constant {
        a
      } else {
        state.try_get_value(a)
      });
      let value_b = function.as_int_64_op(if b.kind() == IrOpKind::Constant {
        b
      } else {
        state.try_get_value(b)
      });

      if let (Some(value_a), Some(value_b)) = (value_a, value_b) {
        let target = if compare_int(
          value_a,
          value_b,
          condition_op(op_c_ref(cur_ref(function, index))),
        ) {
          op_d_ref(cur_ref(function, index))
        } else {
          op_e_ref(cur_ref(function, index))
        };
        replace_with_jump(function, block_idx, index, target);
      }
    }
    IrCmd::JumpCmpNum => {
      let a = op_a(cur_mut(function, index));
      let b = op_b_ref(cur_ref(function, index));
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
        let target = if compare_f64_f64_ir_condition(
          value_a,
          value_b,
          condition_op(op_c_ref(cur_ref(function, index))),
        ) {
          op_d_ref(cur_ref(function, index))
        } else {
          op_e_ref(cur_ref(function, index))
        };
        replace_with_jump(function, block_idx, index, target);
      }
    }
    IrCmd::CheckTag => {
      let target = op_a(cur_mut(function, index));
      let expected = function.tag_op(op_b_ref(cur_ref(function, index)));
      let mut tag = state.try_get_tag(target);

      if tag == K_UNKNOWN_TAG {
        let value = state.try_get_value(target);
        if value.kind() == IrOpKind::Constant {
          let constant = function.const_op(value);
          if matches!(constant, IrConst::Double(_)) {
            tag = LuaType::Number as u8;
          } else if matches!(constant, IrConst::Int64(_)) {
            tag = LuaType::Integer as u8;
          }
        }
      }

      if tag != K_UNKNOWN_TAG {
        if tag == expected {
          kill_ir_function_ir_inst_at(function, index);
        } else {
          replace_with_jump(
            function,
            block_idx,
            index,
            op_c_ref(cur_ref(function, index)),
          );
        }
      } else {
        if let Some(lhs) = function.as_inst_op_ref(target) {
          let mut lhs_inst = lhs.clone();
          let lhs_source = op_a(&mut lhs_inst);
          if lhs_inst.cmd == IrCmd::LoadTag
            && lhs_source.kind() == IrOpKind::VmReg
            && let Some(prev_idx) =
              state.get_previous_versioned_load_index(IrCmd::LoadTvalue, lhs_source)
          {
            state.inst_tag.try_insert(prev_idx, expected);
          }
        }

        state.update_tag(target, expected);
      }
    }
    IrCmd::NumToInt64 => {
      // 同一源上的 INT64_TO_NUM 后接 NUM_TO_INT64 即恒等变换。
      let cur_a = op_a(cur_mut(function, index));
      if let Some(src) = function.as_inst_op_ref(cur_a) {
        let src_clone = src.clone();
        let src_cmd = src_clone.cmd;
        if src_cmd == IrCmd::Int64ToNum {
          let src_op_a = op_a(&mut { src_clone });
          substitute_at(function, index, src_op_a);
          return;
        }
        if src_cmd == IrCmd::AddNum {
          let src_op_b = op_b_ref(&src_clone);
          if let Some(arg) = function.as_double_op(src_op_b)
            && arg == 0.0
          {
            let src_op_a = op_a_ref(&src_clone);
            function.instructions[index as usize].ops[0] = src_op_a;
            state.substitute_or_record(cur_mut(function, index), index);
            return;
          }
          let src_op_a = op_a_ref(&src_clone);
          if let Some(arg) = function.as_double_op(src_op_a)
            && arg == 0.0
          {
            let src_op_b = op_b(src_clone);
            function.instructions[index as usize].ops[0] = src_op_b;
            state.substitute_or_record(cur_mut(function, index), index);
            return;
          }
        }
      }
      state.substitute_or_record(cur_mut(function, index), index);
    }
    IrCmd::LoadEnv => {
      // cpp 无条件复用首个 LOAD_ENV（OptimizeConstProp.cpp:2594）
      {
        if state.load_env_idx != K_INVALID_INST_IDX {
          substitute_at(
            function,
            index,
            IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, state.load_env_idx),
          );
        } else {
          state.load_env_idx = index;
        }
      }
    }
    IrCmd::GetSlotNodeAddr => {
      // 单次遍历找首个操作数匹配的复用候选（cpp 同样在首个匹配处处理并返回）
      let matched = state.get_slot_node_cache.iter().position(|entry| {
        let prev = &function.instructions[entry.inst_idx as usize];
        op_a_ref(prev) == op_a_ref(cur_ref(function, index))
          && op_c_ref(prev) == op_c_ref(cur_ref(function, index))
      });

      if let Some(i) = matched {
        // 检查该复用是否会推高寄存器压力越过上限
        let limit = LuauCodeGenLiveSlotReuseLimit.get();

        if state.get_slot_node_cache.len() as i32 > limit {
          // cpp 直接传原缓存；借用规则下改为传只读快照
          let cache = state.get_slot_node_cache.clone();
          if state.get_max_internal_overlap(&cache, i) > limit {
            return;
          }
        }

        // 更新该值从优化视角看的活跃区间终点
        state.get_slot_node_cache[i].finish_pos = state.inst_pos;

        substitute_at(
          function,
          index,
          IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, state.get_slot_node_cache[i].inst_idx),
        );
        return;
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
        // 只读借用：NLL 保证 prev 在 substitute_at 前结束；op_a_ref 无 resize 副作用
        let prev = &function.instructions[prev_idx as usize];

        if op_a_ref(prev) == op_a_ref(cur_ref(function, index))
          && op_b_ref(prev) == op_b_ref(cur_ref(function, index))
        {
          substitute_at(
            function,
            index,
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
      state.substitute_or_record(cur_mut(function, index), index);
    }
    IrCmd::TryNumToIndex => {
      for prev_idx in state.try_num_to_index_cache.iter().copied() {
        // 只读借用：NLL 保证 prev 在 substitute_at 前结束；op_a_ref 无 resize 副作用
        let prev = &function.instructions[prev_idx as usize];

        if op_a_ref(prev) == op_a_ref(cur_ref(function, index)) {
          substitute_at(
            function,
            index,
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
      // 当前指令操作数快照（IrOp Copy）：循环内 prev 共享借用与写回互不重叠
      let cur_a = op_a(cur_mut(function, index));
      let cur_b = op_b_ref(cur_ref(function, index));
      let cur_c = op_c_ref(cur_ref(function, index));
      for el in &mut state.check_slot_match_cache {
        // 只读借用：NLL 保证 prev 在 kill/replace 前结束；op_a_ref 无 resize 副作用
        let prev = &function.instructions[el.pointer as usize];

        if op_a_ref(prev) == cur_a && op_b_ref(prev) == cur_b {
          if let Some(info) = state.inst_tag.find(&cur_a.index())
            && *info != LuaType::Nil as u8
          {
            el.known_to_not_be_nil = true;
          }

          if el.known_to_not_be_nil {
            kill_ir_function_ir_inst_at(function, index);
          } else {
            replace_ir_function_ir_block_u32_ir_inst(
              function,
              block_idx,
              index,
              const_prop_make_inst(IrCmd::CheckNodeValue, &[cur_a, cur_c]),
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
        kill_ir_function_ir_inst_at(function, index);
      } else {
        state.in_safe_env = true;
      }
    }
    IrCmd::CheckReadonly => {
      // cpp 无条件按指令索引跟踪已知只读表（OptimizeConstProp.cpp:2298）
      let target = op_a(cur_mut(function, index));
      if target.kind() == IrOpKind::Inst {
        let target_idx = target.index();
        if state.inst_not_readonly.find(&target_idx).is_some() {
          if DebugLuauAbortingChecks.get() {
            let replacement = IrOp::ir_op_ir_op_kind_u32(IrOpKind::Undef, 0);
            replace_ir_function_ir_op_ir_op_at(function, index, 1, replacement);
          } else {
            kill_ir_function_ir_inst_at(function, index);
          }
        } else {
          state.inst_not_readonly.insert(target_idx);
        }
      }
    }
    IrCmd::CheckNoMetatable => {
      // cpp 无条件按指令索引跟踪已知无 metatable（OptimizeConstProp.cpp:2318）
      let target = op_a(cur_mut(function, index));
      if target.kind() == IrOpKind::Inst {
        let target_idx = target.index();
        if state.inst_no_metatable.find(&target_idx).is_some() {
          if DebugLuauAbortingChecks.get() {
            let replacement = IrOp::ir_op_ir_op_kind_u32(IrOpKind::Undef, 0);
            replace_ir_function_ir_op_ir_op_at(function, index, 1, replacement);
          } else {
            kill_ir_function_ir_inst_at(function, index);
          }
        } else {
          state.inst_no_metatable.insert(target_idx);
        }
      }
    }
    IrCmd::BufferReadi8 => {
      state.substitute_or_record_buffer_load(block_idx, index, cur_mut(function, index), 1);
    }
    IrCmd::BufferReadu8 => {
      state.substitute_or_record_buffer_load(block_idx, index, cur_mut(function, index), 1);
    }
    IrCmd::BufferWritei8 => forward_buffer_int_store(
      state,
      function,
      index,
      |i, b| i.cmd == IrCmd::Sexti8Int || (i.cmd == IrCmd::BitandUint && b == Some(0xff)),
      IrCmd::BufferReadi8,
      1,
    ),
    IrCmd::BufferReadi16 => {
      state.substitute_or_record_buffer_load(block_idx, index, cur_mut(function, index), 2);
    }
    IrCmd::BufferReadu16 => {
      state.substitute_or_record_buffer_load(block_idx, index, cur_mut(function, index), 2);
    }
    IrCmd::BufferWritei16 => forward_buffer_int_store(
      state,
      function,
      index,
      |i, b| i.cmd == IrCmd::Sexti16Int || (i.cmd == IrCmd::BitandUint && b == Some(0xffff)),
      IrCmd::BufferReadi16,
      2,
    ),
    IrCmd::BufferReadi32 => {
      state.substitute_or_record_buffer_load(block_idx, index, cur_mut(function, index), 4);
    }
    IrCmd::BufferWritei32 => forward_buffer_int_store(
      state,
      function,
      index,
      |i, _| i.cmd == IrCmd::TruncateUint,
      IrCmd::BufferReadi32,
      4,
    ),
    IrCmd::BufferReadf32 => {
      state.substitute_or_record_buffer_load(block_idx, index, cur_mut(function, index), 4);
    }
    IrCmd::BufferWritef32 => {
      state.forward_buffer_store_to_load(cur_mut(function, index), IrCmd::BufferReadf32, 4);
    }
    IrCmd::BufferReadf64 => {
      state.substitute_or_record_buffer_load(block_idx, index, cur_mut(function, index), 8);
    }
    IrCmd::BufferWritef64 => {
      state.forward_buffer_store_to_load(cur_mut(function, index), IrCmd::BufferReadf64, 8);
    }
    IrCmd::BufferReadi64 => {
      state.substitute_or_record_buffer_load(block_idx, index, cur_mut(function, index), 8);
    }
    IrCmd::BufferWritei64 => {
      state.forward_buffer_store_to_load(cur_mut(function, index), IrCmd::BufferReadi64, 8);
    }
    IrCmd::CheckGc => {
      if state.checked_gc {
        kill_ir_function_ir_inst_at(function, index);
      } else {
        state.checked_gc = true;
        state.invalidate_heap_table_data();
      }
    }
    IrCmd::BarrierObj | IrCmd::BarrierTableForward => {
      let value = op_b_ref(cur_ref(function, index));
      if value.kind() == IrOpKind::VmReg {
        let tag = state.try_get_tag(value);
        if tag != K_UNKNOWN_TAG && !is_gco(tag) {
          kill_ir_function_ir_inst_at(function, index);
        }
      }
    }
    IrCmd::NewTable => {
      // cpp 无条件记录新表的只读/metatable/数组大小状态（OptimizeConstProp.cpp:2925）
      let cur_a = op_a(cur_mut(function, index));
      let array_size = function.uint_op(cur_a) as i32;
      state.inst_not_readonly.insert(index);
      state.inst_no_metatable.insert(index);
      state.inst_array_size.try_insert(index, array_size);
    }
    IrCmd::CheckArraySize => {
      let target = op_a(cur_mut(function, index));
      let boundary = op_b_ref(cur_ref(function, index));
      let boundary_value = if boundary.kind() == IrOpKind::Constant {
        function.as_int_op(boundary)
      } else {
        function.as_int_op(state.try_get_value(boundary))
      };

      if let Some(array_index) = boundary_value {
        if array_index < 0 {
          replace_with_jump(
            function,
            block_idx,
            index,
            op_c_ref(cur_ref(function, index)),
          );
          return;
        }

        // cpp 无条件用已知数组大小做越界消除（OptimizeConstProp.cpp:3238）
        if target.kind() == IrOpKind::Inst
          && let Some(known_array_size) = state.inst_array_size.find(&target.index())
          && *known_array_size >= 0
        {
          if (array_index as u32) < (*known_array_size as u32) {
            if DebugLuauAbortingChecks.get() {
              let replacement = IrOp::ir_op_ir_op_kind_u32(IrOpKind::Undef, 0);
              replace_ir_function_ir_op_ir_op_at(function, index, 2, replacement);
            } else {
              kill_ir_function_ir_inst_at(function, index);
            }
          } else {
            replace_with_jump(
              function,
              block_idx,
              index,
              op_c_ref(cur_ref(function, index)),
            );
          }
          return;
        }
      }

      for prev_idx in state.check_array_size_cache.iter().copied() {
        // 只读借用：NLL 保证 prev 在 kill 前结束；op_a_ref 无 resize 副作用
        let prev = &function.instructions[prev_idx as usize];

        if op_a_ref(prev) != op_a_ref(cur_ref(function, index)) {
          continue;
        }

        let prev_boundary = op_b_ref(prev);
        let boundary = op_b_ref(cur_ref(function, index));
        let mut same_boundary = prev_boundary == boundary;

        if !same_boundary
          && boundary.kind() == IrOpKind::Constant
          && prev_boundary.kind() == IrOpKind::Constant
          && (function.int_op(boundary) as u32) < (function.int_op(prev_boundary) as u32)
        {
          same_boundary = true;
        }

        if same_boundary {
          kill_ir_function_ir_inst_at(function, index);
          return;
        }
      }

      if (state.check_array_size_cache.len() as i32) < LuauCodeGenReuseSlotLimit.get() {
        state.check_array_size_cache.push(index);
      }
    }
    IrCmd::CheckBufferLen => {
      let buffer_offset_op = op_b_ref(cur_ref(function, index));
      let buffer_offset = if buffer_offset_op.kind() == IrOpKind::Constant {
        function.as_int_op(buffer_offset_op)
      } else {
        function.as_int_op(state.try_get_value(buffer_offset_op))
      };

      let min_offset = function.int_op(op_c_ref(cur_ref(function, index)));
      let max_offset = function.int_op(op_d_ref(cur_ref(function, index)));
      CODEGEN_ASSERT!(min_offset < max_offset);
      let access_size = max_offset - min_offset;
      CODEGEN_ASSERT!(access_size > 0);

      if let Some(buffer_offset) = buffer_offset
        && (buffer_offset < 0
          || (buffer_offset as u32).wrapping_add(access_size as u32) >= i32::MAX as u32)
      {
        replace_with_jump(
          function,
          block_idx,
          index,
          op_f_ref(cur_ref(function, index)),
        );
        return;
      }

      for prev_idx in state.check_buffer_len_cache.clone() {
        if function.instructions[prev_idx as usize].cmd != IrCmd::CheckBufferLen {
          continue;
        }

        // 只读判定与操作数快照（IrOp 全 Copy）：共享借用于语句内结束，
        // 随后的可变访问按索引重新定位 prev，无需裸指针绕过借用
        let four_ops_match = {
          let prev = &function.instructions[prev_idx as usize];

          op_a_ref(prev) == op_a_ref(cur_ref(function, index))
            && op_b_ref(prev) == op_b_ref(cur_ref(function, index))
            && op_c_ref(prev) == op_c_ref(cur_ref(function, index))
            && op_d_ref(prev) == op_d_ref(cur_ref(function, index))
        };

        if four_ops_match {
          if DebugLuauAbortingChecks.get() {
            let replacement = IrOp::ir_op_ir_op_kind_u32(IrOpKind::Undef, 0);
            replace_ir_function_ir_op_ir_op_at(function, index, 5, replacement);
          } else {
            kill_ir_function_ir_inst_at(function, index);
          }
          return;
        }

        let (a_matches, prev_bound_is_const, prev_bound_op) = {
          let prev = &function.instructions[prev_idx as usize];
          let prev_bound = op_b_ref(prev);

          (
            op_a_ref(prev) == op_a_ref(cur_ref(function, index)),
            prev_bound.kind() == IrOpKind::Constant,
            prev_bound,
          )
        };

        if a_matches
          && op_b_ref(cur_ref(function, index)).kind() == IrOpKind::Constant
          && prev_bound_is_const
        {
          let curr_bound = function.int_op(op_b_ref(cur_ref(function, index)));
          let prev_bound = function.int_op(prev_bound_op);
          CODEGEN_ASSERT!(curr_bound >= 0);
          CODEGEN_ASSERT!(prev_bound >= 0);

          let extra_offset = curr_bound - prev_bound;
          let prev = &mut function.instructions[prev_idx as usize];
          if state.try_merge_and_kill_buffer_length_check(index, prev, extra_offset) {
            return;
          }

          continue;
        }

        let prev = &mut function.instructions[prev_idx as usize];
        if state.try_merge_buffer_range_check(block_idx, index, prev) {
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
      // TagVector(x) 参与向量运算时直接替换为 x；借用（IrOp Copy）在 replace 前结束
      let cur_a = op_a(cur_mut(function, index));
      let a_replacement = tag_vector_source(function, cur_a);
      if let Some(replacement) = a_replacement {
        replace_ir_function_ir_op_ir_op_at(function, index, 0, replacement);
      }

      let b_replacement = tag_vector_source(function, op_b_ref(cur_ref(function, index)));
      if let Some(replacement) = b_replacement {
        replace_ir_function_ir_op_ir_op_at(function, index, 1, replacement);
      }

      state.substitute_or_record(cur_mut(function, index), index);
    }
    IrCmd::UnmVec | IrCmd::FloorVec | IrCmd::CeilVec | IrCmd::AbsVec => {
      let cur_a = op_a(cur_mut(function, index));
      let a_replacement = tag_vector_source(function, cur_a);
      if let Some(replacement) = a_replacement {
        replace_ir_function_ir_op_ir_op_at(function, index, 0, replacement);
      }

      state.substitute_or_record(cur_mut(function, index), index);
    }
    IrCmd::FloatToVec | IrCmd::TagVector => {
      state.substitute_or_record(cur_mut(function, index), index);
    }
    IrCmd::InvokeLibm => {
      state.substitute_or_record(cur_mut(function, index), index);
    }
    IrCmd::DoArith => {
      let target = op_a(cur_mut(function, index));
      state.invalidate_ir_op(target);
      state.invalidate_user_call();
    }
    IrCmd::DoLen => {
      let target = op_a(cur_mut(function, index));
      state.invalidate_ir_op(target);
      state.invalidate_user_call();
      state.save_tag(target, LuaType::Number as u8);
    }
    IrCmd::GetTable => {
      let target = op_a(cur_mut(function, index));
      state.invalidate_ir_op(target);
      state.invalidate_user_call();
    }
    IrCmd::SetTable => {
      state.invalidate_user_call();
    }
    IrCmd::GetCachedImport => {
      let target = op_a(cur_mut(function, index));
      state.invalidate_ir_op(target);

      if state.in_safe_env {
        state.invalidate_value_propagation();
      } else {
        state.invalidate_user_call();
      }
    }
    IrCmd::SETLIST => {
      // cpp OptimizeConstProp.cpp:3428-3434：经由 B 寄存器缓存的指针 load
      // 查询 instArraySize（移植期开关 LuauCodegenExtraTableOpts 在 cpp 中已删除，
      // 旧的 RegisterInfo.knownTableArraySize 旁路不复存在）。
      if let Some(load_idx) = state
        .get_previous_versioned_load_index(IrCmd::LoadPointer, op_b_ref(cur_ref(function, index)))
        && let Some(known_array_size) = state.inst_array_size.find(&load_idx)
        && *known_array_size >= 0
      {
        let replacement = function.const_uint(map, *known_array_size as u32);
        replace_ir_function_ir_op_ir_op_at(function, index, 5, replacement);
      }

      state.invalidate_value_propagation();
      state.invalidate_heap_table_data();
      state.invalidate_heap_buffer_data();
    }
    IrCmd::TableSetnum => {
      state.invalidate_table_array_size();
    }
    IrCmd::CONCAT => {
      let first_reg = vm_reg_op(op_a(cur_mut(function, index)));
      let count = function.uint_op(op_b_ref(cur_ref(function, index))) as i32;
      state.invalidate_register_range(first_reg, count);
      state.invalidate_user_call();
    }
    IrCmd::FallbackGetvarargs => {
      let first_reg = vm_reg_op(op_b_ref(cur_ref(function, index)));
      let count = function.int_op(op_c_ref(cur_ref(function, index)));
      state.invalidate_register_range(first_reg, count);
    }
    IrCmd::FASTCALL => {
      // bfid 校验后再转换：`as u8` 截断 + 非法判别值 transmute 是 UB；
      // 非法 id 归入 LBF_NONE（handle_builtin_effects 该臂为空操作），与 C++ switch default 一致
      let bfid = {
        let cur_a = op_a(cur_mut(function, index));
        LuauBuiltinFunction::from_id(function.uint_op(cur_a) as i32)
      }
      .unwrap_or(LuauBuiltinFunction::LBF_NONE);
      let first_return_reg = vm_reg_op(op_b_ref(cur_ref(function, index)));
      let nresults = function.int_op(op_d_ref(cur_ref(function, index)));

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
      // 同 FASTCALL：校验后再转换，非法 id 归入 LBF_NONE
      let bfid = {
        let cur_a = op_a(cur_mut(function, index));
        LuauBuiltinFunction::from_id(function.uint_op(cur_a) as i32)
      }
      .unwrap_or(LuauBuiltinFunction::LBF_NONE);
      let first_return_reg = vm_reg_op(op_b_ref(cur_ref(function, index))) as u32;
      let nresults = function.int_op(op_g_ref(cur_ref(function, index)));
      handle_builtin_effects(state, bfid, first_return_reg, nresults);
    }
    IrCmd::CALL => {
      let first_reg = vm_reg_op(op_a(cur_mut(function, index)));
      state.invalidate_registers_from(first_reg);
      state.invalidate_user_call();
    }
    IrCmd::FallbackGetglobal => {
      state.invalidate_ir_op(op_b_ref(cur_ref(function, index)));
      state.invalidate_user_call();
    }
    IrCmd::FallbackSetglobal | IrCmd::FallbackSettableks => {
      state.invalidate_user_call();
    }
    IrCmd::FallbackGettableks => {
      state.invalidate_ir_op(op_b_ref(cur_ref(function, index)));
      state.invalidate_user_call();
    }
    IrCmd::FallbackNamecall => {
      let target = op_b_ref(cur_ref(function, index));
      state.invalidate_ir_op(target);
      state.invalidate_ir_op(IrOp::ir_op_ir_op_kind_u32(
        target.kind(),
        target.index() + 1,
      ));
      state.invalidate_user_call();
    }
    IrCmd::FallbackPrepvarargs => {}
    IrCmd::FallbackDupclosure => {
      state.invalidate_ir_op(op_b_ref(cur_ref(function, index)));
      state.invalidate_heap_table_data();
    }
    IrCmd::FallbackForgprep => {
      let target = op_b_ref(cur_ref(function, index));
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
    // cpp CodeGen/src/OptimizeConstProp.cpp:2411-2433 case IrCmd::CHECK_USERDATA_TAG
    IrCmd::CheckUserdataTag => {
      for prev_idx in state.useradata_tag_cache.iter().copied() {
        // 只读借用：clone 纯浪费（op_a_ref 只读），NLL 保证在 kill/replace 前结束
        let prev = &function.instructions[prev_idx as usize];

        // 缓存里只会有 CHECK_USERDATA_TAG 与 NEW_USERDATA 两种指令
        match prev.cmd {
          IrCmd::CheckUserdataTag => {
            if op_a_ref(prev) != op_a_ref(cur_ref(function, index)) || op_b_ref(prev) != op_b_ref(cur_ref(function, index)) {
              continue;
            }
          }
          IrCmd::NewUserdata
            // 对 NEW_USERDATA 结果做同 tag 检查是冗余的
            if op_a_ref(cur_ref(function, index)).kind() == IrOpKind::Inst
              && prev_idx == op_a_ref(cur_ref(function, index)).index()
              && op_b_ref(prev) == op_b_ref(cur_ref(function, index)) => {}
          _ => continue,
        }

        if DebugLuauAbortingChecks.get() {
          let replacement = IrOp::ir_op_ir_op_kind_u32(IrOpKind::Undef, 0);
          replace_ir_function_ir_op_ir_op_at(function, index, 2, replacement);
        } else {
          kill_ir_function_ir_inst_at(function, index);
        }
        return; // 同时跳出循环与 switch
      }

      if (state.useradata_tag_cache.len() as i32) < LuauCodeGenReuseUdataTagLimit.get() {
        state.useradata_tag_cache.push(index);
      }
    }
    // cpp CodeGen/src/OptimizeConstProp.cpp:2962-2966 case IrCmd::NEW_USERDATA
    IrCmd::NewUserdata
      if (state.useradata_tag_cache.len() as i32) < LuauCodeGenReuseUdataTagLimit.get() =>
    {
      state.useradata_tag_cache.push(index);
    }
    _ => {}
  }
}
