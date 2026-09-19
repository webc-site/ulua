use crate::{
  enums::ir_cmd::IrCmd,
  functions::remove_use::remove_use,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_function::IrFunction, ir_inst::IrInst},
};

pub fn kill_ir_function_ir_inst(function: &mut IrFunction, inst: &mut IrInst) {
  CODEGEN_ASSERT!(inst.use_count == 0);

  inst.cmd = IrCmd::NOP;

  for op in inst.ops.as_slice() {
    remove_use(function, *op);
  }
  inst.ops.clear();
}

/// `kill_ir_function_ir_inst` 的索引化变体：只借 `&mut function`，
/// 消除调用方为绕过重叠借用引入的裸指针 unsafe。
///
/// 与原实现逐指令 1:1：置 NOP 就地生效；迭代 old-ops 用快照（原裸指针版
/// 迭代器捕获 clear 前的 slice header，仍遍历旧 ops 全集，快照与此等价）；
/// 末尾就地清空 ops。
pub fn kill_ir_function_ir_inst_at(function: &mut IrFunction, index: u32) {
  let idx = index as usize;

  CODEGEN_ASSERT!(function.instructions[idx].use_count == 0);

  function.instructions[idx].cmd = IrCmd::NOP;

  // 快照原始 ops，避免 remove_use 中途变更本指令 ops 造成借用冲突
  let ops = function.instructions[idx].ops.clone();
  for &op in ops.as_slice() {
    remove_use(function, op);
  }

  function.instructions[idx].ops.clear();
}
