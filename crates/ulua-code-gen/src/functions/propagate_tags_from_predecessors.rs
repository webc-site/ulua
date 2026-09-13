use alloc::boxed::Box;

use crate::{
  functions::predecessors::predecessors,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_block::IrBlock, ir_function::IrFunction},
};

// IrData.h: `constexpr uint8_t kUnknownTag = 0xff;`
const K_UNKNOWN_TAG: u8 = 0xff;

pub fn propagate_tags_from_predecessors(
  function: &IrFunction,
  block: &IrBlock,
  get_tag: Box<dyn Fn(usize) -> u8>,
  set_tag: Box<dyn Fn(usize, u8)>,
) {
  let block_idx = function.get_block_index(block);

  if block_idx >= function.cfg.predecessors_offsets.len() as u32 {
    return;
  }

  let preds = predecessors(&function.cfg, block_idx);

  if preds.clone().next().is_none() {
    return;
  }

  let mut min_regs_known: usize = usize::MAX;

  let num_block_exit_tags = function.block_exit_tags.len();

  for pred_idx in preds {
    if pred_idx as usize >= num_block_exit_tags {
      return;
    }

    min_regs_known = min_regs_known.min(function.block_exit_tags[pred_idx as usize].len());
  }

  let in_ = &function.cfg.r#in[block_idx as usize];

  let mut first_predecessor = true;

  for pred_idx in predecessors(&function.cfg, block_idx) {
    let pred_tags = &function.block_exit_tags[pred_idx as usize];

    CODEGEN_ASSERT!(min_regs_known <= pred_tags.len());

    for i in 0..min_regs_known {
      // Only registers that are live in can receive information from the predecessors
      let live_in = (in_.regs[i / 64] & (1u64 << (i % 64))) != 0
        || (in_.vararg_seq && i >= in_.vararg_start as usize);

      if live_in {
        let current_tag = get_tag(i);

        if first_predecessor {
          set_tag(i, pred_tags[i]);
        } else if current_tag != K_UNKNOWN_TAG && current_tag != pred_tags[i] {
          set_tag(i, K_UNKNOWN_TAG);
        }
      }
    }

    first_predecessor = false;
  }
}
