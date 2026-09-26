use core::{cmp, cmp::max, mem::take};
use std::vec::Vec;

use ulua_common::{
  collections::HashSet,
  enums::{luau_capture_type::LuauCaptureType, luau_opcode::LuauOpcode},
  fflag,
  functions::{
    get_jump_target::get_jump_target, get_op_length::get_op_length, is_fallthrough::is_fallthrough,
    is_fast_call::is_fast_call, is_loop_jump::is_loop_jump,
  },
  macros::luau_assert::LUAU_ASSERT,
  records::{
    dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet, instruction::Instruction,
    small_vector::SmallVector,
  },
};

use crate::{
  enums::{bc_block_edge_kind::BcBlockEdgeKind, bc_op_kind::BcOpKind},
  functions::decode_import_aux::decode_import_aux,
  records::{
    bc_block::BcBlock,
    bc_block_edge::BcBlockEdge,
    bc_function::BcFunction,
    bc_imm::BcImm,
    bc_op::BcOp,
    bc_op_hash::BcOpHash,
    block_producers::{BlockProducers, PRODUCER_SENTINEL},
    bytecode_builder::K_INVALID_REG,
    loop_info::LoopInfo,
  },
  type_aliases::reg::Reg,
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

// ── abs-r139：并自 `methods/bytecode_graph_parser_add_empty_input.rs` ──
impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  /// cpp `addEmptyInput(BcRef<BcInst>)`：占位输入，`kind` 为 `None`。
  pub(crate) fn add_empty_input(&mut self, inst: BcOp) {
    self
      .func
      .inst_op(inst)
      .ops
      .push_back(BcOp::with(BcOpKind::None, 0));
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_parser_add_jump_input.rs` ──
impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  /// cpp `addJumpInput(BcRef<BcInst>, int32_t)`：把跳转目标 PC 解析成块操作数。
  ///
  /// cpp 原址只用 `LUAU_ASSERT` 守 `blockByPC` 命中，release 下对 `end()` 解引用
  /// 是 UB。目标 PC 直接来自不可信字节码，按 `from_function_bytecode` 契约与
  /// `add_vm_const_input` 先例以 `error` 位收口：pc 未登记即不挂输入、置位错误。
  pub(crate) fn add_jump_input(&mut self, inst: BcOp, target: i32) {
    let inst_op = self.func.inst(inst).operator_deref().op;
    LUAU_ASSERT!(!is_fast_call(inst_op));
    if target < 0 {
      LUAU_ASSERT!(inst_op == LuauOpcode::LOP_LOADB);
      return;
    }
    let target = target as u32;
    let Some(bc_op) = self.block_by_pc.find(&target).copied() else {
      self.error = true;
      return;
    };
    self.func.inst_op(inst).ops.push(bc_op);
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_parser_add_producer.rs` ──
impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  pub(crate) fn add_producer(&mut self, reg: Reg, op: BcOp) {
    let block_producers: &mut BlockProducers =
      &mut self.producers[self.current_block.index as usize];

    block_producers.own.insert(reg, op);

    self.func.regs.insert(op, reg);

    block_producers.invalid_after = max(reg as i32, block_producers.invalid_after);
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_parser_add_proto_input.rs` ──
impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  /// cpp `addProtoInput(BcRef<BcInst>, uint32_t)`。
  pub(crate) fn add_proto_input(&mut self, inst: BcOp, idx: u32) {
    self
      .func
      .inst_op(inst)
      .ops
      .push_back(BcOp::with(BcOpKind::VmProto, idx));
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_parser_add_successor.rs` ──
impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  pub(crate) fn add_successor(&mut self, from_op: BcOp, to_op: BcOp, kind: BcBlockEdgeKind) {
    let from: &mut BcBlock = self.func.block_op(from_op);
    from.successors.push_back(BcBlockEdge {
      kind,
      target: to_op,
    });

    let to: &mut BcBlock = self.func.block_op(to_op);
    to.predecessors.push_back(BcBlockEdge {
      kind,
      target: from_op,
    });
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_parser_add_to_phi.rs` ──
impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  pub(crate) fn add_to_phi(&mut self, block: BcOp, op: BcOp, proj: BcOp) -> BcOp {
    if op.kind == BcOpKind::Phi {
      let phi = self.func.phi_op(op);
      if phi.ops.contains(&proj) {
        return op;
      }
      phi.ops.push_back(proj);
      op
    } else {
      let res = self.func.add_phi();
      // phi 归属于合并发生的目标块（对齐 cpp `makePhi(block, reg)`）
      self.func.block_op(block).phis.push(res);
      let phi = self.func.phi_op(res);
      phi.ops = SmallVector::from_iter([op, proj]);
      res
    }
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_parser_add_upval_input.rs` ──
impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  /// cpp `addUpvalInput(BcRef<BcInst>, uint32_t)`。
  pub(crate) fn add_upval_input(&mut self, inst: BcOp, idx: u32) {
    LUAU_ASSERT!(idx < u32::from(self.func.nups));
    self
      .func
      .inst_op(inst)
      .ops
      .push_back(BcOp::with(BcOpKind::VmUpvalue, idx));
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_parser_add_vm_const_input.rs` ──
impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  /// cpp `addVmConstInput(BcRef<BcInst>, uint32_t)`。
  ///
  /// cpp 原址（`BytecodeGraphParser.h:429-433`）只用 `LUAU_ASSERT` 守索引，
  /// release 下把越界索引塞进图、延后到常量取读处才炸。输入是不可信字节码，
  /// 这里按 `from_function_bytecode` 的契约以 `error` 位收口：越界即不挂输入、
  /// 置位错误，`rebuild_graph` 检测到后整体返回 `None`。
  pub(crate) fn add_vm_const_input(&mut self, inst: BcOp, idx: u32) {
    if idx as usize >= self.func.constants.len() {
      self.error = true;
      return;
    }
    self
      .func
      .inst_op(inst)
      .ops
      .push_back(BcOp::with(BcOpKind::VmConst, idx));
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_parser_add_vm_reg_input.rs` ──
impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  /// cpp `addVmRegInput(BcRef<BcInst>, Reg)`：解析该寄存器在当前块里的生产者并挂为输入。
  ///
  /// cpp 原址末尾只用 `LUAU_ASSERT` 守生产者命中，release 下把空 `BcRef` 塞进
  /// 输入延后炸。寄存器编号源自不可信字节码，按 `add_vm_const_input` 先例以
  /// `error` 位收口：找不到生产者即不挂输入、置位错误，由 `rebuild_graph` 整体失败。
  pub(crate) fn add_vm_reg_input(&mut self, inst: BcOp, reg: Reg) {
    let source = self.find_producer(self.current_block, reg);
    if source.is_none() && is_unreachable(self.func, self.current_block) {
      self
        .func
        .inst_op(inst)
        .ops
        .push_back(BcOp::with(BcOpKind::VmReg, reg as u32));
      return;
    }
    let Some(source) = source else {
      self.error = true;
      return;
    };
    self.func.inst_op(inst).ops.push_back(source);
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_parser_apply_call.rs` ──
impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  /// cpp `applyCall(BlockProducers&, BcOp, Reg, int)`（`BytecodeGraphParser.h:366-396`）。
  ///
  /// 不读取解析器自身状态，故为无接收者的关联函数：调用方可直接
  /// `&mut self.producers[..]`，不必再用裸指针绕开借用检查。
  pub(crate) fn apply_call(
    producers: &mut BlockProducers,
    call_op: BcOp,
    target_reg: Reg,
    nresults: i32,
  ) {
    producers.own.retain(|&reg, _| reg < target_reg);
    producers.cached.retain(|&reg, _| reg < target_reg);

    if nresults < 0 {
      producers.multi_return = call_op;
      producers.multi_return_start = target_reg;
      producers.invalid_after = PRODUCER_SENTINEL as i32;
    } else {
      producers.invalid_after = (target_reg as i32) - 1 + nresults;
    }
  }
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

// ── abs-r139：并自 `methods/bytecode_graph_parser_is_jump_trampoline.rs` ──
impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  pub(crate) fn is_jump_trampoline(&self, pc: u32, code: &[Instruction]) -> bool {
    let pc = pc as usize;
    if pc >= code.len() || code[pc].opcode() != Some(LuauOpcode::LOP_JUMP) {
      return false;
    }

    if pc + 1 >= code.len() {
      return false;
    }

    if code[pc + 1].opcode() != Some(LuauOpcode::LOP_JUMPX) {
      return false;
    }

    if pc + 2 >= code.len() {
      return false;
    }

    let target = get_jump_target(code[pc + 2].raw(), (pc + 2) as u32) as u32;
    target == (pc + 1) as u32
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

// ── abs-r139：并自 `methods/bytecode_graph_parser_make_block.rs` ──
impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  pub(crate) fn make_block(&mut self, pc: u32) -> BcOp {
    let new_block_op = self.func.add_block();
    *self.block_by_pc.get_or_insert(pc) = new_block_op;
    let new_block = self.func.block_op(new_block_op);
    new_block.sortkey = pc;
    new_block_op
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_parser_push_imm_input.rs` ──
impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  /// cpp `addImmInput` 三个重载（对应 `BytecodeGraphParser.h:402-427`）的单一源：
  /// 每个立即数操作数独占一条_imm 记录、索引取 `len - 1`，**不按值复用**——
  /// 复用会让同一指令的多个操作数共享记录，`setFbSlot` 之类改写连带污染另一个。
  /// 变体（即 kind）与载荷随 `imm` 参数由各入口注入（`BcImm` 为带载荷 enum）。
  /// 追加与索引包装委托 `BcFunction::add_imm_value` 单源。
  pub(crate) fn push_imm_input(&mut self, inst: BcOp, imm: BcImm) {
    let op = self.func.add_imm_value(imm);
    self.func.inst_op(inst).ops.push_back(op);
  }

  /// cpp `addImmInput(BcRef<BcInst>, bool)`（`BytecodeGraphParser.h:402-409`）：
  /// Boolean 入口，独占记录不复用的理由见 `push_imm_input`（此前的 `position()`
  /// 去重会让同一指令的多个 bool 操作数共享记录，改写其一时连带污染另一个）。
  pub(crate) fn add_imm_input_bc_inst_bool(&mut self, inst: BcOp, value: bool) {
    self.push_imm_input(inst, BcImm::Boolean(value));
  }

  /// cpp `addImmInput(BcRef<BcInst>, int32_t)`（`BytecodeGraphParser.h:411-418`）：
  /// Int 入口，独占记录不复用的理由见 `push_imm_input`（同一指令的 paramCount /
  /// returnCount / fbSlot 可能取值相同，复用会让后续改写连带改掉另一个操作数）。
  pub(crate) fn add_imm_input_bc_inst_i32(&mut self, inst: BcOp, value: i32) {
    self.push_imm_input(inst, BcImm::Int(value));
  }

  /// cpp `addImmInput(BcRef<BcInst>, uint32_t)`（`BytecodeGraphParser.h:420-427`）：
  /// Import 型入口，独占记录不复用的理由见 `push_imm_input`。
  pub(crate) fn add_imm_input_bc_inst_u32(&mut self, inst: BcOp, value: u32) {
    self.push_imm_input(inst, BcImm::Import(value));
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_parser_rebuild_blocks.rs` ──
impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  pub(crate) fn rebuild_blocks(&mut self, code: &[Instruction]) -> usize {
    let entry_block = self.make_block(0);
    self.func.entry_block = entry_block;
    // 出口块无起始 pc，cpp 用 kBlockNoStartPc 作哨兵
    let exit_block = self.make_block(BcBlock::K_BLOCK_NO_START_PC);
    self.func.exit_block = exit_block;
    let codesize = code.len() as u32;
    let mut i: u32 = 0;
    let mut current_block = entry_block;
    let mut instruction_count: usize = 0;

    while i < codesize {
      let insn = code[i as usize];
      let op = insn.luau_opcode();
      // cpp 88-91：目标落在 JUMPX 上时穿透一层，取 JUMPX 自身的跳转目标
      // （JUMP→JUMPX trampoline 的场景）。此前的移植计算了穿透结果却未赋回。
      // cpp 直接索引（契约保证目标在界内）；此处越界视为无目标以避免 panic。
      let mut target = get_jump_target(insn.raw(), i);
      if target >= 0 {
        let t = target as usize;
        if t < code.len() && code[t].opcode() == Some(LuauOpcode::LOP_JUMPX) {
          target = get_jump_target(code[t].raw(), t as u32);
        }
      }

      let needs_block = target >= 0
        && !is_fast_call(op)
        && op != LuauOpcode::LOP_JUMPX
        && !self.is_jump_trampoline(i, code);
      if needs_block {
        // 单次哈希查找取号：键命中直接用；未命中则 make_block（其内部 get_or_insert
        // 已把新块登记进表）。旧实现先 contains_key 再在 if 后 get(..).unwrap() 二次
        // 哈希，此处合并后连 unwrap 一并消除。
        let target_block_op = if let Some(&existing) = self.block_by_pc.get(&(target as u32)) {
          existing
        } else {
          let new_block_op = self.make_block(target as u32);
          if (target as u32) < i {
            // We are jumping back.
            // The new block was created in the middle of the existing one.
            // We need to maintain predecessor/successor relations.
            let mut block_start_pc = target as u32 - 1;
            while !self.block_by_pc.contains_key(&block_start_pc) && block_start_pc != 0 {
              block_start_pc -= 1;
            }
            LUAU_ASSERT!(self.block_by_pc.contains_key(&block_start_pc));
            // unwrap 100% 安全：循环以「命中键」或「block_start_pc==0」终止，而入口块
            // 已由 make_block(0) 注册进表，故终止时键必在。
            let prev_block_op = *self.block_by_pc.get(&block_start_pc).unwrap();
            // 先遍历旧块后继修正其前驱回指，再整体移交（mem::take 免拷贝），
            // 新块继承全部旧后继、旧块只保留指向新块的 fallthrough。
            // 后继表先快照出来：回指改写要同时可变异标目标块，索引式遍历会撞借用冲突。
            let prev_successors: Vec<BcBlockEdge> = self.func.blocks[prev_block_op.index as usize]
              .successors
              .iter()
              .copied()
              .collect();
            for edge in prev_successors {
              let target_block = self.func.block_op(edge.target);
              for back_edge in target_block.predecessors.iter_mut() {
                if back_edge.target == prev_block_op {
                  back_edge.target = new_block_op;
                }
              }
            }
            self.func.blocks[new_block_op.index as usize].successors =
              take(&mut self.func.blocks[prev_block_op.index as usize].successors);
            self.add_successor(prev_block_op, new_block_op, BcBlockEdgeKind::Fallthrough);
          }
          new_block_op
        };
        let edge_kind = if is_loop_jump(op) {
          BcBlockEdgeKind::Loop
        } else {
          BcBlockEdgeKind::Branch
        };
        self.add_successor(current_block, target_block_op, edge_kind);
      }
      if op == LuauOpcode::LOP_RETURN {
        self.add_successor(current_block, exit_block, BcBlockEdgeKind::Fallthrough);
      }
      let op_len = get_op_length(op) as u32;
      i += op_len;
      // 单次 get 取代 contains_key + get 的双重哈希；make_block 已把 i 注册进表
      let block_at_i = if needs_block || (op == LuauOpcode::LOP_RETURN && i < codesize) {
        if let Some(&existing) = self.block_by_pc.get(&i) {
          Some(existing)
        } else {
          Some(self.make_block(i))
        }
      } else {
        self.block_by_pc.get(&i).copied()
      };

      if let Some(next_block) = block_at_i {
        if is_fallthrough(op) {
          self.add_successor(current_block, next_block, BcBlockEdgeKind::Fallthrough);
        }
        current_block = next_block;
      }
      instruction_count += 1;
    }
    instruction_count
  }
}

// ── abs-r139：并自 `methods/bytecode_graph_parser_rebuild_graph.rs` ──
impl<'a, 'f> BytecodeGraphParser<'a, 'f> {
  /// 跳转类指令的图节点构建（cpp `parseJump`）。独立于主循环，避免每次迭代重建闭包。
  ///
  /// 节点以 `BcOp` 标识（对齐 cpp `BcRef<BcInst>` 的"指令句柄"角色），每次写入都经
  /// `BcFunction::inst_op` 现取现用，不再跨调用持有裸指针。
  fn parse_jump(
    &mut self,
    op: LuauOpcode,
    jump_target: i32,
    insn: Instruction,
    aux: Instruction,
    node_op: BcOp,
  ) {
    self.func.inst_op(node_op).op = op;
    match op {
      LuauOpcode::LOP_JUMPXEQKNIL => {
        self.add_vm_reg_input(node_op, insn.a());
        self.add_imm_input_bc_inst_bool(node_op, aux.aux_not() != 0);
        self.add_jump_input(node_op, jump_target);
      }
      LuauOpcode::LOP_JUMPXEQKB => {
        self.add_vm_reg_input(node_op, insn.a());
        self.add_imm_input_bc_inst_bool(node_op, aux.aux_not() != 0);
        self.add_jump_input(node_op, jump_target);
        self.add_imm_input_bc_inst_bool(node_op, aux.aux_kb() != 0);
      }
      LuauOpcode::LOP_JUMPXEQKN | LuauOpcode::LOP_JUMPXEQKS => {
        self.add_vm_reg_input(node_op, insn.a());
        self.add_imm_input_bc_inst_bool(node_op, aux.aux_not() != 0);
        self.add_jump_input(node_op, jump_target);
        self.add_vm_const_input(node_op, aux.aux_kv());
      }
      LuauOpcode::LOP_JUMPIF | LuauOpcode::LOP_JUMPIFNOT => {
        self.add_vm_reg_input(node_op, insn.a());
        self.add_jump_input(node_op, jump_target);
      }
      LuauOpcode::LOP_JUMPIFEQ
      | LuauOpcode::LOP_JUMPIFLE
      | LuauOpcode::LOP_JUMPIFLT
      | LuauOpcode::LOP_JUMPIFNOTEQ
      | LuauOpcode::LOP_JUMPIFNOTLE
      | LuauOpcode::LOP_JUMPIFNOTLT => {
        self.add_vm_reg_input(node_op, insn.a());
        self.add_vm_reg_input(node_op, aux.aux_a());
        self.add_jump_input(node_op, jump_target);
      }
      LuauOpcode::LOP_FORNPREP => {
        // forg loop protocol: A, A+1, A+2 are used for iteration protocol; A+3, ... are loop variables
        self.add_vm_reg_input(node_op, insn.a());
        self.add_vm_reg_input(node_op, insn.a().wrapping_add(1));
        self.add_vm_reg_input(node_op, insn.a().wrapping_add(2));
        self.add_jump_input(node_op, jump_target);
        self.func.regs.insert(node_op, insn.a());
        for (proj_idx, reg) in [
          (0u32, insn.a() as u32),
          (1, insn.a() as u32 + 1),
          (2, insn.a() as u32 + 2),
        ] {
          let proj = self.func.add_proj(node_op, proj_idx);
          self.add_producer(reg as u8, proj);
        }
      }
      LuauOpcode::LOP_FORNLOOP => {
        self.add_vm_reg_input(node_op, insn.a());
        self.add_vm_reg_input(node_op, insn.a().wrapping_add(1));
        self.add_vm_reg_input(node_op, insn.a().wrapping_add(2));
        self.add_jump_input(node_op, jump_target);
      }
      _ => {
        LUAU_ASSERT!(false);
      }
    }
  }

  /// KS/NAMECALL 族的 aux 域两种编码（镜像序列化侧 `emit_ks_aux` 的解析对偶）：
  /// UDATA 变体拆「低 16 位 KV16 常量 + 高 16 位 SLOT imm」
  /// （cpp BytecodeGraphParser.h:666-691/712-720），普通变体整字是常量索引。
  fn add_ks_aux_input(&mut self, node_op: BcOp, udata: bool, aux: Instruction) {
    if udata {
      self.add_vm_const_input(node_op, aux.aux_kv16() as u32);
      self.add_imm_input_bc_inst_i32(node_op, aux.aux_slot() as i32);
    } else {
      self.add_vm_const_input(node_op, aux.raw());
    }
  }

  /// 由字节码重建 SSA 图，并返回「指令下标 → 图内 `BcInst` 序号」映射（`insns_pc`）。
  /// 建图失败（块数超限、不可信输入越界、`error` 位）返回 `None`。
  pub(crate) fn rebuild_graph(&mut self, code: &[Instruction], lines: &[u32]) -> Option<Vec<u32>> {
    let codesize = code.len() as u32;
    let instructions_count = self.rebuild_blocks(code);
    if self.block_by_pc.size() > Self::K_MAX_CFG_BLOCKS as usize {
      return None;
    }

    let mut loops: Vec<LoopInfo> = Vec::new();

    self
      .producers
      .resize(self.func.blocks.len(), Default::default());
    // 每条指令占一格，初值 0；下方按 pc 逐格填入指令序号，故预铺满整段。
    let mut pcs = vec![0u32; codesize as usize];

    self.current_block = self.func.entry_block;

    // i 是入口参数寄存器号：既是 producers 表的键，又被原样编进 `BcOp::VmReg` 句柄，
    // 属图数据而非容器游标，故保留数值范围遍历。
    for i in 0..self.func.numparams {
      self.add_producer(i, BcOp::with(BcOpKind::VmReg, i as u32));
    }

    // Create instructions.
    self.current_block = self.func.entry_block;
    self.func.instructions.reserve(instructions_count);

    let mut i: u32 = 0;
    while i < codesize {
      let insn = code[i as usize];
      // op_length/aux 声明为 mut：JUMP trampoline 分支会按合并后的落点指令
      // 重赋值（对齐 cpp BytecodeGraphParser.h:784-785），否则 2 字长条件跳转
      // 合并后循环底仍按 LOP_JUMP 的 1 推进，AUX 词被误解析成伪指令节点
      let op = insn.luau_opcode();
      let mut op_length = get_op_length(op) as u32;
      let aux = if op_length > 1 && i + 1 < codesize {
        code[(i + 1) as usize]
      } else {
        Instruction(0)
      };
      let node_op = self.func.add_inst();
      self
        .func
        .block_op(self.current_block)
        .append_instruction(node_op);
      {
        let node = self.func.inst_op(node_op);
        node.block = self.current_block;
        if (i as usize) < lines.len() {
          node.line = lines[i as usize];
        }
        node.op = op;
      }

      pcs[i as usize] = node_op.index;

      match op {
        LuauOpcode::LOP_NOP | LuauOpcode::LOP_BREAK | LuauOpcode::LOP_NATIVECALL => {}

        LuauOpcode::LOP_FASTPCALL => {
          self.add_imm_input_bc_inst_i32(node_op, insn.a() as i32);
          self.add_imm_input_bc_inst_i32(node_op, insn.b() as i32);
          // 第三个 imm 承载指向 CALL 的原 C 域（跳转偏移），序列化时原样回填
          self.add_imm_input_bc_inst_i32(node_op, insn.c() as i32);
        }

        LuauOpcode::LOP_NEWCLASS => {
          let b = insn.b();
          if (b as u32) != K_INVALID_REG {
            self.add_vm_reg_input(node_op, b);
          } else {
            self.add_empty_input(node_op);
          }
          self.add_imm_input_bc_inst_u32(node_op, insn.c() as u32);
          self.add_vm_const_input(node_op, aux.raw());
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_LOADNIL => {
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_LOADB => {
          self.add_imm_input_bc_inst_bool(node_op, insn.b() != 0);
          self.add_jump_input(node_op, insn.jump_target(i));
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_LOADN => {
          self.add_imm_input_bc_inst_i32(node_op, insn.d() as i32);
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_LOADK => {
          self.add_vm_const_input(node_op, insn.d() as u32);
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_MOVE => {
          self.add_vm_reg_input(node_op, insn.b());
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_GETGLOBAL => {
          self.add_imm_input_bc_inst_i32(node_op, insn.c() as i32);
          self.add_vm_const_input(node_op, aux.raw());
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_SETGLOBAL => {
          self.add_vm_reg_input(node_op, insn.a());
          self.add_imm_input_bc_inst_i32(node_op, insn.c() as i32);
          self.add_vm_const_input(node_op, aux.raw());
        }

        LuauOpcode::LOP_GETUPVAL => {
          self.add_upval_input(node_op, insn.b() as u32);
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_SETUPVAL => {
          self.add_vm_reg_input(node_op, insn.a());
          self.add_upval_input(node_op, insn.b() as u32);
        }

        LuauOpcode::LOP_CLOSEUPVALS => {
          self
            .func
            .inst_op(node_op)
            .ops
            .push_back(BcOp::with(BcOpKind::VmReg, insn.a() as u32));
        }

        LuauOpcode::LOP_GETIMPORT => {
          self.add_vm_const_input(node_op, insn.d() as u32);
          // aux 高 2 位为组件数，每 10 位为一个导入组件的常量索引
          let (components_count, components) = decode_import_aux(aux.raw());
          self.add_imm_input_bc_inst_i32(node_op, components_count as i32);
          for &component in &components[..components_count.min(components.len() as u32) as usize] {
            self.add_vm_const_input(node_op, component);
          }
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_SETTABLE => {
          self.add_vm_reg_input(node_op, insn.a());
          self.add_vm_reg_input(node_op, insn.b());
          self.add_vm_reg_input(node_op, insn.c());
        }

        LuauOpcode::LOP_GETUDATAKS | LuauOpcode::LOP_GETTABLEKS => {
          self.add_vm_reg_input(node_op, insn.b());
          self.add_imm_input_bc_inst_i32(node_op, insn.c() as i32);
          self.add_ks_aux_input(node_op, op == LuauOpcode::LOP_GETUDATAKS, aux);
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_SETUDATAKS | LuauOpcode::LOP_SETTABLEKS => {
          self.add_vm_reg_input(node_op, insn.a());
          self.add_vm_reg_input(node_op, insn.b());
          self.add_imm_input_bc_inst_i32(node_op, insn.c() as i32);
          self.add_ks_aux_input(node_op, op == LuauOpcode::LOP_SETUDATAKS, aux);
        }

        LuauOpcode::LOP_GETTABLEN => {
          self.add_vm_reg_input(node_op, insn.b());
          self.add_imm_input_bc_inst_i32(node_op, (insn.c() as i32) + 1);
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_SETTABLEN => {
          self.add_vm_reg_input(node_op, insn.a());
          self.add_vm_reg_input(node_op, insn.b());
          self.add_imm_input_bc_inst_i32(node_op, (insn.c() as i32) + 1);
        }

        LuauOpcode::LOP_NEWCLOSURE => {
          self.add_proto_input(node_op, insn.d() as u32);
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_NAMECALLUDATA | LuauOpcode::LOP_NAMECALL => {
          self.add_vm_reg_input(node_op, insn.b());
          self.add_imm_input_bc_inst_i32(node_op, insn.c() as i32);
          self.add_ks_aux_input(node_op, op == LuauOpcode::LOP_NAMECALLUDATA, aux);
          self.func.regs.insert(node_op, insn.a());
          // A 与 A+1 两个寄存器各挂一个 proj（与 FORNPREP 同模式）
          for (proj_idx, reg) in [(0u32, insn.a() as u32), (1, insn.a() as u32 + 1)] {
            let proj = self.func.add_proj(node_op, proj_idx);
            self.add_producer(reg as u8, proj);
          }
        }

        LuauOpcode::LOP_CALL | LuauOpcode::LOP_CALLFB => {
          let nparams = insn.b() as i32 - 1;
          let nresults = insn.c() as i32 - 1;
          self.add_imm_input_bc_inst_i32(node_op, nparams);
          self.add_imm_input_bc_inst_i32(node_op, nresults);
          if op == LuauOpcode::LOP_CALLFB {
            self.add_imm_input_bc_inst_i32(node_op, aux.raw() as i32);
          }

          // Call target.
          self.add_vm_reg_input(node_op, insn.a());
          // Fixed arguments.
          for j in 1..=nparams {
            self.add_vm_reg_input(node_op, (insn.a() as i32 + j) as u8);
          }

          if nparams < 0 {
            let producers_up_to_top =
              self.find_producers_up_to_top(self.current_block, insn.a().wrapping_add(1));
            for inp in producers_up_to_top {
              self.func.inst_op(node_op).ops.push_back(inp);
            }
          }

          let current_block_idx = self.current_block.index as usize;
          BytecodeGraphParser::apply_call(
            &mut self.producers[current_block_idx],
            node_op,
            insn.a(),
            nresults,
          );

          self.func.regs.insert(node_op, insn.a());
          // j 是结果投影号（编进 `BcOp::Proj` 的 index），同时是相对基寄存器 A 的偏移；
          // nresults<0（变长调用）时范围为空，与原语义一致。
          for j in 0..nresults {
            let proj = self.func.add_proj(node_op, j as u32);
            self.add_producer((insn.a() as i32 + j) as u8, proj);
          }
        }

        LuauOpcode::LOP_RETURN => {
          let nresults = insn.b() as i32 - 1;
          self.add_imm_input_bc_inst_i32(node_op, nresults);
          // j 是第 j 个返回值寄存器相对基址 A 的偏移（寄存器号即数据）；nresults<0 时为空。
          for j in 0..nresults {
            self.add_vm_reg_input(node_op, (insn.a() as i32 + j) as u8);
          }
          if nresults < 0 {
            let producers_up_to_top =
              self.find_producers_up_to_top(self.current_block, insn.a());
            for inp in producers_up_to_top {
              self.func.inst_op(node_op).ops.push_back(inp);
            }
          }
          if nresults == 0 {
            self
              .func
              .inst_op(node_op)
              .ops
              .push_back(BcOp::with(BcOpKind::VmReg, insn.a() as u32));
          }
        }

        LuauOpcode::LOP_JUMP => {
          if self.is_jump_trampoline(i, code) {
            // it is long jump trampoline
            let long_offset = code[(i + 1) as usize].e();
            i += get_op_length(LuauOpcode::LOP_JUMP) as u32
              + get_op_length(LuauOpcode::LOP_JUMPX) as u32;
            // `is_jump_trampoline` 只保证 pc+1/pc+2 在界内，推进后的 `i` 仍可能越过
            // 码尾（不可信输入）；受检访问，越界即放弃建图而不是 panic。
            let next_insn = *code.get(i as usize)?;
            let next_op = next_insn.luau_opcode();
            let next_op_length = get_op_length(next_op) as u32;
            let next_aux = if next_op_length > 1 && i + 1 < codesize {
              code[(i + 1) as usize]
            } else {
              Instruction(0)
            };
            // 回写 op_length/aux（cpp BytecodeGraphParser.h:784-785 同步重赋
            // 值）：底部按合并后指令的长度推进，2 字长条件跳转的 AUX 词不再
            // 被误解析成伪指令。op 不回写——cpp 的 loop 判定在 needsBlock 建
            // 边处（:122，用当轮原始 op），trampoline 轮的 JUMP 非 loop 跳转；
            // Rust 底部 LoopInfo 登记同理按物理指令判定，合并 op 会令
            // is_loop_jump 查到尚未建块的目标 pc 而 misset
            op_length = next_op_length;
            self.parse_jump(
              next_op,
              (i as i32) + long_offset,
              next_insn,
              next_aux,
              node_op,
            );
          } else {
            self.add_jump_input(node_op, insn.jump_target(i));
          }
        }

        LuauOpcode::LOP_JUMPBACK => {
          // repeat .. until loops use it for back edge.
          self.add_jump_input(node_op, insn.jump_target(i));
        }

        LuauOpcode::LOP_JUMPXEQKNIL
        | LuauOpcode::LOP_JUMPXEQKB
        | LuauOpcode::LOP_JUMPXEQKN
        | LuauOpcode::LOP_JUMPXEQKS
        | LuauOpcode::LOP_JUMPIF
        | LuauOpcode::LOP_JUMPIFNOT
        | LuauOpcode::LOP_JUMPIFEQ
        | LuauOpcode::LOP_JUMPIFLE
        | LuauOpcode::LOP_JUMPIFLT
        | LuauOpcode::LOP_JUMPIFNOTEQ
        | LuauOpcode::LOP_JUMPIFNOTLE
        | LuauOpcode::LOP_JUMPIFNOTLT
        | LuauOpcode::LOP_FORNPREP
        | LuauOpcode::LOP_FORNLOOP => {
          self.parse_jump(op, insn.jump_target(i), insn, aux, node_op);
        }

        LuauOpcode::LOP_ADD
        | LuauOpcode::LOP_SUB
        | LuauOpcode::LOP_MUL
        | LuauOpcode::LOP_DIV
        | LuauOpcode::LOP_MOD
        | LuauOpcode::LOP_POW
        | LuauOpcode::LOP_AND
        | LuauOpcode::LOP_OR
        // 与算术族同形：B/C 寄存器输入 + A 产出（GETTABLE/IDIV 逐字节同体）
        | LuauOpcode::LOP_GETTABLE
        | LuauOpcode::LOP_IDIV => {
          self.add_vm_reg_input(node_op, insn.b());
          self.add_vm_reg_input(node_op, insn.c());
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_ADDK
        | LuauOpcode::LOP_SUBK
        | LuauOpcode::LOP_MULK
        | LuauOpcode::LOP_DIVK
        | LuauOpcode::LOP_MODK
        | LuauOpcode::LOP_POWK
        | LuauOpcode::LOP_ANDK
        | LuauOpcode::LOP_ORK
        // 与 K 常量族同形：B 寄存器 + C 常量输入 + A 产出（IDIVK 逐字节同体）
        | LuauOpcode::LOP_IDIVK => {
          self.add_vm_reg_input(node_op, insn.b());
          self.add_vm_const_input(node_op, insn.c() as u32);
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_CONCAT => {
          LUAU_ASSERT!(insn.b() <= insn.c());
          for param in insn.b()..=insn.c() {
            self.add_vm_reg_input(node_op, param);
          }
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_NOT | LuauOpcode::LOP_MINUS | LuauOpcode::LOP_LENGTH => {
          self.add_vm_reg_input(node_op, insn.b());
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_NEWTABLE => {
          self.add_imm_input_bc_inst_i32(node_op, insn.b() as i32);
          self.add_imm_input_bc_inst_i32(node_op, aux.raw() as i32);
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_DUPTABLE => {
          self.add_producer(insn.a(), node_op);
          self.add_vm_const_input(node_op, insn.d() as u32);
        }

        LuauOpcode::LOP_SETLIST => {
          let count = insn.c() as i32 - 1;
          self.add_imm_input_bc_inst_i32(node_op, aux.raw() as i32);
          self.add_imm_input_bc_inst_i32(node_op, count);
          self.add_vm_reg_input(node_op, insn.a());
          // param 是第 param 个待写入元素的寄存器偏移（基址取 B），寄存器号即数据；
          // count<0（变长尾参）时范围为空，改走下面的 findProducersUpToTop。
          for param in 0..count {
            self.add_vm_reg_input(node_op, (insn.b() as i32 + param) as u8);
          }
          if count < 0 {
            let producers_up_to_top =
              self.find_producers_up_to_top(self.current_block, insn.b());
            for inp in producers_up_to_top {
              self.func.inst_op(node_op).ops.push_back(inp);
            }
          }
        }

        LuauOpcode::LOP_FORGPREP
        | LuauOpcode::LOP_FORGPREP_NEXT
        | LuauOpcode::LOP_FORGPREP_INEXT => {
          self.add_vm_reg_input(node_op, insn.a());
          self.add_vm_reg_input(node_op, insn.a().wrapping_add(1));
          self.add_vm_reg_input(node_op, insn.a().wrapping_add(2));
          let loop_insn_pc = insn.jump_target(i);
          self.add_jump_input(node_op, loop_insn_pc);
          // 跳转目标来自不可信输入：cpp 只在 `LUAU_ASSERT` 里守这一步，release 下
          // `code[loopInsnPc]` / `code[loopInsnPc + 1]` 就是越界读。这里改成受检访问，
          // 条件不满足就整体放弃建图（`rebuild_graph` 返回 `None`），
          // 原断言降级为 debug 契约保留。
          let loop_pc = usize::try_from(loop_insn_pc).ok()?;
          let (Some(&loop_insn), Some(&forgloop_insn)) = (code.get(loop_pc), code.get(loop_pc + 1))
          else {
            return None;
          };
          let loop_insn_op = loop_insn.luau_opcode();
          LUAU_ASSERT!(loop_insn_op == LuauOpcode::LOP_FORGLOOP);
          let vars = forgloop_insn.aux_a() as i32;
          self.func.regs.insert(node_op, insn.a());
          // idx 是迭代协议三元组之后的第 idx 个投影号（`2 + idx` 编进 `BcOp::Proj`），
          // 同时给出对应的循环变量寄存器偏移——两个都是数据，不是容器游标。
          for idx in 0..=cmp::max(vars, 2) {
            let proj = self.func.add_proj(node_op, (2 + idx) as u32);
            self.add_producer((insn.a() as i32 + 2 + idx) as u8, proj);
          }
        }

        LuauOpcode::LOP_FORGLOOP => {
          self.add_vm_reg_input(node_op, insn.a());
          self.add_vm_reg_input(node_op, insn.a().wrapping_add(1));
          self.add_vm_reg_input(node_op, insn.a().wrapping_add(2));
          self.add_imm_input_bc_inst_bool(node_op, aux.aux_not() != 0);
          let vars = aux.aux_a() as i32;
          self.add_imm_input_bc_inst_i32(node_op, vars);
          self.add_jump_input(node_op, insn.jump_target(i));
        }

        // FASTCALL 族五变体同骨架：首尾 imm 相同（A 域 + 指向 CALL 的 C 域偏移，
        // 序列化时原样回填）；中段实参按变体递增——1 加 B 寄存器、2/3 追加 aux
        // 低/高字节寄存器、2K 换 aux 常量（cpp BytecodeGraphParser.h:823-871）。
        LuauOpcode::LOP_FASTCALL
        | LuauOpcode::LOP_FASTCALL1
        | LuauOpcode::LOP_FASTCALL2
        | LuauOpcode::LOP_FASTCALL2K
        | LuauOpcode::LOP_FASTCALL3 => {
          self.add_imm_input_bc_inst_i32(node_op, insn.a() as i32);
          if op != LuauOpcode::LOP_FASTCALL {
            self.add_vm_reg_input(node_op, insn.b());
            if matches!(op, LuauOpcode::LOP_FASTCALL2 | LuauOpcode::LOP_FASTCALL3) {
              self.add_vm_reg_input(node_op, aux.aux_a());
            }
            if op == LuauOpcode::LOP_FASTCALL2K {
              self.add_vm_const_input(node_op, aux.raw());
            }
            if op == LuauOpcode::LOP_FASTCALL3 {
              self.add_vm_reg_input(node_op, aux.aux_b());
            }
          }
          self.add_imm_input_bc_inst_i32(node_op, insn.c() as i32);
        }

        LuauOpcode::LOP_GETVARARGS => {
          self
            .func
            .inst_op(node_op)
            .ops
            .push_back(BcOp::with(BcOpKind::VmReg, insn.a() as u32));
          let count = insn.b() as i32 - 1;
          self.add_imm_input_bc_inst_i32(node_op, count);
          self.func.regs.insert(node_op, insn.a());
          if count < 0 {
            let block_producers = &mut self.producers[self.current_block.index as usize];
            block_producers.multi_return = node_op;
            block_producers.multi_return_start = insn.a();
            block_producers.invalid_after = PRODUCER_SENTINEL as i32;
          } else {
            // j 是第 j 个变参返回值的投影号（编进 `BcOp::Proj`），兼相对 A 的寄存器偏移。
            for j in 0..count {
              let proj = self.func.add_proj(node_op, j as u32);
              self.add_producer((insn.a() as i32 + j) as u8, proj);
            }
          }
        }

        LuauOpcode::LOP_DUPCLOSURE => {
          self.add_vm_const_input(node_op, insn.d() as u32);
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_PREPVARARGS => {
          self.add_imm_input_bc_inst_i32(node_op, insn.a() as i32);
        }

        LuauOpcode::LOP_LOADKX => {
          self.add_vm_const_input(node_op, aux.raw());
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_JUMPX => {
          LUAU_ASSERT!(false);
          self.add_jump_input(node_op, insn.jump_target(i));
        }

        LuauOpcode::LOP_COVERAGE => {
          self.add_imm_input_bc_inst_i32(node_op, insn.e());
        }

        LuauOpcode::LOP_CAPTURE => {
          let capture_type = insn.a() as u32;
          self.add_imm_input_bc_inst_i32(node_op, capture_type as i32);
          if capture_type == LuauCaptureType::LCT_VAL as u32
            || capture_type == LuauCaptureType::LCT_REF as u32
          {
            self.add_vm_reg_input(node_op, insn.b());
          } else {
            self.add_upval_input(node_op, insn.b() as u32);
          }
          self.add_imm_input_bc_inst_i32(node_op, insn.c() as i32);
        }

        LuauOpcode::LOP_SUBRK | LuauOpcode::LOP_DIVRK => {
          self.add_vm_const_input(node_op, insn.b() as u32);
          self.add_vm_reg_input(node_op, insn.c());
          self.add_producer(insn.a(), node_op);
        }

        LuauOpcode::LOP_CMPPROTO => {
          self.add_vm_reg_input(node_op, insn.a());
          self.add_imm_input_bc_inst_i32(node_op, aux.raw() as i32);
          self.add_jump_input(node_op, insn.jump_target(i));
        }

        LuauOpcode::LOP_NEWCLASSMEMBER => {
          LUAU_ASSERT!(fflag::DebugLuauUserDefinedClasses.get());
          self.add_vm_reg_input(node_op, insn.a());
          self.add_vm_reg_input(node_op, insn.c());
          self.add_vm_const_input(node_op, aux.raw());
        }

        LuauOpcode::LOP__COUNT => {
          LUAU_ASSERT!(false);
        }
      }

      if is_loop_jump(op) {
        let target = insn.jump_target(i);
        // `rebuildBlocks` 会为每条跳转指令在目标 pc 建块，正常输入必然命中；
        // 但 target 由不可信字节码算出，cpp 的 `LUAU_ASSERT` 在 release 下是 no-op，
        // 紧跟的 `unwrap()` 就成了 panic 路径。改为受检查找，查不到即放弃建图。
        let &entry = u32::try_from(target)
          .ok()
          .and_then(|t| self.block_by_pc.get(&t))?;
        loops.push(LoopInfo {
          entry,
          exit: self.current_block,
        });
      }

      i += op_length;
      // 单次 get 取代 contains_key + get 的双重哈希
      if let Some(&block) = self.block_by_pc.get(&i) {
        self.current_block = block;
      }

      // 不可信输入在 `add_vm_const_input` 等处只置 `error` 位（cpp 是 release
      // no-op 的 LUAU_ASSERT）：逐条指令收尾检查，置位即放弃建图返回 false。
      if self.error {
        return None;
      }
    }

    // visited/queue 提升为循环外复用（参照 `sccp_visit` 的 scratch 约定）：
    // 每个 loop 只 clear 不重分配，避免多层循环嵌套时的逐轮堆分配。
    let mut visited: HashSet<BcOp, BcOpHash> = HashSet::default();
    let mut queue: Vec<BcOp> = Vec::new();
    // 前驱/块内指令/指令操作数三级快照同样循环外复用：元素全 Copy，
    // 每轮 clear 后重填（替代旧实现逐块 collect VecDeque + 双重 clone）。
    let mut pred_snap: SmallVector<(BcBlockEdgeKind, BcOp), 4> = SmallVector::new();
    let mut ops_snap: Vec<BcOp> = Vec::new();
    let mut inst_ops: SmallVector<BcOp, 4> = SmallVector::new();
    for loop_ in &loops {
      visited.clear();
      queue.clear();
      queue.push(loop_.exit);
      while let Some(cur) = queue.pop() {
        if visited.contains(&cur) {
          continue;
        }
        visited.insert(cur);
        {
          let bl = self.func.block_op(cur);
          pred_snap.clear();
          pred_snap.extend(bl.predecessors.iter().map(|e| (e.kind, e.target)));
          ops_snap.clear();
          ops_snap.extend(bl.ops.iter().copied());
        }

        for op in &ops_snap {
          inst_ops.clear();
          inst_ops.extend(self.func.inst(*op).operator_deref().ops.iter().copied());
          for (inp_idx, &inp) in inst_ops.as_slice().iter().enumerate() {
            let Some(reg) = self.func.regs.get(&inp).copied() else {
              continue;
            };
            // try to find it in the same loop before
            if self.has_producer_before(loop_.entry, cur, *op, reg) {
              continue;
            }
            if let Some(forward_input) =
              self.find_forward_producer_in_range(cur, loop_.exit, *op, reg)
            {
              let op_val = self.func.inst(*op).operator_deref().ops[inp_idx];
              let new_val = self.add_to_phi(cur, op_val, forward_input);
              self.func.inst_op(*op).ops[inp_idx] = new_val;
              self.func.regs.insert(new_val, reg);
            }
          }
        }

        for &(ctrl, pred) in &pred_snap {
          if ctrl != BcBlockEdgeKind::Loop && !visited.contains(&pred) {
            queue.push(pred);
          }
        }
      }
    }
    // 图重建成功：交回「pc → 指令序号」映射。
    Some(pcs)
  }
}
