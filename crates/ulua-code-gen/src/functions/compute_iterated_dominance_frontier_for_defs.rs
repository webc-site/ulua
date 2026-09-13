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

  while !ctx.queue.is_empty() {
    let root = ctx.queue.pop().unwrap();
    CODEGEN_ASSERT!(ctx.worklist.is_empty());
    ctx.worklist.push(root.block_idx);
    ctx.visits[root.block_idx as usize].seen_in_worklist = true;

    while let Some(block_idx) = ctx.worklist.pop() {
      // Check if successor node is the node where dominance of the current root ends, making it a part of dominance frontier set
      for succ_idx in successors(&function.cfg, block_idx) {
        let succ_ordering = function.cfg.dom_ordering[succ_idx as usize];

        // Nodes in the DF of root always have a level that is less than or equal to the level of root
        if succ_ordering.depth > root.ordering.depth {
          continue;
        }

        if ctx.visits[succ_idx as usize].seen_in_queue {
          continue;
        }

        ctx.visits[succ_idx as usize].seen_in_queue = true;

        // Skip successor block if it doesn't have our variable as a live in there
        if !live_in_blocks.contains(&succ_idx) {
          continue;
        }

        ctx.idf.push(succ_idx);

        // If block doesn't have its own definition of the variable, add it to the queue
        if !def_blocks.contains(&succ_idx) {
          ctx.queue.push(BlockAndOrdering {
            block_idx: succ_idx,
            ordering: succ_ordering,
          });
        }
      }

      // Add dominance tree children that haven't been processed yet to the worklist
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
