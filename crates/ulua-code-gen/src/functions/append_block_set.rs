use crate::{
  functions::{
    append::append, to_string_ir_dump_alt_c::to_string_ir_to_string_context_ir_block_u32,
  },
  records::{
    block_iterator_wrapper::BlockIteratorWrapper, ir_block::IrBlock,
    ir_to_string_context::IrToStringContext,
  },
};

pub fn append_block_set(ctx: &mut IrToStringContext, blocks: BlockIteratorWrapper) {
  let mut comma = false;

  // Iterate using the wrapper's raw pointer range: [begin, end)
  let mut it = blocks.begin();
  let end = blocks.end();

  while it < end {
    let target = unsafe { *it };
    it = unsafe { it.add(1) };

    if comma {
      append(ctx.result, format_args!(", "));
    }
    comma = true;

    let block: &IrBlock = &ctx.blocks[target as usize];
    to_string_ir_to_string_context_ir_block_u32(ctx, block, target);
  }
}
