use alloc::vec;

use crate::{enums::ir_op_kind::IrOpKind, records::ir_function::IrFunction};

pub fn update_use_counts(function: &mut IrFunction) {
  // 先以共享只读迭代统计引用计数，免去对每条指令 ops 的克隆（SmallVec 拷贝）
  // 与写别名冲突；随后以 zip 批量写回，消灭 C 风格下标循环与初始清零循环。
  let mut inst_counts = vec![0u16; function.instructions.len()];
  let mut block_counts = vec![0u16; function.blocks.len()];

  for inst in &function.instructions {
    for op in &inst.ops {
      match op.kind() {
        IrOpKind::Inst => {
          let count = &mut inst_counts[op.index() as usize];
          debug_assert!(*count < 0xffff);
          *count += 1;
        }
        IrOpKind::Block => {
          let count = &mut block_counts[op.index() as usize];
          debug_assert!(*count < 0xffff);
          *count += 1;
        }
        _ => {}
      }
    }
  }

  for (inst, count) in function.instructions.iter_mut().zip(inst_counts) {
    inst.use_count = count;
  }
  for (block, count) in function.blocks.iter_mut().zip(block_counts) {
    block.use_count = count;
  }
}
