use crate::{
  functions::dom_children::dom_children,
  records::{block_ordering::BlockOrdering, ir_function::IrFunction},
};

pub fn compute_cfg_dominance_tree_children(function: &mut IrFunction) {
  let block_count = function.blocks.len();

  function.cfg.dom_children.clear();
  function.cfg.dom_children_offsets.clear();
  function.cfg.dom_children_offsets.resize(block_count, 0);

  for block_idx in 0..block_count {
    let dom_parent = function.cfg.idoms[block_idx];

    if dom_parent != !0u32 {
      function.cfg.dom_children_offsets[dom_parent as usize] += 1;
    }
  }

  let mut total = 0u32;
  for item in function
    .cfg
    .dom_children_offsets
    .iter_mut()
    .take(block_count)
  {
    let count = *item;
    *item = total;
    total += count;
  }

  function.cfg.dom_children.resize(total as usize, 0);

  for block_idx in 0..block_count {
    let dom_parent = function.cfg.idoms[block_idx];

    if dom_parent != !0u32 {
      let offset = function.cfg.dom_children_offsets[dom_parent as usize] as usize;
      function.cfg.dom_children[offset] = block_idx as u32;
      function.cfg.dom_children_offsets[dom_parent as usize] += 1;
    }
  }

  for block_idx in (1..block_count).rev() {
    function.cfg.dom_children_offsets[block_idx] = function.cfg.dom_children_offsets[block_idx - 1];
  }

  if block_count != 0 {
    function.cfg.dom_children_offsets[0] = 0;
  }

  compute_dom_child_ordering(function);
}

fn compute_dom_child_ordering(function: &mut IrFunction) {
  let block_count = function.blocks.len();

  function.cfg.dom_ordering.clear();
  function
    .cfg
    .dom_ordering
    .resize(block_count, BlockOrdering::default());

  if block_count == 0 {
    return;
  }

  let mut stack: Vec<(u32, u32)> = Vec::new();
  let mut next_pre_order = 0u32;
  let mut next_post_order = 0u32;

  stack.push((0, 0));
  function.cfg.dom_ordering[0].visited = true;
  function.cfg.dom_ordering[0].pre_order = next_pre_order;
  next_pre_order += 1;

  while let Some((block_idx, mut it_pos)) = stack.pop() {
    let children = dom_children(&function.cfg, block_idx);

    if it_pos < children.size() as u32 {
      let child_idx = children.operator_index(it_pos as usize);
      it_pos += 1;
      stack.push((block_idx, it_pos));

      if !function.cfg.dom_ordering[child_idx as usize].visited {
        function.cfg.dom_ordering[child_idx as usize].visited = true;
        function.cfg.dom_ordering[child_idx as usize].depth = stack.len() as u32;
        function.cfg.dom_ordering[child_idx as usize].pre_order = next_pre_order;
        next_pre_order += 1;
        stack.push((child_idx, 0));
      }
    } else {
      function.cfg.dom_ordering[block_idx as usize].post_order = next_post_order;
      next_post_order += 1;
    }
  }
}
