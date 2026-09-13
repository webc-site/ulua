use crate::{
  functions::{append::append, get_block_kind_name::get_block_kind_name},
  records::{ir_block::IrBlock, ir_to_string_context::IrToStringContext},
};

pub fn to_string_ir_to_string_context_ir_block_u32(
  ctx: &mut IrToStringContext,
  block: &IrBlock,
  index: u32,
) {
  append(
    ctx.result,
    format_args!("{}_{}", get_block_kind_name(block.kind), index),
  );
}
