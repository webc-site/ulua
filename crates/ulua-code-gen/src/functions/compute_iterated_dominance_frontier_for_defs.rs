use crate::{
  functions::{dom_children::dom_children, successors::successors},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    block_and_ordering::BlockAndOrdering, idf_context::IdfContext, ir_function::IrFunction,
  },
};

pub fn compute_iterated_dominance_frontier_for_defs(
  ctx: &mut IdfContext,
  function: &IrFunction,
  def_blocks: &[u32],
  live_in_blocks: &[u32],
) {
  CODEGEN_ASSERT!(!function.cfg.dom_ordering.is_empty());

  CODEGEN_ASSERT!(ctx.queue.is_empty());
  CODEGEN_ASSERT!(ctx.worklist.is_empty());

  ctx.idf.clear();

  ctx.visits.clear();
  ctx.visits.resize(function.blocks.len(), Default::default());

  for &def_block in def_blocks.iter() {
    let ordering = function.cfg.dom_ordering[def_block as usize];
    ctx.queue.push(BlockAndOrdering {
      block_idx: def_block,
      ordering,
    });
  }

  // cpp 原语 `while (!queue.empty()) { root = queue.back(); queue.pop_back(); }` 的惯用等价：
  // 以 while-let 消 pop().unwrap()，逐轮取队尾，行为相同。
  while let Some(root) = ctx.queue.pop() {
    CODEGEN_ASSERT!(ctx.worklist.is_empty());
    ctx.worklist.push(root.block_idx);
    ctx.visits[root.block_idx as usize].seen_in_worklist = true;

    while let Some(block_idx) = ctx.worklist.pop() {
      // 检查后继是否恰为当前 root 支配结束处的节点，即支配 frontier 成员
      for succ_idx in successors(&function.cfg, block_idx) {
        let succ_ordering = function.cfg.dom_ordering[succ_idx as usize];

        // root 的 DF 中节点的 level 总不高于 root 自身的 level
        if succ_ordering.depth > root.ordering.depth {
          continue;
        }

        if ctx.visits[succ_idx as usize].seen_in_queue {
          continue;
        }

        ctx.visits[succ_idx as usize].seen_in_queue = true;

        // 若后继 block 不以本变量为 live in，跳过
        if !live_in_blocks.contains(&succ_idx) {
          continue;
        }

        ctx.idf.push(succ_idx);

        // 若 block 没有自己的该变量定义，加入队列
        if !def_blocks.contains(&succ_idx) {
          ctx.queue.push(BlockAndOrdering {
            block_idx: succ_idx,
            ordering: succ_ordering,
          });
        }
      }

      // 把尚未处理的支配树孩子加入工作列表
      for dom_child_idx in dom_children(&function.cfg, block_idx) {
        if ctx.visits[dom_child_idx as usize].seen_in_worklist {
          continue;
        }

        ctx.visits[dom_child_idx as usize].seen_in_worklist = true;
        ctx.worklist.push(dom_child_idx);
      }
    }
  }
}
