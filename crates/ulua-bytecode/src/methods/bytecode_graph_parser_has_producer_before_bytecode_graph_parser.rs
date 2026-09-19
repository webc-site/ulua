use std::collections::HashSet;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{bc_block_edge_kind::BcBlockEdgeKind, bc_op_kind::BcOpKind},
  records::{
    bc_function::BcFunction, bc_op::BcOp, bc_op_hash::BcOpHash, block_producers::BlockProducers,
    bytecode_graph_parser::BytecodeGraphParser,
  },
  type_aliases::reg::Reg,
};

/// `has_producer_before` 的只读递归核。
///
/// 整个递归对图**只读**（仅写 visited 集合），因此把 `func`/`producers` 收成
/// 共享借用、`visited` 单独可变，回边迭代可以直接在 `predecessors` 切片上进行，
/// 不再需要旧实现「每层递归 clone 整条 ops/predecessors」绕 `&mut self` 借用。
fn has_producer_before_impl(
  func: &BcFunction,
  producers: &[BlockProducers],
  range_start: BcOp,
  range_end: BcOp,
  start_op: BcOp,
  reg: Reg,
  check_cached: bool,
  visited: &mut HashSet<BcOp, BcOpHash>,
) -> bool {
  LUAU_ASSERT!(start_op.kind == BcOpKind::Inst);
  visited.insert(range_end);
  LUAU_ASSERT!((range_end.index as usize) < producers.len());

  let block_producers = &producers[range_end.index as usize];
  if (reg as i32) > block_producers.invalid_after {
    return false;
  }

  if block_producers.multi_return.kind != BcOpKind::None && reg >= block_producers.multi_return_start
  {
    return true;
  }

  let block = &func.blocks[range_end.index as usize];

  if check_cached {
    if block_producers.own.contains_key(&reg) {
      return true;
    }
  } else {
    for op in block.ops.iter() {
      if *op == start_op {
        break;
      }
      if let Some(&op_reg) = func.regs.get(op)
        && op_reg == reg
      {
        return true;
      }
    }
  }

  if range_end == range_start {
    return false;
  }

  for edge in block.predecessors.iter() {
    if edge.kind == BcBlockEdgeKind::Loop || visited.contains(&edge.target) {
      continue;
    }
    if has_producer_before_impl(
      func,
      producers,
      range_start,
      edge.target,
      start_op,
      reg,
      true,
      visited,
    ) {
      return true;
    }
  }

  false
}

impl<'a> BytecodeGraphParser<'a> {
  pub fn has_producer_before_bc_op_bc_op_bc_op_reg_bool_unordered_set_bc_op_bc_op_hash(
    &mut self,
    range_start: BcOp,
    range_end: BcOp,
    start_op: BcOp,
    reg: Reg,
    check_cached: bool,
    visited: &mut HashSet<BcOp, BcOpHash>,
  ) -> bool {
    let BytecodeGraphParser {
      func, producers, ..
    } = self;
    has_producer_before_impl(
      func,
      producers,
      range_start,
      range_end,
      start_op,
      reg,
      check_cached,
      visited,
    )
  }
}
