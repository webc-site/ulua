use crate::{
  functions::successors::successors,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{block_ordering::BlockOrdering, ir_function::IrFunction},
};

/// 对 CFG 做迭代式 DFS，产出逐块 `BlockOrdering`（pre/post 序号与深度）。
/// `collect_pre_order`/`collect_post_order` 按需收集前序/后序块号序列，
/// 未请求时返回对应空 Vec。
pub fn compute_block_ordering(
  function: &mut IrFunction,
  collect_pre_order: bool,
  collect_post_order: bool,
) -> (Vec<BlockOrdering>, Vec<u32>, Vec<u32>) {
  CODEGEN_ASSERT!(function.cfg.idoms.len() == function.blocks.len());

  let mut ordering = vec![BlockOrdering::default(); function.blocks.len()];
  let mut pre_order: Vec<u32> = Vec::new();
  let mut post_order: Vec<u32> = Vec::new();

  if collect_pre_order {
    pre_order.reserve(function.blocks.len());
  }
  if collect_post_order {
    post_order.reserve(function.blocks.len());
  }

  if function.blocks.is_empty() {
    return (ordering, pre_order, post_order);
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

        if collect_pre_order {
          pre_order.push(block_idx);
        }

        stack.push((child_idx, 0));
      }
    } else {
      ordering[block_idx as usize].post_order = next_post_order;
      next_post_order += 1;

      if collect_post_order {
        post_order.push(block_idx);
      }
    }
  }

  (ordering, pre_order, post_order)
}
