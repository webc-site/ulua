use core::cmp::max;

use ulua_common::{
  enums::luau_opcode::LuauOpcode, functions::is_fast_call::is_fast_call,
  macros::luau_assert::LUAU_ASSERT, records::small_vector::SmallVector,
};

use super::{BytecodeGraphParser, is_unreachable};
use crate::{
  enums::{bc_block_edge_kind::BcBlockEdgeKind, bc_op_kind::BcOpKind},
  records::{
    bc_block::BcBlock,
    bc_block_edge::BcBlockEdge,
    bc_op::BcOp,
    block_producers::{BlockProducers, PRODUCER_SENTINEL},
  },
  type_aliases::reg::Reg,
};
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
