use crate::{
  enums::ir_cmd::IrCmd,
  records::{ir_builder::IrBuilder, ir_op::IrOp},
};

/// TValue 形态向量结果 builtin（map1_x_4 / min_max / lerp / normalize）的公共尾部
/// （cpp `buildStoreTvalue(tagVector(value))` 收敛）：`TagVector` 打标向量值后
/// `StoreTvalue` 整体写回 ra。
pub(crate) fn builtin_store_vector_tvalue_result(build: &mut IrBuilder, ra: i32, value: IrOp) {
  let ra_reg = build.vm_reg(ra as u8);
  let tagged = build.inst_ir_cmd_ir_op(IrCmd::TagVector, value);
  build.inst_ir_cmd_ir_op_ir_op(IrCmd::StoreTvalue, ra_reg, tagged);
}
