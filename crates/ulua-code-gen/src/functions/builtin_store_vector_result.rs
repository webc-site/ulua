use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::ir_cmd::IrCmd,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

/// 标量三分量向量结果 builtin（vector 构造 / vector.cross / map1 标量路径）的公共
/// 尾部（cpp `buildStoreVector(A, x, y, z) + buildStoreTag(LUA_TVECTOR)` 收敛）：
/// 以 `StoreVector` 存 x/y/z 后打 Vector 标签。
pub(crate) fn builtin_store_vector_result(
  build: &mut IrBuilder,
  ra: i32,
  x: IrOp,
  y: IrOp,
  z: IrOp,
) {
  let ra_reg = build.vm_reg(ra as u8);
  build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::StoreVector, ra_reg, x, y, z);
  build.store_tag(ra_reg, LuaType::Vector as u8);
}
