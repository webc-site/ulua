//! 变步长指令流扫描的单一权威实现。
//!
//! cpp 各扫描点手写 `i += getOpLength(op)`（或迭代器等价的 `for _ in 1..len { next() }`），
//! Rust 端把「取字 → 算步长 → 前进」收口到 [`stepped`] 一处，扫描点只留消费逻辑。

use core::iter::from_fn;

use ulua_common::{functions::get_op_length::get_op_length, records::instruction::Instruction};

/// 从 `start` 起逐步产出 `(pc, end, insn)`：`insn` 为 `pc` 处的指令字，
/// `end = pc + get_op_length(op)` 是该指令之后第一个槽位。
///
/// 步长经 `luau_opcode()` 取值：未知操作码被钳位为 `NOP`（步长 1），故 `end > pc`
/// 恒成立，迭代器对不可信字节流也必然前进、不会空转；`pc` 越过切片末尾即结束。
/// `pc` 落在指令边界上（aux 字不单独产出），与 cpp 各扫描循环逐一对应。
pub(super) fn stepped(
  insns: &[Instruction],
  start: usize,
) -> impl Iterator<Item = (usize, usize, Instruction)> + '_ {
  let mut pc = start;
  from_fn(move || {
    let insn = insns.get(pc).copied()?;
    let cur = pc;
    pc += get_op_length(insn.luau_opcode()) as usize;
    Some((cur, pc, insn))
  })
}
