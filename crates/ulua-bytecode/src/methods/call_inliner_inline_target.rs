use alloc::vec::Vec;
use core::{cmp, mem};

use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::{bc_block_edge_kind::BcBlockEdgeKind, bc_op_kind::BcOpKind},
  records::{bc_block_edge::BcBlockEdge, bc_op::BcOp, call_inliner::CallInliner},
  type_aliases::bc_edges::BcEdges,
};

/// cpp `CallInliner::kMaxInlinerCombinedStackSize`。
const K_MAX_INLINER_COMBINED_STACK_SIZE: u8 = 250;

/// 把 `edges` 中首条 fallthrough 边重定向到 `target`；无则追加。
fn upsert_fallthrough(edges: &mut BcEdges, target: BcOp) {
  for edge in edges.iter_mut() {
    if edge.kind == BcBlockEdgeKind::Fallthrough {
      edge.target = target;
      return;
    }
  }
  edges.push_back(BcBlockEdge {
    kind: BcBlockEdgeKind::Fallthrough,
    target,
  });
}

impl<'a> CallInliner<'a> {
  /// cpp `inlineTarget(uint32_t targetProtoId)`：把 `target` 图内联进 `caller`。
  ///
  /// 全程只在图之间传 `BcOp` 句柄；任何一次读写都经 `self.caller` / `self.target`
  /// 现取现用，不再持有跨可变操作的 `BcRef` 视图（旧实现在 `&Vec<T>` 与
  /// `&mut BcFunction` 之间互造裸指针，属 Stacked Borrows UB）。
  pub fn inline_target(&mut self, target_proto_id: u32) -> bool {
    let mut new_max_stack_size: u32 =
      u32::from(self.caller.maxstacksize) + u32::from(self.target.maxstacksize);

    if self.target.is_vararg {
      // cpp `newMaxStackSize += std::max(uint8_t(callParams.size()), target.numparams)`：
      // 实参少于形参时，缺口由 fillUnderCallArguments 用 LOADNIL 补齐，
      // 栈预算必须按 numparams 起算，否则映射后的寄存器会越过 maxstacksize。
      new_max_stack_size = new_max_stack_size.wrapping_add(u32::from(cmp::max(
        self.call_params.len() as u8,
        self.target.numparams,
      )));
    }

    if new_max_stack_size >= u32::from(K_MAX_INLINER_COMBINED_STACK_SIZE) {
      return false;
    }

    let mut call = self.call_view();
    if call.param_count() < 0 || call.return_count() < 0 {
      return false;
    }

    // inlining of upvalues is not supported yet
    if self.target.nups > 0 {
      return false;
    }

    self.caller.maxstacksize = new_max_stack_size as u8;

    let (prev_block_op, next_block_op) = self.split_block_on_op(self.call_op);

    let mut target_op: BcOp = self.call_view().target();

    // cpp：prevBlock 末尾若是 NAMECALL，先就地换成 MOVE + GETTABLEKS
    if let Some(last_op) = self.caller.block_op(prev_block_op).ops.back().copied()
      && self.caller.inst_op(last_op).op == LuauOpcode::LOP_NAMECALL
    {
      target_op = self.replace_namecall(last_op, prev_block_op);
    }

    self.append_cmp_proto(prev_block_op, target_op, target_proto_id);

    // Seal FB slot of inlined call：内联成功后回退路径上的 CALLFB 不再需要 feedback 槽位。
    self.call_view().set_fb_slot(-1);

    self.allocate_graph_entities_for_target();

    self.fill_under_call_arguments();

    self.find_target_call_projections();

    if !self.migrate_blocks(next_block_op) {
      return false;
    }

    let caller_inlined_entry_op = self.map_block_op(self.target.entry_block);

    // Remove prevBlock fallthrough to call block from its predecessors
    let call_block = self.call_block_op();
    {
      let insn_preds: &mut BcEdges = &mut self.caller.block_op(call_block).predecessors;
      *insn_preds = insn_preds
        .iter()
        .filter(|p| !(p.kind == BcBlockEdgeKind::Fallthrough && p.target == prev_block_op))
        .copied()
        .collect();
    }

    upsert_fallthrough(
      &mut self.caller.block_op(prev_block_op).successors,
      caller_inlined_entry_op,
    );
    upsert_fallthrough(
      &mut self.caller.block_op(caller_inlined_entry_op).predecessors,
      prev_block_op,
    );

    self.migrate_block_phis();

    // cpp `for (auto& [callerBlockOp, targetReturnOp] : returnSites) replaceReturn(...)`：
    // 延后到此处，保证 replaceGetVarArg 已把 varArgMoves 填好，
    // 返回 phi 里的变参投影才能解析到对应的 MOVE/LOADNIL。
    for (caller_block_op, target_return_op) in mem::take(&mut self.return_sites) {
      if !self.replace_return(next_block_op, caller_block_op, target_return_op) {
        return false;
      }
    }

    self.migrate_instructions();

    self.replace_call_usages_with_return_phis();

    self.drop_prep_var_args_in_inlined_path();

    // 多返回值聚合 phi 锚定到汇合块（cpp inline 尾部）
    let return_phis: Vec<BcOp> = self
      .return_ops
      .iter()
      .copied()
      .filter(|op| op.kind == BcOpKind::Phi)
      .collect();
    for ret_op in return_phis {
      self.caller.block_op(next_block_op).phis.push(ret_op);
    }

    LUAU_ASSERT!(self.validate_cfg());

    true
  }
}
