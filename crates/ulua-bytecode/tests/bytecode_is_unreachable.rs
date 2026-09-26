use ulua_bytecode::{
  enums::{bc_block_edge_kind::BcBlockEdgeKind, bc_op_kind::BcOpKind},
  is_unreachable,
  records::{bc_block::BcBlock, bc_block_edge::BcBlockEdge, bc_function::BcFunction, bc_op::BcOp},
};

fn block_op(idx: u32) -> BcOp {
  BcOp::with(BcOpKind::Block, idx)
}

fn edge(kind: BcBlockEdgeKind, target: BcOp) -> BcBlockEdge {
  BcBlockEdge { kind, target }
}

/// entry(0) 加 n 个空块的最小函数骨架。
fn func_with_blocks(n: usize) -> BcFunction<'static> {
  let mut func = BcFunction::default();
  func.blocks = vec![BcBlock::default(); n];
  func.entry_block = block_op(0);
  func
}

/// entry(0) → a(1)，b(2) 的唯一前驱是自身：模拟不可信字节码里 JUMPIF
/// 自指等构造出的**非 Loop 回边**（`rebuild_blocks` 只给三种循环指令标
/// Loop）。旧实现在此无限递归栈溢出；守卫后 b 判不可达、a 判可达。
#[test]
fn non_loop_self_backedge_terminates() {
  let mut func = func_with_blocks(3);
  func.blocks[1]
    .predecessors
    .push_back(edge(BcBlockEdgeKind::Branch, block_op(0)));
  func.blocks[2]
    .predecessors
    .push_back(edge(BcBlockEdgeKind::Branch, block_op(2)));

  assert!(
    is_unreachable(&func, block_op(2)),
    "自环前驱不能自证可达，b 应判不可达"
  );
  assert!(
    !is_unreachable(&func, block_op(1)),
    "entry 直达的 a 应判可达"
  );
}

/// x(1) ↔ y(2) 互相以非 Loop 边指认，且 x 另有 entry 前驱：环上有真实
/// 入口时仍须判可达——visited 只跳过环上重复展开，不吞掉外部路径。
#[test]
fn non_loop_cycle_with_entry_reaches_through() {
  let mut func = func_with_blocks(3);
  func.blocks[1]
    .predecessors
    .push_back(edge(BcBlockEdgeKind::Branch, block_op(0)));
  func.blocks[1]
    .predecessors
    .push_back(edge(BcBlockEdgeKind::Fallthrough, block_op(2)));
  func.blocks[2]
    .predecessors
    .push_back(edge(BcBlockEdgeKind::Branch, block_op(1)));

  assert!(
    !is_unreachable(&func, block_op(2)),
    "y 经环外前驱 x 可达 entry，须判可达"
  );
  assert!(!is_unreachable(&func, block_op(1)));
}

/// Loop 边照旧被忽略：只有 Loop 前驱的块判不可达（原有语义）。
#[test]
fn loop_only_predecessor_still_unreachable() {
  let mut func = func_with_blocks(2);
  func.blocks[1]
    .predecessors
    .push_back(edge(BcBlockEdgeKind::Loop, block_op(0)));

  assert!(is_unreachable(&func, block_op(1)));
}
