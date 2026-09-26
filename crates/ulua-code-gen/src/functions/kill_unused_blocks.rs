use crate::{
  enums::ir_block_kind::IrBlockKind, functions::kill_ir_utils::kill_ir_function_ir_block_at,
  records::ir_function::IrFunction,
};

pub fn kill_unused_blocks(function: &mut IrFunction) {
  // 从 1 开始，因为第 0 个 block 是 entry block
  // kill 只就地标记块为 Dead（不改 blocks 长度），故顺序游标即等价于定长区间遍历
  for i in 1..function.blocks.len() {
    let block = &function.blocks[i];
    if block.kind != IrBlockKind::Dead && block.use_count == 0 {
      kill_ir_function_ir_block_at(function, i);
    }
  }
}
