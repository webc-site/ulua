use crate::{
  functions::{append::append, to_string_ir_dump::to_string_ir_to_string_context_ir_block_u32},
  records::{
    block_iterator_wrapper::BlockIteratorWrapper, ir_block::IrBlock,
    ir_to_string_context::IrToStringContext,
  },
};

pub fn append_block_set(ctx: &mut IrToStringContext, blocks: BlockIteratorWrapper<'_>) {
  for (index, target) in blocks.enumerate() {
    if index != 0 {
      append(ctx.result, format_args!(", "));
    }

    let block: &IrBlock = &ctx.blocks[target as usize];
    to_string_ir_to_string_context_ir_block_u32(ctx, block, target);
  }
}
