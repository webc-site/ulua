use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::replace_ir_utils::replace_ir_function_ir_block_u32_ir_inst,
  macros::{codegen_assert::CODEGEN_ASSERT, has_op_d::HAS_OP_D},
  records::{
    ir_builder::ConstantMap,
    ir_function::IrFunction,
    ir_inst::IrInst,
    ir_op::IrOp,
    remove_dead_store_state::{
      RemoveDeadStoreState, kill_t_value_store_core, kill_tag_and_value_store_pair_core,
    },
    store_reg_info::StoreRegInfo,
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

/// tvalue 侧三分支（StoreSplitTvalue/StoreVector/StoreTvalue 常量 tag）共用的提交尾：
/// 原位替换为拆分 store、kill 旧记录并把槽位指向新指令。依赖旧指令仍在位的
/// 一致性断言（known_tag 一致、HAS_OP_D）由各调用点先行完成。
fn commit_split(
  function: &mut IrFunction,
  block_idx: u32,
  inst_index: u32,
  repl: IrInst,
  reg_info: &mut StoreRegInfo,
) -> bool {
  replace_ir_function_ir_block_u32_ir_inst(function, block_idx, inst_index, repl);

  CODEGEN_ASSERT!(reg_info.tag_inst_idx == !0u32 && reg_info.value_inst_idx == !0u32);
  kill_t_value_store_core(function, reg_info);

  reg_info.tvalue_inst_idx = inst_index;
  true
}

/// 目标寄存器槽位改以索引传入，函数体内借用 `state.info` 槽位（见 tag 变体注释）。
pub fn try_replace_value_with_full_store(
  state: &mut RemoveDeadStoreState,
  constant_map: &mut ConstantMap,
  function: &mut IrFunction,
  block_idx: u32,
  inst_index: u32,
  target_op: IrOp,
  value_op: IrOp,
  reg_idx: usize,
) -> bool {
  let reg_info = &mut state.info[reg_idx];

  // 若 tag+value 对已确立，可将两者标记为 dead，改用单条拆分 TValue store
  if reg_info.tag_inst_idx != !0u32 && reg_info.value_inst_idx != !0u32 {
    let prev_tag_op = function.instructions[reg_info.tag_inst_idx as usize].ops[1];
    let prev_tag = function.tag_op(prev_tag_op);

    CODEGEN_ASSERT!(reg_info.known_tag == prev_tag);
    let repl = make_split(target_op, prev_tag_op, value_op);
    replace_ir_function_ir_block_u32_ir_inst(function, block_idx, inst_index, repl);

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
      let repl = make_split(target_op, prev_tag_op, value_op);
      return commit_split(function, block_idx, inst_index, repl, reg_info);
    } else if prev_cmd == IrCmd::StoreVector {
      let prev_tag_op = function.instructions[reg_info.tvalue_inst_idx as usize].ops[4];
      CODEGEN_ASSERT!(prev_tag_op.kind() != IrOpKind::None);
      let prev_tag = function.tag_op(prev_tag_op);

      CODEGEN_ASSERT!(reg_info.known_tag == prev_tag);
      let repl = make_split(target_op, prev_tag_op, value_op);
      return commit_split(function, block_idx, inst_index, repl, reg_info);
    } else if prev_cmd == IrCmd::StoreTvalue
      && reg_info.known_tag != 0xff
      && reg_info.tag_inst_idx == !0u32
    {
      let prev_tag_op = function.const_tag(constant_map, reg_info.known_tag);
      let repl = make_split(target_op, prev_tag_op, value_op);
      return commit_split(function, block_idx, inst_index, repl, reg_info);
    }
  }

  false
}
