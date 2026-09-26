use crate::{
  functions::{
    compute_block_ordering::compute_block_ordering, find_common_dominator::find_common_dominator,
    predecessors::predecessors,
  },
  records::ir_function::IrFunction,
};

pub fn compute_cfg_immediate_dominators(function: &mut IrFunction) {
  let block_count = function.blocks.len();

  function.cfg.idoms.clear();
  function.cfg.idoms.resize(block_count, !0u32);

  if block_count == 0 {
    return;
  }

  // 出参改返回值：仅需后序序列，前序收集关闭
  let (ordering, _pre_order, blocks_in_post_order) = compute_block_ordering(function, false, true);

  function.cfg.idoms[0] = 0;

  let mut updated = true;
  while updated {
    updated = false;

    if blocks_in_post_order.len() < 2 {
      break;
    }

    // 去掉末位（根块）后逆序迭代
    for &block_idx in blocks_in_post_order[..blocks_in_post_order.len() - 1]
      .iter()
      .rev()
    {
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
