use crate::{
  functions::replace_ir_utils::replace_ir_function_ir_op_ir_op_at,
  records::{ir_function::IrFunction, ir_op::IrOp},
};

/// 与原版逐操作 1:1：`get_op_mut` 的按需补位（仅越界扩容）与 kill 后写入丢弃
/// 均由索引化替换变体内部等价处理（见其文档注释），无需裸指针绕借用。
pub fn replace_ir_function_ir_inst_operand(
  function: &mut IrFunction,
  inst_idx: u32,
  op_idx: u32,
  replacement: IrOp,
) {
  replace_ir_function_ir_op_ir_op_at(function, inst_idx, op_idx, replacement);
}
