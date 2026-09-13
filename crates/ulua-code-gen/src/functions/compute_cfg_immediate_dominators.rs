use crate::{
  functions::{
    compute_block_ordering::compute_block_ordering, find_common_dominator::find_common_dominator,
    predecessors::predecessors,
  },
  records::{block_ordering::BlockOrdering, ir_function::IrFunction},
};

pub fn compute_cfg_immediate_dominators(function: &mut IrFunction) {
  let block_count = function.blocks.len();

  function.cfg.idoms.clear();
  function.cfg.idoms.resize(block_count, !0u32);

  if block_count == 0 {
    return;
  }

  let mut ordering: Vec<BlockOrdering> = Vec::new();
  let mut blocks_in_post_order: Vec<u32> = Vec::new();
  compute_block_ordering(
    function,
    &mut ordering,
    None,
    Some(&mut blocks_in_post_order),
  );

  function.cfg.idoms[0] = 0;

  let mut updated = true;
  while updated {
    updated = false;

    if blocks_in_post_order.len() < 2 {
      break;
    }

    for i in (0..blocks_in_post_order.len() - 1).rev() {
      let block_idx = blocks_in_post_order[i];
      let mut new_idom = !0u32;

      for pred_idx in predecessors(&function.cfg, block_idx) {
        let pred_idom = function.cfg.idoms[pred_idx as usize];

        if pred_idom != !0u32 {
          if new_idom == !0u32 {
            new_idom = pred_idx;
          } else {
            new_idom = find_common_dominator(&function.cfg.idoms, &ordering, new_idom, pred_idx);
          }
        }
      }

      if new_idom != function.cfg.idoms[block_idx as usize] {
        function.cfg.idoms[block_idx as usize] = new_idom;
        updated = true;
      }
    }
  }

  function.cfg.idoms[0] = !0u32;
}
