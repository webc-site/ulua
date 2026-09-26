use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::replace_ir_utils::replace_ir_function_ir_op_ir_op_at,
  macros::{codegen_assert::CODEGEN_ASSERT, ir_operand::{HAS_OP_D, HAS_OP_E}},
  records::{
    ir_function::IrFunction,
    ir_op::IrOp,
    remove_dead_store_state::{
      RemoveDeadStoreState, kill_t_value_store_core, kill_tag_and_value_store_pair_core,
    },
  },
};

// 确保 inst_index 处的 STORE_VECTOR 有 tag 操作数（OP_E），并用 prev_tag_op 替换它
fn store_vector_set_tag(function: &mut IrFunction, inst_index: u32, prev_tag_op: IrOp) {
  CODEGEN_ASSERT!(function.instructions[inst_index as usize].cmd == IrCmd::StoreVector);

  if !HAS_OP_E!(function.instructions[inst_index as usize]) {
    function.instructions[inst_index as usize]
      .ops
      .push_back(IrOp::default());
  }

  // 目标槽位改走索引化替换变体（其注释保证与 get_op_mut + 指针版逐操作 1:1），
  // 消除“&mut function 与 function 内指令槽位”双借用引入的裸指针
  replace_ir_function_ir_op_ir_op_at(function, inst_index, 4, prev_tag_op);
}

/// 目标寄存器槽位改以索引传入，函数体内借用 `state.info` 槽位（见 tag 变体注释）。
pub fn try_replace_vector_value_with_full_store(
  state: &mut RemoveDeadStoreState,
  function: &mut IrFunction,
  inst_index: u32,
  reg_idx: usize,
) -> bool {
  let reg_info = &mut state.info[reg_idx];
  // 若 tag+value 对已确立，可将两者标记为 dead，改用单条拆分 TValue store
  if reg_info.tag_inst_idx != !0u32 && reg_info.value_inst_idx != !0u32 {
    let prev_tag_op = function.instructions[reg_info.tag_inst_idx as usize].ops[1];
    let prev_tag = function.tag_op(prev_tag_op);

    CODEGEN_ASSERT!(reg_info.known_tag == prev_tag);

    store_vector_set_tag(function, inst_index, prev_tag_op);

    // 同 try_replace_tag_with_full_store：调用点 pair 已建立，用带 restore hint 的 pair kill 版本
    kill_tag_and_value_store_pair_core(function, reg_info);

    reg_info.tvalue_inst_idx = inst_index;
    return true;
  }

  // 也可用新 store 替换已 dead 的拆分 TValue store，同时保持 value 不变
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
      kill_t_value_store_core(function, reg_info);

      reg_info.tvalue_inst_idx = inst_index;
      return true;
    } else if prev_cmd == IrCmd::StoreVector {
      let prev_tag_op = function.instructions[reg_info.tvalue_inst_idx as usize].ops[4];
      CODEGEN_ASSERT!(prev_tag_op.kind() != IrOpKind::None);
      let prev_tag = function.tag_op(prev_tag_op);

      CODEGEN_ASSERT!(reg_info.known_tag == prev_tag);

      store_vector_set_tag(function, inst_index, prev_tag_op);

      CODEGEN_ASSERT!(reg_info.tag_inst_idx == !0u32 && reg_info.value_inst_idx == !0u32);
      kill_t_value_store_core(function, reg_info);

      reg_info.tvalue_inst_idx = inst_index;
      return true;
    }
  }

  false
}
