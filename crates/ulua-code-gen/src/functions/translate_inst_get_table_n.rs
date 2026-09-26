//! LOP_GETTABLEN / LOP_SETTABLEN 共享翻译骨架。
//!
//! 与 [`crate::functions::translate_inst_get_table`]（GETTABLE/SETTABLE）同理：
//! cpp `IrBuilder.cpp` 两份同型展开仅在三处不同 —— 表命令方向（`cmd`）、SET 侧
//! 写前追加的 CheckReadonly、读写搬移方向（GET 读数组槽 TValue 入 RAr；SET 取
//! RAr 写回数组槽并补 BarrierTableForward）。其余逐指令同构，收敛到
//! [`translate_table_access_n`] 单源，`IrCmd::SetTable` 即 SETTABLEN 路径。

use core::mem::size_of;

use ulua_common::{
  enums::luau_bytecode_type::LuauBytecodeType,
  macros::luau_insn_ops::{luau_insn_a, luau_insn_b, luau_insn_c},
};
use ulua_vm::type_aliases::t_value::TValue;

use crate::{
  enums::ir_cmd::IrCmd,
  functions::{
    check_table_tag_guard::check_table_tag_guard,
    is_userdata_bytecode_type::is_userdata_bytecode_type,
  },
  records::{fallback_stream_scope::FallbackStreamScope, ir_builder::IrBuilder},
  type_aliases::instruction_ir_builder::Instruction,
};

pub fn translate_inst_get_table_n(build: &mut IrBuilder, code: &[Instruction], pcpos: i32) {
  translate_table_access_n(build, code, pcpos, IrCmd::GetTable)
}

/// GETTABLEN/SETTABLEN 共用主体：`cmd` 取 `GetTable`（读）/`SetTable`（写）。
pub(crate) fn translate_table_access_n(
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
  let c = luau_insn_c(insn);

  let bc_types = build.function.get_bytecode_types_at(pcpos);

  if is_userdata_bytecode_type(bc_types.a) {
    let savedpc_arg = build.const_uint((pcpos + 1) as u32);
    build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc_arg);
    let reg_ra = build.vm_reg(ra);
    let reg_rb = build.vm_reg(rb);
    let c_plus_one = build.const_uint(c + 1);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(cmd, reg_ra, reg_rb, c_plus_one);
    return;
  }

  let fallback = build.fallback_block(pcpos as u32);

  let reg_rb = build.vm_reg(rb);
  let tb = build.inst_ir_cmd_ir_op(IrCmd::LoadTag, reg_rb);
  let a_is_table = bc_types.a == LuauBytecodeType::LBC_TYPE_TABLE.0 as u8;
  check_table_tag_guard(build, tb, a_is_table, pcpos, fallback);

  let vb = build.inst_ir_cmd_ir_op(IrCmd::LoadPointer, reg_rb);

  let c_op = build.const_int(c as i32);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckArraySize, vb, c_op, fallback);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckNoMetatable, vb, fallback);

  let zero = build.const_int(0);
  let reg_ra = build.vm_reg(ra);

  if is_set {
    // SET：写方向须先挡掉只读表，再取数组槽写回；末尾补前向 barrier，防旧值所在表逃逸
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::CheckReadonly, vb, fallback);
    let arr_el = build.inst_ir_cmd_ir_op_ir_op(IrCmd::GetArrAddr, vb, zero);
    let tva = build.inst_ir_cmd_ir_op(IrCmd::LoadTvalue, reg_ra);
    let c_times_sizeof = build.const_int((c as i32) * (size_of::<TValue>() as i32));
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::StoreTvalue, arr_el, tva, c_times_sizeof);
    let undef = build.undef();
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::BarrierTableForward, vb, reg_ra, undef);
  } else {
    // GET：读方向，数组槽 TValue 直入 RAr
    let arr_el = build.inst_ir_cmd_ir_op_ir_op(IrCmd::GetArrAddr, vb, zero);
    let c_times_sizeof = build.const_int((c as i32) * (size_of::<TValue>() as i32));
    let arr_el_tval = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LoadTvalue, arr_el, c_times_sizeof);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, reg_ra, arr_el_tval);
  }

  let next = build.block_at_inst((pcpos + 1) as u32);

  let scope = FallbackStreamScope::new(build, fallback, next);
  let build = &mut *scope.build;

  let savedpc_arg = build.const_uint((pcpos + 1) as u32);
  build.inst_ir_cmd_ir_op(IrCmd::SetSavedpc, savedpc_arg);
  let reg_ra = build.vm_reg(ra);
  let reg_rb = build.vm_reg(rb);
  let c_plus_one = build.const_uint(c + 1);
  build.inst_ir_cmd_ir_op_ir_op_ir_op(cmd, reg_ra, reg_rb, c_plus_one);
  build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
}
