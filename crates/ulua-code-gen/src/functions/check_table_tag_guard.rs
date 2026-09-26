use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::ir_cmd::IrCmd,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

/// table-tag 守卫收口（length/gettable/gettablen/gettableks 四站同构）：bytecode 类型侧
/// 为 TABLE 时失败即 vm_exit，否则走 fallback，随后 `CheckTag` 校验 Table。
#[inline]
pub(crate) fn check_table_tag_guard(
  build: &mut IrBuilder,
  tb_op: IrOp,
  types_side_is_table: bool,
  pcpos: i32,
  fallback: IrOp,
) {
  let table_tag = build.const_tag(LuaType::Table as u8);
  let exit_or_fallback = if types_side_is_table {
    build.vm_exit(pcpos as u32)
  } else {
    fallback
  };
  build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CheckTag, tb_op, table_tag, exit_or_fallback);
}
