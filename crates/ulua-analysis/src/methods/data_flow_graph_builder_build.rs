use core::ptr::{null, null_mut};

use ulua_ast::records::ast_stat_block::AstStatBlock;
use ulua_common::{FFlag, macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE};

use crate::{
  enums::scope_type::ScopeType,
  records::{
    data_flow_graph::DataFlowGraph, data_flow_graph_builder::DataFlowGraphBuilder,
    def_arena::DefArena, dfg_scope::DfgScope, internal_error_reporter::InternalErrorReporter,
    push_scope::PushScope, refinement_key_arena::RefinementKeyArena, symbol::Symbol,
  },
  type_aliases::{bindings::Bindings, props_data_flow_graph::Props},
};
impl DataFlowGraphBuilder {
  /// # Safety
  /// 调用方须保证 `block、`def_arena、`key_arena、`handle` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn build(
    block: *mut AstStatBlock,
    def_arena: *mut DefArena,
    key_arena: *mut RefinementKeyArena,
    handle: *mut InternalErrorReporter,
  ) -> DataFlowGraph {
    LUAU_TIMETRACE_SCOPE!("DataFlowGraphBuilder::build", "Typechecking");

    let mut builder = DataFlowGraphBuilder::data_flow_graph_builder_not_null_def_arena_not_null_refinement_key_arena(def_arena, key_arena);
    builder.handle = handle;

    let module_scope = builder.scopes.push(DfgScope {
      parent: null_mut(),
      scope_type: ScopeType::Linear,
      bindings: Bindings::new(Symbol::default()),
      props: Props::new(null()),
    });

    let _ps = PushScope::new(&mut builder.scope_stack, module_scope);
    let _ = unsafe { builder.visit_block_without_child_scope(block) };
    builder.resolve_captures();

    if FFlag::DebugLuauFreezeArena.get() {
      unsafe {
        (*builder.def_arena).allocator.freeze();
        (*builder.key_arena).allocator.freeze();
      }
    }

    // C++: `return std::move(builder.graph);` — move the graph out of the
    // owned builder (the rest of `builder` is dropped here).
    builder.graph
  }
}
