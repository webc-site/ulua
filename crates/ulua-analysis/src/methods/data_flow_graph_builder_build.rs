use core::ptr::null_mut;

use ulua_ast::records::ast_stat_block::AstStatBlock;
use ulua_common::{fflag, macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE};

use crate::{
  enums::scope_type::ScopeType,
  records::{
    arena_handle::Handle, data_flow_graph::DataFlowGraph,
    data_flow_graph_builder::DataFlowGraphBuilder, def_arena::DefArena, dfg_scope::DfgScope,
    internal_error_reporter::InternalErrorReporter, push_scope::PushScope,
    refinement_key_arena::RefinementKeyArena, symbol::Symbol,
  },
  type_aliases::{bindings::Bindings, def_id_def::DefId, props_data_flow_graph::Props},
};
impl DataFlowGraphBuilder {
  /// # Safety
  /// - `block` 须为非空、对齐且指向 arena 存活 `AstStatBlock` 的句柄，存活期覆盖整个构建过程；
  /// - `def_arena`、`key_arena` 为 arena 句柄（Handle 编码非空，构造名即断言
  ///   non-null），比返回的 DataFlowGraph 长寿；`handle` 为存活的
  ///   InternalErrorReporter 句柄或调用方约定的空值。
  pub unsafe fn build(
    block: *mut AstStatBlock,
    def_arena: Handle<DefArena>,
    key_arena: Handle<RefinementKeyArena>,
    handle: *mut InternalErrorReporter,
  ) -> DataFlowGraph {
    LUAU_TIMETRACE_SCOPE!("DataFlowGraphBuilder::build", "Typechecking");

    let mut builder = DataFlowGraphBuilder::data_flow_graph_builder_not_null_def_arena_not_null_refinement_key_arena(def_arena, key_arena);
    builder.handle = Handle::from_opt_ptr(handle);

    let module_scope = builder.scopes.push(DfgScope {
      parent: null_mut(),
      scope_type: ScopeType::Linear,
      bindings: Bindings::new(Symbol::default()),
      props: Props::new(DefId::NULL),
    });

    let _ps = PushScope::new(&mut builder.scope_stack, module_scope);
    // SAFETY: 本函数 Safety 契约保证 block 为 arena 存活非空 AstStatBlock，
    // 存活期覆盖整个构建过程且分析期只读。
    let block =
      unsafe { block.as_ref() }.expect("block 非空为 build() 的 Safety 契约，分析器产出了空模块块");
    let _ = builder.visit_block_without_child_scope(block);
    builder.resolve_captures();

    if fflag::DebugLuauFreezeArena.get() {
      // def_arena/key_arena 为 Handle：指向比 builder 长寿的 arena（构造期
      // not_null 契约保证），此处仅在其 allocator 上调用 safe 的 freeze()
      // （调试冻结），builder 为本函数独占的局部值，无并存借用。
      builder.def_arena.get_mut().allocator.freeze();
      builder.key_arena.get_mut().allocator.freeze();
    }

    // C++: `return std::move(builder.graph);` — move the graph out of the
    // owned builder (the rest of `builder` is dropped here).
    builder.graph
  }
}
