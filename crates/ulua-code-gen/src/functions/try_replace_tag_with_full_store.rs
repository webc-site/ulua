use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::ir_cmd::IrCmd,
  functions::{is_gco::is_gco, replace_ir_utils::replace_ir_function_ir_block_u32_ir_inst},
  macros::{
    codegen_assert::CODEGEN_ASSERT,
    ir_operand::{HAS_OP_D, HAS_OP_E},
  },
  records::{
    ir_function::IrFunction,
    ir_inst::IrInst,
    ir_op::IrOp,
    remove_dead_store_state::{
      RemoveDeadStoreState, kill_t_value_store_core, kill_tag_and_value_store_pair_core,
    },
    store_reg_info::StoreRegInfo,
  },
};

/// StoreVector 重放：从旧 store 取 xyz 三通道，与目标槽位、新 tag 组成五元 repl
/// （两处重放块逐字重复的收拢；原位替换由调用点的 replace 完成）。
fn make_vector_repl(
  function: &IrFunction,
  prev_inst_idx: u32,
  target_op: IrOp,
  tag_op: IrOp,
) -> IrInst {
  let prev_inst_idx = prev_inst_idx as usize;
  let prev_value_x = function.instructions[prev_inst_idx].ops[1];
  let prev_value_y = function.instructions[prev_inst_idx].ops[2];
  let prev_value_z = function.instructions[prev_inst_idx].ops[3];
  IrInst::ir_inst_new(
    IrCmd::StoreVector,
    &[target_op, prev_value_x, prev_value_y, prev_value_z, tag_op],
  )
}

/// 两个 tvalue 分支（StoreSplitTvalue/StoreVector）共用的清库尾：kill 旧记录后
/// 槽位指向新指令并刷新 tag 状态；`has_gco_to_clear |=` 因 `state` 双借用留在调用点。
fn commit_tvalue_store(
  function: &mut IrFunction,
  inst_index: u32,
  tag: u8,
  reg_info: &mut StoreRegInfo,
) {
  CODEGEN_ASSERT!(reg_info.tag_inst_idx == !0u32 && reg_info.value_inst_idx == !0u32);
  kill_t_value_store_core(function, reg_info);

  reg_info.tvalue_inst_idx = inst_index;
  reg_info.maybe_gco = is_gco(tag);
  reg_info.known_tag = tag;
}

/// 目标寄存器槽位改以索引传入：函数体内对 `state.info` 的借用与
/// `state` 其余字段的读写按字段拆分，不再依赖调用方的裸指针回转。
pub fn try_replace_tag_with_full_store(
  state: &mut RemoveDeadStoreState,
  function: &mut IrFunction,
  block_idx: u32,
  inst_index: u32,
  target_op: IrOp,
  tag_op: IrOp,
  reg_idx: usize,
) -> bool {
  let tag = function.tag_op(tag_op);
  let nil = LuaType::Nil as u8;

  let reg_info = &mut state.info[reg_idx];

  // 若 tag+value 对已确立，可将两者标记为 dead，改用单条拆分 TValue store
  if reg_info.tag_inst_idx != !0u32
    && (reg_info.value_inst_idx != !0u32 || reg_info.known_tag == nil)
  {
    if tag != nil && reg_info.value_inst_idx != !0u32 {
      let prev_cmd = function.instructions[reg_info.value_inst_idx as usize].cmd;

      if prev_cmd == IrCmd::StoreVector {
        CODEGEN_ASSERT!(!HAS_OP_E!(
          function.instructions[reg_info.value_inst_idx as usize]
        ));
        let repl = make_vector_repl(function, reg_info.value_inst_idx, target_op, tag_op);
        replace_ir_function_ir_block_u32_ir_inst(function, block_idx, inst_index, repl);
      } else {
        let prev_value_op = function.instructions[reg_info.value_inst_idx as usize].ops[1];
        let repl =
          IrInst::ir_inst_new(IrCmd::StoreSplitTvalue, &[target_op, tag_op, prev_value_op]);
        replace_ir_function_ir_block_u32_ir_inst(function, block_idx, inst_index, repl);
      }
    }

    // cpp：DseRestoreHints 开启时 killTagAndValueStorePair（先记 restore hint 再 kill），否则逐个 kill；
    // 此调用点 tag+value pair 已建立，两条路径 kill 效果一致，直接用带 hint 的 pair 版本
    kill_tag_and_value_store_pair_core(function, reg_info);

    reg_info.tvalue_inst_idx = inst_index;
    reg_info.maybe_gco = is_gco(tag);
    reg_info.known_tag = tag;
    state.has_gco_to_clear |= reg_info.maybe_gco;
    return true;
  }

  // 也可用新 store 替换已 dead 的拆分 TValue store，同时保持 value 不变
  if reg_info.tvalue_inst_idx != !0u32 {
    let prev_cmd = function.instructions[reg_info.tvalue_inst_idx as usize].cmd;

    if prev_cmd == IrCmd::StoreSplitTvalue {
      CODEGEN_ASSERT!(!HAS_OP_D!(
        function.instructions[reg_info.tvalue_inst_idx as usize]
      ));

      // 若存的是 'nil'，保留 'STORE_TAG Rn, tnil'，因为它写入的是完整 TValue
      if tag != nil {
        let prev_value_op = function.instructions[reg_info.tvalue_inst_idx as usize].ops[2];
        let repl =
          IrInst::ir_inst_new(IrCmd::StoreSplitTvalue, &[target_op, tag_op, prev_value_op]);
        replace_ir_function_ir_block_u32_ir_inst(function, block_idx, inst_index, repl);
      }

      commit_tvalue_store(function, inst_index, tag, reg_info);
      state.has_gco_to_clear |= reg_info.maybe_gco;
      return true;
    } else if prev_cmd == IrCmd::StoreVector {
      if tag != nil {
        let repl = make_vector_repl(function, reg_info.tvalue_inst_idx, target_op, tag_op);
        replace_ir_function_ir_block_u32_ir_inst(function, block_idx, inst_index, repl);
      }

      commit_tvalue_store(function, inst_index, tag, reg_info);
      state.has_gco_to_clear |= reg_info.maybe_gco;
      return true;
    }
  }

  false
}
