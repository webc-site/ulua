use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::{
    produces_dirty_high_register_bits::produces_dirty_high_register_bits,
    replace_ir_utils::replace_ir_function_ir_block_u32_ir_inst, substitute::substitute_at,
  },
  records::{ir_function::IrFunction, ir_inst::IrInst, ir_op::IrOp},
  type_aliases::ir_ops::IrOps,
};

/// 索引化变体：只借 `&mut function`，消除调用方为绕过重叠借用引入的裸指针 unsafe。
///
/// 与原实现 1:1：脏高位判定读同一指令的 cmd（原实现经由 `&mut *inst` 读取，
/// 这里用索引读取同一位置），命中走 `replace_ir_function_ir_block_u32_ir_inst`
/// 直接按 index 替换；否则走 `substitute_at`。原实现的 `get_inst_index(inst)`
/// 本就等于这里的 `index`，故省略指针换算。
pub fn substitute_with_truncated_uint_at(
  function: &mut IrFunction,
  block_idx: u32,
  index: u32,
  op: IrOp,
) -> bool {
  // 脏高位判定：仅当 op 指向 Inst 时才读取其 cmd（与原 as_inst_op 空指针分支等价）
  let dirty = op.kind() == IrOpKind::Inst
    && produces_dirty_high_register_bits(function.instructions[op.index() as usize].cmd);

  if dirty {
    let mut ops = IrOps::new();
    ops.push(op);
    let replacement = IrInst {
      cmd: IrCmd::TruncateUint,
      ops,
      ..Default::default()
    };
    replace_ir_function_ir_block_u32_ir_inst(function, block_idx, index, replacement);

    true
  } else {
    substitute_at(function, index, op);

    false
  }
}
