use std::vec::Vec;

use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet};

use crate::{
  enums::bc_block_edge_kind::BcBlockEdgeKind,
  records::{
    bc_function::BcFunction, bc_op::BcOp, bc_op_hash::BcOpHash, block_producers::BlockProducers,
  },
};

#[derive(Debug)]
pub(crate) struct BytecodeGraphParser<'a, 'f> {
  pub(crate) func: &'a mut BcFunction<'f>,
  pub(crate) block_by_pc: DenseHashMap<u32, BcOp>,
  pub(crate) producers: Vec<BlockProducers>,
  pub(crate) current_block: BcOp,
  /// 不可信输入的错误位：越界常量索引等在 release 下不走断言，置位后由
  /// `rebuild_graph` 统一以 `None` 收口（与 `K_MAX_CFG_BLOCKS` 失败路径同级）。
  pub(crate) error: bool,
}

impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  pub(crate) const K_MAX_CFG_BLOCKS: u32 = 1000;
}

// ── abs-r139：并自 `methods/bytecode_graph_parser_bytecode_graph_parser.rs` ──
impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  /// cpp `BytecodeGraphParser::BytecodeGraphParser(BcFunction& fn)`
  pub fn new(func: &'a mut BcFunction<'f>) -> Self {
    Self {
      func,
      // key 哨兵取 `u32::MAX - 1`：出口块以 `K_BLOCK_NO_START_PC`（`u32::MAX`）
      // 作 pc 插入本表，哨兵必须避开它；真实 pc 触到 `MAX-1` 需要 ≥16GB 指令流
      // 输入，远超读入路径 `has_room_for(codesize, 4)` 的可达上限，不会相撞。
      block_by_pc: DenseHashMap::new(u32::MAX - 1),
      producers: Vec::new(),
      current_block: BcOp::new(),
      error: false,
    }
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_parser_is_unreachable.rs` ──
/// 判定块是否（不经环边）从 entry 不可达。整个递归对图**只读**，因此收
/// `&BcFunction` 而非 `&mut BytecodeGraphParser`：回边迭代直接走
/// `predecessors` 切片，不再需要旧实现「每层递归 clone 整条 VecDeque」
/// 绕 `&mut self` 借用（拆分形态参考 `has_producer_before_impl`）。
pub fn is_unreachable(func: &BcFunction<'_>, block_op: BcOp) -> bool {
  // visited 用 crate 内统一的 DenseHashSet（开放寻址 + 位图占用）：块句柄是
  // 8 字节 Copy、每块至多入集一次，std HashSet 的链式分配在这条编译期路径上是纯开销。
  !reachable_from_entry(func, block_op, &mut DenseHashSet::new(BcOp::new()))
}

/// 前驱链上是否存在通往 entry 的非 Loop 路径。
///
/// 不可信字节码可构造出未被标成 `Loop` 的回边（`rebuild_blocks` 只把
/// JUMPBACK/FORGLOOP/FORNLOOP 三种指令的边标 Loop，伪造的 JUMPIF 自指等
/// 会以 Branch/Fallthrough 形态成环），旧实现的朴素递归在环上不终止。
/// visited 守卫把已在前驱链上的块视作"此路不通"：这等价于只搜索**简单
/// 路径**，而 entry 可达性存在简单路径即为可达，结论不受影响。
fn reachable_from_entry(
  func: &BcFunction<'_>,
  block_op: BcOp,
  visited: &mut DenseHashSet<BcOp, BcOpHash>,
) -> bool {
  if block_op == func.entry_block {
    return true;
  }
  if !visited.try_insert(block_op) {
    return false;
  }

  // BcRef 临时值借 func 共享，for 头部表达式存活期覆盖循环体，递归可继续用 func
  for pred in &func.block(block_op).operator_deref().predecessors {
    if pred.kind != BcBlockEdgeKind::Loop && reachable_from_entry(func, pred.target, visited) {
      return true;
    }
  }

  false
}

mod inputs;
mod producer;
mod rebuild_blocks;
mod rebuild_graph;
mod trampoline_block;
