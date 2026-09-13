use ulua_common::{enums::luau_opcode::LuauOpcode, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::bc_block_edge_kind::BcBlockEdgeKind,
  records::{bc_block_edge::BcBlockEdge, bc_op::BcOp, call_inliner::CallInliner},
  type_aliases::bc_edges::BcEdges,
};

const K_MAX_INLINER_COMBINED_STACK_SIZE: u8 = 250;

impl<'a> CallInliner<'a> {
  pub fn inline_target(&mut self, target_proto_id: u32) -> bool {
    let mut new_max_stack_size: u32 =
      self.caller.maxstacksize as u32 + self.target.maxstacksize as u32;

    if self.target.is_vararg {
      new_max_stack_size = new_max_stack_size.wrapping_add(self.call_params.len() as u32);
    }

    if new_max_stack_size >= K_MAX_INLINER_COMBINED_STACK_SIZE as u32 {
      return false;
    }

    if self.call.param_count() < 0 || self.call.return_count() < 0 {
      return false;
    }

    // inlining of upvalues is not supported yet
    if self.target.nups > 0 {
      return false;
    }

    self.caller.maxstacksize = new_max_stack_size as u8;

    let (mut prev_block, mut next_block) = self.split_block_on_op(self.call.op());

    let mut target_op: BcOp = self.call.target();

    if !prev_block.operator_deref().ops.is_empty() {
      let last_inst = self
        .caller
        .inst(prev_block.operator_deref().ops.back().copied().unwrap());
      if last_inst.operator_deref().op == LuauOpcode::LOP_NAMECALL {
        target_op = self.replace_namecall(last_inst.op, &mut prev_block);
      }
    }

    self.append_cmp_proto(&mut prev_block, target_op, target_proto_id);

    self.allocate_graph_entities_for_target();

    self.fill_under_call_arguments();

    self.find_target_call_projections();

    if !self.migrate_blocks(&mut next_block) {
      return false;
    }

    let caller_inlined_entry_op = self.map_block_op(self.target.entry_block);

    // Remove prevBlock fallthrough to call block from its predecessors
    let call_block = self.call.base.operator_deref().block;
    let mut call_block_ref = self.caller.block(call_block);
    let insn_preds: &mut BcEdges = &mut call_block_ref.operator_deref_mut().predecessors;

    let prev_block_op = prev_block.op;

    let retained: Vec<_> = insn_preds
      .iter()
      .cloned()
      .filter(|p| !(p.kind == BcBlockEdgeKind::Fallthrough && p.target == prev_block_op))
      .collect();

    insn_preds.clear();
    for edge in retained {
      insn_preds.push_back(edge);
    }

    {
      let edges = &mut prev_block.operator_deref_mut().successors;
      let mut found = false;
      for edge in edges.iter_mut() {
        if edge.kind == BcBlockEdgeKind::Fallthrough {
          edge.target = caller_inlined_entry_op;
          found = true;
          break;
        }
      }
      if !found {
        edges.push_back(BcBlockEdge {
          kind: BcBlockEdgeKind::Fallthrough,
          target: caller_inlined_entry_op,
        });
      }
    }
    {
      let edges = &mut self.caller.blocks[caller_inlined_entry_op.index as usize].predecessors;
      let mut found = false;
      for edge in edges.iter_mut() {
        if edge.kind == BcBlockEdgeKind::Fallthrough {
          edge.target = prev_block.op;
          found = true;
          break;
        }
      }
      if !found {
        edges.push_back(BcBlockEdge {
          kind: BcBlockEdgeKind::Fallthrough,
          target: prev_block.op,
        });
      }
    }

    self.migrate_instructions();

    self.replace_call_usages_with_return_phis();

    self.drop_prep_var_args_in_inlined_path();

    LUAU_ASSERT!(self.validate_cfg());

    true
  }
}
