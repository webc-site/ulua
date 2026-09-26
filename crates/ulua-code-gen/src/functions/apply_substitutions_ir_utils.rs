use crate::{
  enums::{ir_cmd::IrCmd, ir_op_kind::IrOpKind},
  functions::remove_use::remove_use,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::ir_function::IrFunction,
};

/// cpp `applySubstitutions(IrFunction&, IrInst&)` 的索引化变体：目标指令按
/// `function.instructions[index]` 即时定位，操作数槽位就地回写，调用方不再
/// 需要 `&mut IrFunction` 与 `&mut IrInst` 重叠借用的裸指针别名。
///
/// 与原实现 1:1：遍历指令起始时刻的 ops（原 `as_mut_slice()` 捕获的切片），
/// 若中途 remove_use 链清空本指令 ops（原实现迭代悬垂槽位、写入不可见），
/// 此处按长度判定跳过该槽位，等价丢弃悬垂写入；每个槽位先读当前值，
/// 与原实现经 `&mut IrOp` 实时读取一致。
pub fn apply_substitutions_at(function: &mut IrFunction, index: u32) {
  let idx = index as usize;
  let op_count = function.instructions[idx].ops.size();

  for slot in 0..op_count as usize {
    // 原实现逐槽位经 `&mut IrOp` 实时读；等价地每轮取当前值快照（IrOp 为 Copy）
    let Some(&op) = function.instructions[idx].ops.get(slot) else {
      break;
    };

    if op.kind() != IrOpKind::Inst {
      continue;
    }

    // src 指令下标先行快照：槽位被替换后仍需按索引回写 src 的 use_count。
    // cmd 先行判定，ops[0] 仅在 SUBSTITUTE 分支读取（其他指令 ops 可为空）
    let src_idx = op.index();
    let src_cmd = function.instructions[src_idx as usize].cmd;

    if src_cmd != IrCmd::SUBSTITUTE {
      continue;
    }

    let src_a = function.instructions[src_idx as usize].ops.as_slice()[0];

    // 就地回写槽位（原实现经 `*op = src_a` 写指令内元素），后续按 src_a 语义推进
    function.instructions[idx].ops[slot] = src_a;

    // 若替换为另一条指令的结果，更新 use 计数
    if src_a.kind() == IrOpKind::Inst {
      let dst = &mut function.instructions[src_a.index() as usize];
      CODEGEN_ASSERT!(dst.cmd != IrCmd::SUBSTITUTE);
      dst.use_count += 1;
    }

    let dead = {
      let src = &mut function.instructions[src_idx as usize];
      CODEGEN_ASSERT!(src.use_count > 0);
      src.use_count -= 1;
      let dead = src.use_count == 0;
      if dead {
        src.cmd = IrCmd::NOP;
      }
      dead
    };

    if dead {
      let src_a2 = function.instructions[src_idx as usize].ops.as_slice()[0];
      remove_use(function, src_a2);
      function.instructions[src_idx as usize].ops.clear();
    }
  }
}
