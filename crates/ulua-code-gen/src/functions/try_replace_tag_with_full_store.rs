use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::ir_cmd::IrCmd,
  functions::{is_gco::is_gco, replace_ir_utils_alt_b::replace_ir_function_ir_block_u32_ir_inst},
  macros::{codegen_assert::CODEGEN_ASSERT, has_op_d::HAS_OP_D, has_op_e::HAS_OP_E},
  records::{
    ir_block::IrBlock, ir_builder::IrBuilder, ir_function::IrFunction, ir_inst::IrInst,
    ir_op::IrOp, remove_dead_store_state::RemoveDeadStoreState, store_reg_info::StoreRegInfo,
  },
  type_aliases::ir_ops::IrOps,
};

fn make_inst(cmd: IrCmd, ops_slice: &[IrOp]) -> IrInst {
  let mut ops = IrOps::new();
  for op in ops_slice {
    ops.push_back(*op);
  }
  IrInst {
    cmd,
    ops,
    ..IrInst::default()
  }
}

pub fn try_replace_tag_with_full_store(
  state: &mut RemoveDeadStoreState,
  _build: &mut IrBuilder,
  function: &mut IrFunction,
  block: &mut IrBlock,
  inst_index: u32,
  target_op: IrOp,
  tag_op: IrOp,
  reg_info: &mut StoreRegInfo,
) -> bool {
  let tag = function.tag_op(tag_op);
  let nil = LuaType::Nil as u8;

  // If the tag+value pair is established, we can mark both as dead and use a single split TValue store
  if reg_info.tag_inst_idx != !0u32
    && (reg_info.value_inst_idx != !0u32 || reg_info.known_tag == nil)
  {
    if tag != nil && reg_info.value_inst_idx != !0u32 {
      let prev_cmd = function.instructions[reg_info.value_inst_idx as usize].cmd;

      if prev_cmd == IrCmd::StoreVector {
        CODEGEN_ASSERT!(!HAS_OP_E!(
          function.instructions[reg_info.value_inst_idx as usize]
        ));
        let prev_value_x = function.instructions[reg_info.value_inst_idx as usize].ops[1];
        let prev_value_y = function.instructions[reg_info.value_inst_idx as usize].ops[2];
        let prev_value_z = function.instructions[reg_info.value_inst_idx as usize].ops[3];
        let repl = make_inst(
          IrCmd::StoreVector,
          &[target_op, prev_value_x, prev_value_y, prev_value_z, tag_op],
        );
        replace_ir_function_ir_block_u32_ir_inst(function, block, inst_index, repl);
      } else {
        let prev_value_op = function.instructions[reg_info.value_inst_idx as usize].ops[1];
        let repl = make_inst(IrCmd::StoreSplitTvalue, &[target_op, tag_op, prev_value_op]);
        replace_ir_function_ir_block_u32_ir_inst(function, block, inst_index, repl);
      }
    }

    state.kill_tag_store(reg_info);
    state.kill_value_store(reg_info);

    reg_info.tvalue_inst_idx = inst_index;
    reg_info.maybe_gco = is_gco(tag);
    reg_info.known_tag = tag;
    state.has_gco_to_clear |= reg_info.maybe_gco;
    return true;
  }

  // We can also replace a dead split TValue store with a new one, while keeping the value the same
  if reg_info.tvalue_inst_idx != !0u32 {
    let prev_cmd = function.instructions[reg_info.tvalue_inst_idx as usize].cmd;

    if prev_cmd == IrCmd::StoreSplitTvalue {
      CODEGEN_ASSERT!(!HAS_OP_D!(
        function.instructions[reg_info.tvalue_inst_idx as usize]
      ));

      // If the 'nil' is stored, we keep 'STORE_TAG Rn, tnil' as it writes the 'full' TValue
      if tag != nil {
        let prev_value_op = function.instructions[reg_info.tvalue_inst_idx as usize].ops[2];
        let repl = make_inst(IrCmd::StoreSplitTvalue, &[target_op, tag_op, prev_value_op]);
        replace_ir_function_ir_block_u32_ir_inst(function, block, inst_index, repl);
      }

      CODEGEN_ASSERT!(reg_info.tag_inst_idx == !0u32 && reg_info.value_inst_idx == !0u32);
      state.kill_t_value_store(reg_info);

      reg_info.tvalue_inst_idx = inst_index;
      reg_info.maybe_gco = is_gco(tag);
      reg_info.known_tag = tag;
      state.has_gco_to_clear |= reg_info.maybe_gco;
      return true;
    } else if prev_cmd == IrCmd::StoreVector {
      if tag != nil {
        let prev_value_x = function.instructions[reg_info.tvalue_inst_idx as usize].ops[1];
        let prev_value_y = function.instructions[reg_info.tvalue_inst_idx as usize].ops[2];
        let prev_value_z = function.instructions[reg_info.tvalue_inst_idx as usize].ops[3];
        let repl = make_inst(
          IrCmd::StoreVector,
          &[target_op, prev_value_x, prev_value_y, prev_value_z, tag_op],
        );
        replace_ir_function_ir_block_u32_ir_inst(function, block, inst_index, repl);
      }

      CODEGEN_ASSERT!(reg_info.tag_inst_idx == !0u32 && reg_info.value_inst_idx == !0u32);
      state.kill_t_value_store(reg_info);

      reg_info.tvalue_inst_idx = inst_index;
      reg_info.maybe_gco = is_gco(tag);
      reg_info.known_tag = tag;
      state.has_gco_to_clear |= reg_info.maybe_gco;
      return true;
    }
  }

  false
}
