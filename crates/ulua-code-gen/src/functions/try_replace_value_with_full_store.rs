use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::replace_ir_utils_alt_b::replace_ir_function_ir_block_u32_ir_inst,
  macros::{codegen_assert::CODEGEN_ASSERT, has_op_d::HAS_OP_D},
  records::{
    ir_block::IrBlock, ir_builder::IrBuilder, ir_function::IrFunction, ir_inst::IrInst,
    ir_op::IrOp, remove_dead_store_state::RemoveDeadStoreState, store_reg_info::StoreRegInfo,
  },
  type_aliases::ir_ops::IrOps,
};

fn make_split(target_op: IrOp, tag_op: IrOp, value_op: IrOp) -> IrInst {
  let mut ops = IrOps::new();
  ops.push_back(target_op);
  ops.push_back(tag_op);
  ops.push_back(value_op);
  IrInst {
    cmd: IrCmd::StoreSplitTvalue,
    ops,
    ..IrInst::default()
  }
}

pub fn try_replace_value_with_full_store(
  state: &mut RemoveDeadStoreState,
  build: &mut IrBuilder,
  function: &mut IrFunction,
  block: &mut IrBlock,
  inst_index: u32,
  target_op: IrOp,
  value_op: IrOp,
  reg_info: &mut StoreRegInfo,
) -> bool {
  // If the tag+value pair is established, we can mark both as dead and use a single split TValue store
  if reg_info.tag_inst_idx != !0u32 && reg_info.value_inst_idx != !0u32 {
    let prev_tag_op = function.instructions[reg_info.tag_inst_idx as usize].ops[1];
    let prev_tag = function.tag_op(prev_tag_op);

    CODEGEN_ASSERT!(reg_info.known_tag == prev_tag);
    let repl = make_split(target_op, prev_tag_op, value_op);
    replace_ir_function_ir_block_u32_ir_inst(function, block, inst_index, repl);

    state.kill_tag_store(reg_info);
    state.kill_value_store(reg_info);

    reg_info.tvalue_inst_idx = inst_index;
    return true;
  }

  // We can also replace a dead split TValue store with a new one, while keeping the value the same
  if reg_info.tvalue_inst_idx != !0u32 {
    let prev_cmd = function.instructions[reg_info.tvalue_inst_idx as usize].cmd;

    if prev_cmd == IrCmd::StoreSplitTvalue {
      let prev_tag_op = function.instructions[reg_info.tvalue_inst_idx as usize].ops[1];
      let prev_tag = function.tag_op(prev_tag_op);

      CODEGEN_ASSERT!(reg_info.known_tag == prev_tag);
      CODEGEN_ASSERT!(!HAS_OP_D!(
        function.instructions[reg_info.tvalue_inst_idx as usize]
      ));
      let repl = make_split(target_op, prev_tag_op, value_op);
      replace_ir_function_ir_block_u32_ir_inst(function, block, inst_index, repl);

      CODEGEN_ASSERT!(reg_info.tag_inst_idx == !0u32 && reg_info.value_inst_idx == !0u32);
      state.kill_t_value_store(reg_info);

      reg_info.tvalue_inst_idx = inst_index;
      return true;
    } else if prev_cmd == IrCmd::StoreVector {
      let prev_tag_op = function.instructions[reg_info.tvalue_inst_idx as usize].ops[4];
      CODEGEN_ASSERT!(prev_tag_op.kind() != IrOpKind::None);
      let prev_tag = function.tag_op(prev_tag_op);

      CODEGEN_ASSERT!(reg_info.known_tag == prev_tag);
      let repl = make_split(target_op, prev_tag_op, value_op);
      replace_ir_function_ir_block_u32_ir_inst(function, block, inst_index, repl);

      CODEGEN_ASSERT!(reg_info.tag_inst_idx == !0u32 && reg_info.value_inst_idx == !0u32);
      state.kill_t_value_store(reg_info);

      reg_info.tvalue_inst_idx = inst_index;
      return true;
    } else if prev_cmd == IrCmd::StoreTvalue
      && reg_info.known_tag != 0xff
      && reg_info.tag_inst_idx == !0u32
    {
      let prev_tag_op = build.const_tag(reg_info.known_tag);
      let repl = make_split(target_op, prev_tag_op, value_op);
      replace_ir_function_ir_block_u32_ir_inst(function, block, inst_index, repl);

      CODEGEN_ASSERT!(reg_info.tag_inst_idx == !0u32 && reg_info.value_inst_idx == !0u32);
      state.kill_t_value_store(reg_info);

      reg_info.tvalue_inst_idx = inst_index;
      return true;
    }
  }

  false
}
