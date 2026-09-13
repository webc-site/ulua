//! Source: `Analysis/src/ControlFlowGraph.cpp:424-462` (hand-ported)
//! C++ `void CFGBuilder::emitRefineInstruction(Block* block, RefinementId refinement)`.
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  methods::refinement_arena_type_proposition::refinement_arena_type_proposition,
  records::{
    block::Block, cfg_builder::CfgBuilder, proposition_control_flow_graph::Proposition,
    refine::Refine,
  },
  type_aliases::{
    def_id_control_flow_graph::DefId,
    refinement_control_flow_graph::{Refinement, RefinementMember},
    refinement_id_control_flow_graph::RefinementId,
  },
};

impl CfgBuilder {
  /// # Safety
  /// 调用方须保证 `block` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn emit_refine_instruction(&mut self, block: *mut Block, refinement: RefinementId) {
    // C++ `Luau::visit(overloaded{...}, *refinement)` over the 4 variant kinds.
    // Scalar fields are copied out of the borrow before mutating `self`.
    let variant_index = unsafe { (*refinement).index() };

    match variant_index {
      // Proposition (variant index 3)
      3 => {
        let prop: &Proposition =
          <Proposition as RefinementMember>::get_if(unsafe { &*refinement }).unwrap();
        // DefId refined = newDefinition(prop.ptr->sym);
        let ptr = prop.ptr;
        let sym = unsafe { (*ptr).sym.clone() };
        let refined: DefId = self.new_definition(sym.clone());
        // emit<Refine>(block, refined, refinement);
        self.emit::<Refine, _>(block, (refined, refinement as *const Refinement));
        // block->setReachingDefinition(prop.ptr->sym, refined);
        unsafe { (*block).set_reaching_definition(sym, refined) };
      }
      // Conjunction (variant index 0)
      0 => {
        use crate::records::conjunction_control_flow_graph::Conjunction;
        let conj: &Conjunction =
          <Conjunction as RefinementMember>::get_if(unsafe { &*refinement }).unwrap();
        let (lhs, rhs) = (conj.lhs, conj.rhs);
        // emitRefineInstruction(block, conj.lhs); emitRefineInstruction(block, conj.rhs);
        unsafe { self.emit_refine_instruction(block, lhs) };
        unsafe { self.emit_refine_instruction(block, rhs) };
      }
      // Negation (variant index 2)
      2 => {
        use crate::records::negation_control_flow_graph::Negation;
        let neg: &Negation =
          <Negation as RefinementMember>::get_if(unsafe { &*refinement }).unwrap();
        // RefinementArena::negation pushes through And/Or via DeMorgan and cancels
        // double negation, so the only shape that reaches here is Negation(Proposition).
        // auto prop = neg.refinement->get_if<Proposition>();
        let prop_ref = unsafe { &*neg.refinement };
        let prop = <Proposition as RefinementMember>::get_if(prop_ref);
        // LUAU_ASSERT(prop != nullptr);
        LUAU_ASSERT!(prop.is_some());
        let prop = prop.unwrap();
        let (p_ptr, p_type, p_is_typeof, p_sense) =
          (prop.ptr, prop.r#type.clone(), prop.is_typeof, prop.sense);
        // emitRefineInstruction(block, allocator->refinementArena.typeProposition(
        //     prop->ptr, prop->type, prop->is_typeof, !prop->sense));
        let fresh = {
          let arena = unsafe { &mut (*self.allocator).refinement_arena };
          refinement_arena_type_proposition(arena, p_ptr, p_type, p_is_typeof, !p_sense)
        };
        unsafe { self.emit_refine_instruction(block, fresh) };
      }
      // Disjunction (variant index 1)
      // CLI-205330 tracks the work needed to handle Disjunctions; no-op for now.
      _ => {}
    }
  }
}
