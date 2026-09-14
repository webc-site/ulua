use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{get_op_ir_data::get_op_mut, replace_ir_utils::replace_ir_function_ir_op_ir_op},
  macros::{codegen_assert::CODEGEN_ASSERT, has_op_d::HAS_OP_D, has_op_e::HAS_OP_E},
  records::{
    ir_block::IrBlock, ir_builder::IrBuilder, ir_function::IrFunction, ir_op::IrOp,
    remove_dead_store_state::RemoveDeadStoreState, store_reg_info::StoreRegInfo,
  },
};

// Ensures the STORE_VECTOR at inst_index has a tag operand (OP_E) and replaces it with prev_tag_op
fn store_vector_set_tag(function: &mut IrFunction, inst_index: u32, prev_tag_op: IrOp) {
  CODEGEN_ASSERT!(function.instructions[inst_index as usize].cmd == IrCmd::StoreVector);

  if !HAS_OP_E!(function.instructions[inst_index as usize]) {
    function.instructions[inst_index as usize]
      .ops
      .push_back(IrOp::default());
  }

  let op_e_ptr: *mut IrOp =
    get_op_mut(&mut function.instructions[inst_index as usize], 4) as *mut IrOp;
  replace_ir_function_ir_op_ir_op(function, unsafe { &mut *op_e_ptr }, prev_tag_op);
}

pub fn try_replace_vector_value_with_full_store(
  state: &mut RemoveDeadStoreState,
  _build: &mut IrBuilder,
  function: &mut IrFunction,
  _block: &mut IrBlock,
  inst_index: u32,
  reg_info: &mut StoreRegInfo,
) -> bool {
  // If the tag+value pair is established, we can mark both as dead and use a single split TValue store
  if reg_info.tag_inst_idx != !0u32 && reg_info.value_inst_idx != !0u32 {
    let prev_tag_op = function.instructions[reg_info.tag_inst_idx as usize].ops[1];
    let prev_tag = function.tag_op(prev_tag_op);

    CODEGEN_ASSERT!(reg_info.known_tag == prev_tag);

    store_vector_set_tag(function, inst_index, prev_tag_op);

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

      store_vector_set_tag(function, inst_index, prev_tag_op);

      CODEGEN_ASSERT!(reg_info.tag_inst_idx == !0u32 && reg_info.value_inst_idx == !0u32);
      state.kill_t_value_store(reg_info);

      reg_info.tvalue_inst_idx = inst_index;
      return true;
    } else if prev_cmd == IrCmd::StoreVector {
      let prev_tag_op = function.instructions[reg_info.tvalue_inst_idx as usize].ops[4];
      CODEGEN_ASSERT!(prev_tag_op.kind() != IrOpKind::None);
      let prev_tag = function.tag_op(prev_tag_op);

      CODEGEN_ASSERT!(reg_info.known_tag == prev_tag);

      store_vector_set_tag(function, inst_index, prev_tag_op);

      CODEGEN_ASSERT!(reg_info.tag_inst_idx == !0u32 && reg_info.value_inst_idx == !0u32);
      state.kill_t_value_store(reg_info);

      reg_info.tvalue_inst_idx = inst_index;
      return true;
    }
  }

  false
}
