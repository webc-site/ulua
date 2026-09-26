use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_function::IrFunction, ir_inst::IrInst},
};

pub fn update_use_counts(function: &mut IrFunction) {
  for block in &mut function.blocks {
    block.use_count = 0;
  }

  for inst in &mut function.instructions {
    inst.use_count = 0;
  }

  // 操作数可自引用（Inst 可指向任意指令含自身），cpp 直接下标混用读写。
  // 逐条拷贝 ops（SmallVec 内联，通常无堆分配）后再回写：
  // 若直接迭代 &instructions 同时经指针/索引可变写，共享借用与 &mut 别名是 UB
  for inst_idx in 0..function.instructions.len() {
    let ops = function.instructions[inst_idx].ops.clone();
    for op in &ops {
      match op.kind() {
        IrOpKind::Inst => {
          let target: &mut IrInst = &mut function.instructions[op.index() as usize];
          debug_assert!(target.use_count < 0xffff);
          target.use_count += 1;
        }
        IrOpKind::Block => {
          let target = &mut function.blocks[op.index() as usize];
          debug_assert!(target.use_count < 0xffff);
          target.use_count += 1;
        }
        _ => {}
      }
    }
  }
}
