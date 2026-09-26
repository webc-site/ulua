use crate::{
  enums::{ir_block_kind::IrBlockKind, ir_cmd::IrCmd},
  functions::remove_use::remove_use,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::ir_function::IrFunction,
};

/// cpp `kill(IrFunction&, uint32_t)`：置 NOP 并按 ops 快照逐出使用计数。
///
/// 只借 `&mut function`，迭代用 ops 快照（remove_use 可能级联 kill 本指令），
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

/// cpp `kill(IrFunction&, uint, uint)`：区间内可安全清除的指令逐个 kill
pub fn kill_ir_function_u32_u32(function: &mut IrFunction, start: u32, end: u32) {
  // 逆序 kill，避免误杀仍被标记为使用的指令
  for idx in (start..=end).rev() {
    CODEGEN_ASSERT!((idx as usize) < function.instructions.len());

    let inst = &function.instructions[idx as usize];
    // 只读判定：cmd 为 NOP 或仍在使用则跳过；借用在此结束，下面的 kill 才能取可变借用
    let killable = inst.cmd != IrCmd::NOP && inst.use_count == 0;

    // 不强制销毁仍在使用的指令：操作数释放时它会随之自动销毁
    if killable {
      kill_ir_function_ir_inst_at(function, idx);
    }
  }
}

/// `kill(IrFunction&, IrBlock&)` 的索引化变体（cpp `kill(function, block)`）：只借
/// `&mut function`，消除调用方为绕过「function 与其内部块」重叠借用引入的裸指针回转
/// （对齐已入库的 `kill_ir_function_ir_inst_at` 模式）。
pub fn kill_ir_function_ir_block_at(function: &mut IrFunction, index: usize) {
  CODEGEN_ASSERT!(function.blocks[index].use_count == 0);

  function.blocks[index].kind = IrBlockKind::Dead;

  let (start, finish) = (function.blocks[index].start, function.blocks[index].finish);
  kill_ir_function_u32_u32(function, start, finish);

  let block = &mut function.blocks[index];
  block.start = !0u32;
  block.finish = !0u32;
}
