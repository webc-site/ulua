use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::ir_cmd::IrCmd,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

/// bit32/math/int64 族 builtin 快速路径的共同收尾（cpp IrTranslateBuiltins.cpp 各翻译器
/// 内联同形尾巴的收敛）：把 number 结果 `StoreDouble` 写入 ra；`ra != arg` 时 check 只
/// 落在 arg 上、ra 的旧 tag 尚未失效，须补写 number tag（`LUA_TNUMBER` = 3）。
pub(crate) fn builtin_store_number_result(build: &mut IrBuilder, ra: i32, arg: i32, value: IrOp) {
  let ra_reg = build.vm_reg(ra as u8);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreDouble, ra_reg, value);

  if ra != arg {
    build.store_tag(ra_reg, LuaType::Number as u8);
  }
}
