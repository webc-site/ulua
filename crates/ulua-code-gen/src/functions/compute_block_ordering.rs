use crate::{
  functions::successors::successors,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{block_ordering::BlockOrdering, ir_function::IrFunction},
};

pub fn compute_block_ordering(
  function: &mut IrFunction,
  ordering: &mut Vec<BlockOrdering>,
  pre_order: Option<&mut Vec<u32>>,
  post_order: Option<&mut Vec<u32>>,
) {
  CODEGEN_ASSERT!(function.cfg.idoms.len() == function.blocks.len());

  ordering.clear();
  ordering.resize(function.blocks.len(), BlockOrdering::default());

  let mut pre_order = pre_order;
  let mut post_order = post_order;

  if let Some(pre_order) = pre_order.as_deref_mut() {
    pre_order.reserve(function.blocks.len());
  }
  if let Some(post_order) = post_order.as_deref_mut() {
    post_order.reserve(function.blocks.len());
  }

  if function.blocks.is_empty() {
    return;
  }

  let mut stack: Vec<(u32, u32)> = Vec::new();
  let mut next_pre_order = 0u32;
  let mut next_post_order = 0u32;

  stack.push((0, 0));
  ordering[0].visited = true;
  ordering[0].pre_order = next_pre_order;
  next_pre_order += 1;

  while let Some((block_idx, mut it_pos)) = stack.pop() {
    let children = successors(&function.cfg, block_idx);

    if it_pos < children.size() as u32 {
      let child_idx = children.operator_index(it_pos as usize);
      it_pos += 1;
      stack.push((block_idx, it_pos));

      let child_ordering = &mut ordering[child_idx as usize];

      if !child_ordering.visited {
        child_ordering.visited = true;
        child_ordering.depth = stack.len() as u32;
        child_ordering.pre_order = next_pre_order;
        next_pre_order += 1;

        if let Some(pre_order) = pre_order.as_deref_mut() {
          pre_order.push(block_idx);
        }

        stack.push((child_idx, 0));
      }
    } else {
      ordering[block_idx as usize].post_order = next_post_order;
      next_post_order += 1;

      if let Some(post_order) = post_order.as_deref_mut() {
        post_order.push(block_idx);
      }
    }
  }
}
