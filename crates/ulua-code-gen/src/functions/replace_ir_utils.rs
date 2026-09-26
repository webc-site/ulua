use core::mem::take;

use crate::{
  enums::ir_block_kind::IrBlockKind,
  functions::{
    add_use::add_use, is_block_terminator::is_block_terminator,
    kill_ir_utils::kill_ir_function_u32_u32, remove_use::remove_use,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_function::IrFunction, ir_inst::IrInst, ir_op::IrOp},
};

pub fn replace_ir_function_ir_op_ir_op(
  function: &mut IrFunction,
  original: &mut IrOp,
  replacement: IrOp,
) {
  // 若这是维持目标操作符存活的最后一个 use，先加新 use 再删除
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

/// cpp `replace(IrFunction&, IrBlock&, uint, IrInst)`：按块内下标整条替换指令
///
/// 目标块以索引传入（索引化变体）：块视图与函数借用同源于 `function`，
/// 由本函数内部即时解引用，调用方不再需要 `&mut IrFunction` 与 `&mut IrBlock`
/// 重叠借用的裸指针别名。
pub fn replace_ir_function_ir_block_u32_ir_inst(
  function: &mut IrFunction,
  block_idx: u32,
  inst_idx: u32,
  mut replacement: IrInst,
) {
  // 若这些是维持目标操作符存活的最后几个 use，先加 use 再删除
  for &op in replacement.ops.as_slice() {
    add_use(function, op);
  }

  // 若插入了更早的终止指令，其后所有指令都变成 dead
  let inst_cmd = function.instructions[inst_idx as usize].cmd;
  if !is_block_terminator(inst_cmd) && is_block_terminator(replacement.cmd) {
    // 执行替换前 block 必须已完整构建
    let block_finish = function.blocks[block_idx as usize].finish;
    CODEGEN_ASSERT!(block_finish != !0u32);
    CODEGEN_ASSERT!(inst_idx < block_finish);

    kill_ir_function_u32_u32(function, inst_idx + 1, block_finish);

    // 若删除该 range 连带杀掉了当前 block，需回滚替换指令的 use 并退出
    if function.blocks[block_idx as usize].kind == IrBlockKind::Dead {
      for &op in replacement.ops.as_slice() {
        remove_use(function, op);
      }
      return;
    }

    function.blocks[block_idx as usize].finish = inst_idx;
  }

  let inst = &mut function.instructions[inst_idx as usize];
  // 继承既有 use 计数（最后一个 use 跳过，稍后会重新定义）
  replacement.use_count = inst.use_count;

  // 取出旧操作数即可，无需整条 IrInst clone
  let old_ops = take(&mut inst.ops);
  *inst = replacement;

  for op in old_ops.as_slice() {
    remove_use(function, *op);
  }
}
