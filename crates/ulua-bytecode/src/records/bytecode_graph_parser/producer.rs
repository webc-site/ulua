use std::vec::Vec;

use ulua_common::{
  collections::HashSet, macros::luau_assert::LUAU_ASSERT, records::small_vector::SmallVector,
};

use super::BytecodeGraphParser;
use crate::{
  enums::{bc_block_edge_kind::BcBlockEdgeKind, bc_op_kind::BcOpKind},
  records::{
    bc_function::BcFunction,
    bc_op::BcOp,
    bc_op_hash::BcOpHash,
    block_producers::{BlockProducers, PRODUCER_SENTINEL},
  },
  type_aliases::reg::Reg,
};
// ── 前驱生产者搜索的共用样板（find_producer / find_forward_producer_in_range 两族递归） ──

/// 前驱边 `(kind, target)` 快照：递归每层只消费这两个 Copy 字段，快照成栈上
/// SmallVector 后即释放 `func` 借用，深层递归可继续独占（避免整条边表的堆 clone）。
fn snapshot_predecessor_edges(
  func: &BcFunction<'_>,
  block: BcOp,
) -> SmallVector<(BcBlockEdgeKind, BcOp), 4> {
  func.blocks[block.index as usize]
    .predecessors
    .iter()
    .map(|e| (e.kind, e.target))
    .collect()
}

impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  /// 吸收一次递归命中到汇合候选集：Phi 结果展开为其全部操作数（先快照 Copy
  /// 句柄再逐个去重吸收），非 Phi 结果原样去重吸收——两族前驱递归里逐字重复的
  /// 展开段收口于此。
  fn absorb_producer_result(&mut self, op: BcOp, results: &mut SmallVector<BcOp, 4>) {
    if op.kind == BcOpKind::Phi {
      let projections: SmallVector<BcOp, 4> = self.func.phi_op(op).ops.iter().copied().collect();
      for proj in projections {
        if !results.contains(&proj) {
          results.push_back(proj);
        }
      }
    } else if !results.contains(&op) {
      results.push_back(op);
    }
  }

  /// 前驱汇合尾段：候选为空返回 `None`、单条原样返回、多条新建归属块 `owner`
  /// 的 Phi（操作数即去重后的候选集，对齐 cpp `makePhi(block, reg)`）。
  fn merge_producer_results(&mut self, owner: BcOp, results: SmallVector<BcOp, 4>) -> Option<BcOp> {
    match results.as_slice() {
      [] => None,
      [only] => Some(*only),
      _ => {
        let res = self.func.add_phi();
        self.func.block_op(owner).phis.push(res);
        let phi = self.func.phi_op(res);
        for &op in &results {
          phi.ops.push_back(op);
        }
        Some(res)
      }
    }
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_parser_find_forward_producer_in_range_visited.rs` ──
impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  pub(crate) fn find_forward_producer_in_range_visited(
    &mut self,
    range_start: BcOp,
    range_end: BcOp,
    start_op: BcOp,
    reg: Reg,
    visited: &mut HashSet<BcOp, BcOpHash>,
  ) -> Option<BcOp> {
    LUAU_ASSERT!(start_op.kind == BcOpKind::Inst);

    visited.insert(range_end);

    LUAU_ASSERT!(range_end.index < self.producers.len() as u32);
    let block_producers = &self.producers[range_end.index as usize];

    if reg as i32 > block_producers.invalid_after {
      return None;
    }

    if let Some(local) = block_producers.own.get(&reg) {
      return Some(*local);
    }

    if range_start == range_end {
      return None;
    }

    if block_producers.multi_return.kind != BcOpKind::None
      && reg >= block_producers.multi_return_start
    {
      return Some(block_producers.multi_return);
    }

    let predecessors = snapshot_predecessor_edges(self.func, range_end);

    // 栈上暂存前驱汇合值：块前驱通常仅 1~2 个，SmallVector 零堆分配
    let mut results: SmallVector<BcOp, 4> = SmallVector::new();

    for &(ctrl, pred) in predecessors.iter() {
      if ctrl == BcBlockEdgeKind::Loop || visited.contains(&pred) {
        continue;
      }

      LUAU_ASSERT!(range_end != pred);

      if let Some(op) =
        self.find_forward_producer_in_range_visited(range_start, pred, start_op, reg, visited)
      {
        self.absorb_producer_result(op, &mut results);
      }
    }

    self.merge_producer_results(range_end, results)
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_parser_find_forward_producer_in_range.rs` ──
impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  pub(crate) fn find_forward_producer_in_range(
    &mut self,
    range_start: BcOp,
    range_end: BcOp,
    start_op: BcOp,
    reg: Reg,
  ) -> Option<BcOp> {
    let mut visited: HashSet<BcOp, BcOpHash> = HashSet::default();
    self.find_forward_producer_in_range_visited(range_start, range_end, start_op, reg, &mut visited)
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_parser_find_producer_visited.rs` ──
impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  pub(crate) fn find_producer_visited(
    &mut self,
    block: BcOp,
    reg: Reg,
    visited: &mut HashSet<BcOp, BcOpHash>,
  ) -> Option<BcOp> {
    visited.insert(block);
    LUAU_ASSERT!(block.index < self.producers.len() as u32);
    let block_producers = &self.producers[block.index as usize];

    if (reg as i32) > block_producers.invalid_after {
      return None;
    }

    if let Some(local) = block_producers.own.get(&reg) {
      return Some(*local);
    }

    if let Some(cached) = block_producers.cached.get(&reg) {
      return Some(*cached);
    }

    if block_producers.multi_return.kind != BcOpKind::None
      && reg >= block_producers.multi_return_start
    {
      return Some(self.func.add_proj(
        block_producers.multi_return,
        (reg - block_producers.multi_return_start) as u32,
      ));
    }

    // 候选集手工去重吸收（BcOp 无 Ord，语义对齐 cpp 的 unordered_set 归并）；
    // 前驱快照与汇合尾段与 forward 族共用样板。
    let predecessors = snapshot_predecessor_edges(self.func, block);
    let mut results: SmallVector<BcOp, 4> = SmallVector::new();

    for &(ctrl, pred) in predecessors.iter() {
      if ctrl == BcBlockEdgeKind::Loop || visited.contains(&pred) {
        continue;
      }
      LUAU_ASSERT!(block != pred);

      if let Some(op) = self.find_producer_visited(pred, reg, visited) {
        self.absorb_producer_result(op, &mut results);
      }
    }

    let res = self.merge_producer_results(block, results)?;

    let block_producers = &mut self.producers[block.index as usize];
    block_producers.cached.insert(reg, res);
    Some(res)
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_parser_find_producer.rs` ──
impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  pub(crate) fn find_producer(&mut self, block: BcOp, reg: Reg) -> Option<BcOp> {
    let mut visited: HashSet<BcOp, BcOpHash> = HashSet::default();
    self.find_producer_visited(block, reg, &mut visited)
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_parser_find_producers_up_to_top.rs` ──
impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  pub(crate) fn find_producers_up_to_top(&mut self, block: BcOp, reg: Reg) -> Vec<BcOp> {
    // We assume it called only for search of var return calls.
    LUAU_ASSERT!(block.index < self.producers.len() as u32);

    let multi_return_start;
    let multi_return;
    {
      let block_producers = &self.producers[block.index as usize];
      LUAU_ASSERT!(block_producers.multi_return.kind == BcOpKind::Inst);
      multi_return_start = block_producers.multi_return_start;
      multi_return = block_producers.multi_return;
    }

    // So we need to find all producers from reg to blockProducers.multi_return_start.
    // cpp 用 `multiReturnStart - reg + 1` 预留容量；两个操作数都是 u8 且源自不可信
    // 字节码，差为负时在 Rust 里会 panic 或回绕成天文数字再触发巨量分配，故按
    // saturating 计算。容量只是提示，输出序列与 cpp 保持一致。
    let mut res = Vec::with_capacity(
      usize::from(multi_return_start)
        .saturating_sub(usize::from(reg))
        .saturating_add(1),
    );

    // r 是寄存器号（数据语义），用 `reg..multi_return_start` 区间迭代取代手工游标；
    // reg >= multi_return_start 时区间为空，与 cpp 的 while 判定一致。
    // cpp 原址只用 `LUAU_ASSERT` 守生产者命中，release 下把空 `BcRef` 塞进结果。
    // 寄存器区间来自不可信字节码，按 `add_vm_const_input` 先例以 `error` 位收口：
    // 找不到生产者即返回已收集部分并置位错误，由 `rebuild_graph` 整体失败。
    for r in reg..multi_return_start {
      let Some(static_reg_op) = self.find_producer(block, r) else {
        self.error = true;
        return res;
      };
      res.push(static_reg_op);
    }

    res.push(multi_return);

    // multireturn is consumed, clean it up
    let block_producers = &mut self.producers[block.index as usize];
    block_producers.multi_return = BcOp::new();
    block_producers.multi_return_start = PRODUCER_SENTINEL;

    res
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_parser_has_producer_before_visited.rs` ──
/// `has_producer_before` 的只读递归核。
///
/// 整个递归对图**只读**（仅写 visited 集合），因此把 `func`/`producers` 收成
/// 共享借用、`visited` 单独可变，回边迭代可以直接在 `predecessors` 切片上进行，
/// 不再需要旧实现「每层递归 clone 整条 ops/predecessors」绕 `&mut self` 借用。
fn has_producer_before_impl(
  func: &BcFunction<'_>,
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

  if block_producers.multi_return.kind != BcOpKind::None
    && reg >= block_producers.multi_return_start
  {
    return true;
  }

  let block = &func.blocks[range_end.index as usize];

  if check_cached {
    if block_producers.own.contains_key(&reg) {
      return true;
    }
  } else {
    // start_op 之前（不含）的 ops 中是否有写 reg 的生产者：迭代器链自带早停
    let hits = block
      .ops
      .iter()
      .take_while(|&&op| op != start_op)
      .any(|op| func.regs.get(op) == Some(&reg));
    if hits {
      return true;
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

impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  pub(crate) fn has_producer_before_visited(
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

// ── abs-r139：并自 `methods/bytecode_graph_parser_has_producer_before.rs` ──
impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  pub(crate) fn has_producer_before(
    &mut self,
    range_start: BcOp,
    range_end: BcOp,
    start_op: BcOp,
    reg: Reg,
  ) -> bool {
    let mut visited: HashSet<BcOp, BcOpHash> = HashSet::default();
    self.has_producer_before_visited(range_start, range_end, start_op, reg, false, &mut visited)
  }
}
