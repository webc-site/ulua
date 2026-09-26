use crate::{
  enums::ir_cmd::IrCmd,
  functions::{
    add_use::add_use, get_op_ir_data::get_op_mut, is_block_terminator::is_block_terminator,
    remove_use::remove_use,
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_function::IrFunction, ir_inst::IrInst, ir_op::IrOp},
};

pub fn substitute(function: &mut IrFunction, inst: &mut IrInst, replacement: IrOp) {
  CODEGEN_ASSERT!(!is_block_terminator(inst.cmd));

  inst.cmd = IrCmd::SUBSTITUTE;

  add_use(function, replacement);

  // 单次借用切片遍历，替代 C 风格索引循环
  for &op in inst.ops.as_slice() {
    remove_use(function, op);
  }

  inst.ops.resize(1);
  *get_op_mut(inst, 0) = replacement;
}

/// `substitute` 的索引化变体：只借 `&mut function`，消除调用方为绕过
/// `&mut function` 与 `&mut instructions[index]` 重叠借用而引入的裸指针 unsafe。
///
/// 与 `substitute` 逐指令 1:1：
/// - 断言与置 `SUBSTITUTE` 就地作用于原始指令；
/// - 迭代 old-ops 用快照。原裸指针版迭代 `inst.ops.as_slice()` 时，若
///   `remove_use` 恰好 kill 到 inst 自身会 clear 其 ops，但迭代器已捕获
///   clear 前的 slice header，仍遍历旧 ops 全集——快照与此等价；
/// - `use_count`/`cmd` 副作用留在原始对象，最后就地 resize 并写 op0。
pub fn substitute_at(function: &mut IrFunction, index: u32, replacement: IrOp) {
  let idx = index as usize;

  CODEGEN_ASSERT!(!is_block_terminator(function.instructions[idx].cmd));

  function.instructions[idx].cmd = IrCmd::SUBSTITUTE;

  add_use(function, replacement);

  // 快照原始 ops：与原 slice 迭代器捕获的序列完全一致
  let ops = function.instructions[idx].ops.clone();
  for &op in ops.as_slice() {
    remove_use(function, op);
  }

  let inst = &mut function.instructions[idx];
  inst.ops.resize(1);
  *get_op_mut(inst, 0) = replacement;
}
