//! LOP_GETTABLE / LOP_SETTABLE 共享翻译骨架。
//!
//! cpp `IrBuilder.cpp` 中 translateInstGETTABLE/SETTABLE 是两份同型展开：两条指令
//! 仅在三处不同 —— 表命令方向（`cmd`：GetTable/SetTable）、SET 侧写前追加的只读
//! 检查（CheckReadonly）、读写搬移方向（GET 取数组槽 TValue 入 RAr；SET 取 RAr
//! 写回数组槽并补 BarrierTableForward）。其余逐指令同构，收敛到
//! [`translate_table_access`] 单源，`IrCmd::SetTable` 即 SETTABLE 路径。

use ulua_common::{
  enums::luau_bytecode_type::LuauBytecodeType,
  macros::luau_insn_ops::{luau_insn_a, luau_insn_b, luau_insn_c},
};

use crate::{
  enums::ir_cmd::IrCmd,
  functions::{
    check_table_tag_guard::check_table_tag_guard,
    is_userdata_bytecode_type::is_userdata_bytecode_type,
    translate_inst_binary::check_number_tag_guard,
  },
  records::{fallback_stream_scope::FallbackStreamScope, ir_builder::IrBuilder},
  type_aliases::instruction_ir_builder::Instruction,
};

pub fn translate_inst_get_table(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  translate_table_access(build, code, pcpos, IrCmd::GetTable)
}

/// GETTABLE/SETTABLE 共用主体：`cmd` 取 `GetTable`（读）/`SetTable`（写）。
pub(crate) fn translate_table_access(
  build: &mut IrBuilder,
  code: &[Instruction],
  pcpos: i32,
  cmd: IrCmd,
) {
  let is_set = cmd == IrCmd::SetTable;

  // 界内约定:  契约保证 code 切片于 pcpos 指向字节码缓冲内存活、对齐(Instruction=u32)的合法指令字，code[pcpos] 只读
  // 取出整条指令供 A/B/C 域提取，纯读无别名冲突。
  let insn = code[pcpos as usize];
  let ra = luau_insn_a(insn) as u8;
  let rb = luau_insn_b(insn) as u8;
  let rc = luau_insn_c(insn) as u8;

  let bc_types = build.function.get_bytecode_types_at(pcpos);

  if is_userdata_bytecode_type(bc_types.a)
    || bc_types.b == LuauBytecodeType::LBC_TYPE_STRING.0 as u8
  {
    let savedpc_arg = build.const_uint((pcpos + 1) as u32);
    build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc_arg);
    let reg_ra = build.vm_reg(ra);
    let reg_rb = build.vm_reg(rb);
    let reg_rc = build.vm_reg(rc);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(cmd, reg_ra, reg_rb, reg_rc);
    return;
  }

  // 前置 Table 守卫需 fallback 值供后续 TryNumToIndex/CheckArraySize 等复用，保持 eager 创建。
  let mut fallback = build.fallback_block(pcpos as u32);

  let reg_rb = build.vm_reg(rb);
  let tb = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg_rb);
  let a_is_table = bc_types.a == LuauBytecodeType::LBC_TYPE_TABLE.0 as u8;
  check_table_tag_guard(build, tb, a_is_table, pcpos, fallback);

  // R93 守卫并网：双臂（b 侧 NUMBER → vm_exit，否则 fallback）与原全形态守卫同形。
  let c_is_number = bc_types.b == LuauBytecodeType::LBC_TYPE_NUMBER.0 as u8;
  check_number_tag_guard(build, rc, c_is_number, pcpos, &mut fallback);

  let reg_rc = build.vm_reg(rc);
  let vb = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, reg_rb);
  let vc = build.inst_ir_cmd_ir_op(IrCmd::LoadDouble, reg_rc);

  let index = build.inst_ir_cmd_ir_op_ir_op(IrCmd::TryNumToIndex, vc, fallback);

  let const_int_one = build.const_int(1);
  let index = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SubInt, index, const_int_one);

  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckArraySize, vb, index, fallback);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckNoMetatable, vb, fallback);

  let reg_ra = build.vm_reg(ra);

  if is_set {
    // SET：写方向须先挡掉只读表，再取数组槽写回；末尾补前向 barrier，防旧值所在表逃逸
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckReadonly, vb, fallback);
    let arr_el = build.inst_ir_cmd_ir_op_ir_op(IrCmd::GetArrAddr, vb, index);
    let tva = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, reg_ra);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, arr_el, tva);
    let undef = build.undef();
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::BarrierTableForward, vb, reg_ra, undef);
  } else {
    // GET：读方向，数组槽 TValue 直入 RAr
    let arr_el = build.inst_ir_cmd_ir_op_ir_op(IrCmd::GetArrAddr, vb, index);
    let arr_el_tval = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, arr_el);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, reg_ra, arr_el_tval);
  }

  let next = build.block_at_inst((pcpos + 1) as u32);

  let scope = FallbackStreamScope::new(build, fallback, next);
  let build = &mut *scope.build;

  let savedpc_arg = build.const_uint((pcpos + 1) as u32);
  build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc_arg);
  let reg_ra = build.vm_reg(ra);
  let reg_rb = build.vm_reg(rb);
  let reg_rc = build.vm_reg(rc);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(cmd, reg_ra, reg_rb, reg_rc);
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
}
