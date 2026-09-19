use crate::{
  functions::{add_use::add_use, remove_use::remove_use},
  records::{ir_function::IrFunction, ir_op::IrOp},
};

pub fn replace_ir_function_ir_op_ir_op(
  function: &mut IrFunction,
  original: &mut IrOp,
  replacement: IrOp,
) {
  // Add use before removing new one if that's the last one keeping target operand alive
  add_use(function, replacement);
  remove_use(function, *original);

  *original = replacement;
}

/// `replace_ir_function_ir_op_ir_op` 的索引化变体：目标槽位定位在
/// `function.instructions[index].ops[slot]`，消除调用方为绕过
/// `&mut function` 与 `&mut inst` 重叠借用引入的裸指针 unsafe。
///
/// 与原实现（先 `get_op_mut` 取槽位再调用本函数）逐操作 1:1：
/// - 先按 `get_op_mut` 语义补位（仅在越界时扩容，绝不截断）；
/// - 快照槽位旧值，再 add_use / remove_use，顺序不变；
/// - remove_use 可能顺带 kill 掉本指令（ops 被清空），原裸指针此时写入
///   落入已死槽位、永不可见；这里以长度判断等价丢弃该写入。
pub fn replace_ir_function_ir_op_ir_op_at(
  function: &mut IrFunction,
  index: u32,
  slot: u32,
  replacement: IrOp,
) {
  let idx = index as usize;

  if slot >= function.instructions[idx].ops.size() {
    function.instructions[idx].ops.resize(slot + 1);
  }
  let original = function.instructions[idx].ops[slot as usize];

  add_use(function, replacement);
  remove_use(function, original);

  let inst = &mut function.instructions[idx];
  if slot < inst.ops.size() {
    inst.ops[slot as usize] = replacement;
  }
}
