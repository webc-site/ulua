//! `data_flow_graph_builder` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_ast::{
  enums::ast_type_pack_ref::AstTypePackRef,
  functions::optional_node::node_ref,
  records::{
    ast_array::AstArray, ast_generic_type::AstGenericType,
    ast_generic_type_pack::AstGenericTypePack, ast_stat_block::AstStatBlock, ast_type::AstType,
    ast_type_list::AstTypeList, ast_type_pack::AstTypePack,
    ast_type_pack_explicit::AstTypePackExplicit, ast_type_pack_variadic::AstTypePackVariadic,
  },
};
use ulua_common::{
  fflag,
  macros::{luau_assert::LUAU_ASSERT, luau_timetrace_scope::LUAU_TIMETRACE_SCOPE},
  records::{dense_hash_map::DenseHashMap, dense_hash_table::DenseDefault},
};

use crate::{
  enums::{control_flow::ControlFlow, scope_type::ScopeType},
  functions::{arena_ref::arena_ref, collect_operands::collect_operands_into},
  records::{
    arena_handle::{Handle, alias_opt},
    data_flow_graph::DataFlowGraph,
    data_flow_graph_builder::DataFlowGraphBuilder,
    def_arena::DefArena,
    def_registry::def_as_mut,
    dfg_scope::DfgScope,
    function_capture::FunctionCapture,
    internal_error_reporter::InternalErrorReporter,
    phi::Phi,
    push_scope::PushScope,
    refinement_key_arena::RefinementKeyArena,
    symbol::Symbol,
  },
  type_aliases::{
    bindings::Bindings, def_id_def::DefId, props_data_flow_graph::Props, scope_stack::ScopeStack,
  },
};

impl DataFlowGraphBuilder {
  /// # Safety
  /// - `block` 须为非空、对齐且指向 arena 存活 `AstStatBlock` 的句柄，存活期覆盖整个构建过程；
  /// - `def_arena`、`key_arena` 为 arena 句柄（Handle 编码非空，构造名即断言
  ///   non-null），比返回的 DataFlowGraph 长寿；`handle` 为 `Option<Handle>`：
  ///   可空性由类型编码（None = cpp 默认 `nullptr` 成员），`Some` 时其目标须
  ///   比构建过程存活。
  pub unsafe fn build(
    block: *mut AstStatBlock,
    def_arena: Handle<DefArena>,
    key_arena: Handle<RefinementKeyArena>,
    handle: Option<Handle<InternalErrorReporter>>,
  ) -> DataFlowGraph {
    LUAU_TIMETRACE_SCOPE!("DataFlowGraphBuilder::build", "Typechecking");

    let mut builder = DataFlowGraphBuilder::data_flow_graph_builder_not_null_def_arena_not_null_refinement_key_arena(def_arena, key_arena);
    builder.handle = handle;

    let module_scope = builder.scopes.push(DfgScope {
      // 既有约定（review.md §2）：parent 空 = 根 scope 哨兵，是 DFG arena 的图边句柄
      // （子 scope 以 `*mut DfgScope` 回指父节点），非可改 `Option` 的可选依赖。
      parent: null_mut(),
      scope_type: ScopeType::Linear,
      bindings: Bindings::new(Symbol::default()),
      props: Props::new(DefId::NULL),
    });

    let _ps = PushScope::new(&mut builder.scope_stack, module_scope);
    let block =
      alias_opt(block).expect("block 非空为 build() 的 Safety 契约，分析器产出了空模块块");
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

impl DataFlowGraphBuilder {
  pub fn current_scope(&mut self) -> *mut DfgScope {
    LUAU_ASSERT!(!self.scope_stack.is_empty());
    *self.scope_stack.last().unwrap()
  }
}

impl DenseDefault for FunctionCapture {
  fn dense_default() -> Self {
    Self::default()
  }
}
impl DenseDefault for Symbol {
  fn dense_default() -> Self {
    Self::default()
  }
}
impl DataFlowGraphBuilder {
  pub fn data_flow_graph_builder_not_null_def_arena_not_null_refinement_key_arena(
    def_arena: Handle<DefArena>,
    key_arena: Handle<RefinementKeyArena>,
  ) -> Self {
    DataFlowGraphBuilder {
      graph: DataFlowGraphBuilder::empty(),
      // C++ `NotNull` 契约由 Handle（NonNull 编码非空）在类型层承接，
      // 原判空断言的不空前提按构造成立。
      def_arena,
      key_arena,
      // C++ 成员 `InternalErrorReporter* handle = nullptr;` 的对应默认值。
      handle: None,
      scopes: Default::default(),
      scope_stack: ScopeStack::new(),
      captures: DenseHashMap::new(Symbol::default()),
    }
  }
}

impl DataFlowGraphBuilder {
  pub fn empty() -> DataFlowGraph {
    DataFlowGraph {
      ast_defs: DenseHashMap::default(),
      local_defs: DenseHashMap::default(),
      declared_defs: DenseHashMap::default(),
      def_to_symbol: DenseHashMap::default(),
      ast_refinement_keys: DenseHashMap::default(),
    }
  }
}

impl DataFlowGraphBuilder {
  /// # Safety
  /// `p、a、b` 须指向 `DataFlowGraphBuilder` 在本次建图期间持有的存活 `DfgScope`：非空、对齐，地址
  /// 随 bump 分配稳定不移动；本函数从 a/b 读取、写入 p，调用方单线程独占，函数返回后指针仍由 builder 持有。
  /// 对应 C++ `void DataFlowGraphBuilder::join(DfgScope* p, DfgScope* a, DfgScope* b)` (`cpp/Analysis/src/DataFlowGraph.cpp:221`)。
  pub(crate) unsafe fn join(&mut self, p: *mut DfgScope, a: *mut DfgScope, b: *mut DfgScope) {
    // Safety: p/a/b 是 DFG 构建遍历传入的指向 self.scopes（PinnedStorage，仅追加、地址不
    // 移动）的存活非空 DfgScope 节点，`&*a`/`&*b` 重建的共享借用覆盖本次调用；join_bindings
    // 的 pub-unsafe 契约显式容忍 p 与 a 别名（先快照 a/b.bindings 再写 p.bindings），故调用
    // 期间不会同时持有对同一 scope bindings 的 & 与 &mut，单线程顺序构建亦无并发访问。
    unsafe { self.join_bindings(p, &*a, &*b) };
    // Safety: 与上一行同前提——a/b 指向 pinned arena 存活节点，join_props 契约同样先快照
    // 再改写，避免 p 别名 a 时形成 &/&mut 重叠，借用有效期仅覆盖该次调用。
    unsafe { self.join_props(p, &*a, &*b) };
  }
}

impl DataFlowGraphBuilder {
  /// # Safety
  /// `p` 须指向 `DataFlowGraphBuilder` 在本次建图期间持有的存活 `DfgScope`：非空、对齐，地址
  /// 随 bump 分配稳定不移动；本函数从 a/b 读取、写入 p，调用方单线程独占，函数返回后指针仍由 builder 持有。
  /// 对应 C++ `void DataFlowGraphBuilder::joinBindings(DfgScope* p, const DfgScope&, const DfgScope&)` (`cpp/Analysis/src/DataFlowGraph.cpp:227`)。
  /// `void DataFlowGraphBuilder::joinBindings(...)`.
  ///
  /// Same borrow-safety concern as [`join_props`](DataFlowGraphBuilder::join_props):
  /// the `while`-loop visitor calls `join(scope, scope, whileScope)`, so `p` aliases
  /// `a`. Iterating `a.bindings` while `get_or_insert` mutates `p.bindings` (the same
  /// map) is `&`/`&mut` aliasing UB. Snapshot `a`/`b` bindings into owned vectors first
  /// so no borrow of a scope's `bindings` is held across the mutation of `p.bindings`.
  pub(crate) unsafe fn join_bindings(&mut self, p: *mut DfgScope, a: &DfgScope, b: &DfgScope) {
    unsafe {
      let a_bindings: Vec<(Symbol, DefId)> =
        a.bindings.iter().map(|(s, d)| (s.clone(), *d)).collect();
      let b_bindings: Vec<(Symbol, DefId)> =
        b.bindings.iter().map(|(s, d)| (s.clone(), *d)).collect();
      let b_find = |sym: &Symbol| -> Option<DefId> {
        b_bindings.iter().find(|(s, _)| s == sym).map(|(_, d)| *d)
      };

      for (sym, def1) in a_bindings.iter() {
        if let Some(def2) = b_find(sym) {
          let phi = self.def_arena.get_mut().phi_def_id_def_id(*def1, def2);
          *(*p).bindings.get_or_insert(sym.clone()) = phi;
        } else if let Some(def2) = (*p).lookup_symbol(sym.clone()) {
          let phi = self.def_arena.get_mut().phi_def_id_def_id(*def1, def2);
          *(*p).bindings.get_or_insert(sym.clone()) = phi;
        }
      }

      for (sym, def1) in b_bindings.iter() {
        if let Some(def2) = (*p).lookup_symbol(sym.clone()) {
          let phi = self.def_arena.get_mut().phi_def_id_def_id(*def1, def2);
          *(*p).bindings.get_or_insert(sym.clone()) = phi;
        }
      }
    }
  }
}

impl DataFlowGraphBuilder {
  pub fn make_child_scope(&mut self, scope_type: ScopeType) -> *mut DfgScope {
    let parent_scope = self.current_scope();
    // C++ `new DfgScope{currentScope(), scopeType}` uses default member
    // inits: `bindings{Symbol{}}`, `props{nullptr}`.
    let new_scope = DfgScope {
      parent: parent_scope,
      scope_type,
      bindings: Bindings::new(Symbol::default()),
      props: Props::new(DefId::NULL),
    };
    self.scopes.push(new_scope)
  }
}

impl DataFlowGraphBuilder {
  pub fn resolve_captures(&mut self) {
    for (_symbol, capture) in self.captures.iter() {
      let mut operands: Vec<DefId> = Vec::new();
      for &v in &capture.all_versions[capture.version_offset..] {
        collect_operands_into(v, &mut operands);
      }

      for capture_def in &capture.capture_defs {
        // 独占写视图：capture 的 phi 节点由 phi_vector_def_id 刚造、operands 必空
        // （cpp 同一 LUAU_ASSERT 前提）；写点与共享迭代 `self.captures` 分属不同
        // 分配，注册表独占写契约见 `records::def_registry`。
        let phi = def_as_mut::<Phi>(*capture_def);
        LUAU_ASSERT!(phi.is_some());
        let phi = phi.expect("capture def 恒为 Phi 变体（构建期不变量）");
        LUAU_ASSERT!(phi.operands.is_empty());
        phi.operands = operands.clone();
      }
    }
  }
}

impl DataFlowGraphBuilder {
  /// cpp `visitBlockWithoutChildScope(AstStatBlock*)`：在当前作用域内逐条
  /// visit 块体语句，回吐首个非 None 控制流。
  pub fn visit_block_without_child_scope(&mut self, b: &AstStatBlock) -> ControlFlow {
    let mut first_control_flow: Option<ControlFlow> = None;

    for stat in b.body.iter_nodes() {
      let cf = self.visit_stat(stat);
      if cf != ControlFlow::None && first_control_flow.is_none() {
        first_control_flow = Some(cf);
      }
    }

    first_control_flow.unwrap_or(ControlFlow::None)
  }
}

impl DataFlowGraphBuilder {
  /// cpp `visitGenericPacks`：登记泛型型包形参的默认类型包（若有）。
  pub fn visit_generic_packs(&mut self, g: AstArray<*mut AstGenericTypePack>) {
    for &generic in g.as_slice() {
      if let Some(node) = alias_opt(generic)
        && let Some(default_value) = node_ref(node.default_value)
      {
        self.visit_type_pack(default_value);
      }
    }
  }
}

impl DataFlowGraphBuilder {
  /// cpp `visitGenerics`：登记泛型形参的默认类型（若有）。
  pub fn visit_generics(&mut self, g: AstArray<*mut AstGenericType>) {
    for &generic in g.as_slice() {
      if let Some(node) = alias_opt(generic)
        && let Some(default_value) = node_ref(node.default_value)
      {
        self.visit_type(default_value);
      }
    }
  }
}

impl DataFlowGraphBuilder {
  /// cpp `visitTypeList`：逐类型登记，尾型包非空时继续分派。
  pub fn visit_type_list(&mut self, l: AstTypeList) {
    for &t in l.types.as_slice() {
      // SAFETY: 数组元素由 parser 契约保证非空（cpp 直接解引用）。
      let t = arena_ref::<AstType>(t, "AstTypeList.types 元素");
      self.visit_type(t);
    }

    // 尾型包可空：cpp `if (l.tailType)` 同款，判空折叠进取引用。
    if let Some(tail_type) = alias_opt(l.tail_type) {
      self.visit_type_pack(tail_type);
    }
  }
}

impl DataFlowGraphBuilder {
  /// cpp `visit(AstTypePack*)` 的分派入口。
  pub fn visit_type_pack(&mut self, p: &AstTypePack) {
    match p.as_pack_ref() {
      AstTypePackRef::Explicit(e) => {
        self.visit_type_pack_explicit(e);
      }
      AstTypePackRef::Variadic(v) => {
        self.visit_type_pack_variadic(v);
      }
      AstTypePackRef::Generic(_) => {
        // ok
      }
    }
  }

  /// cpp `visit(AstTypePackExplicit*)`：展开显式类型包。
  pub fn visit_type_pack_explicit(&mut self, e: &AstTypePackExplicit) {
    self.visit_type_list(e.type_list);
  }

  /// cpp `visit(AstTypePackVariadic*)`：登记 `T...` 的元素类型。
  pub fn visit_type_pack_variadic(&mut self, v: &AstTypePackVariadic) {
    // variadic_type 槽已句柄化（parser 契约保证非空，cpp 直接解引用），get() 直取。
    self.visit_type(v.variadic_type.get());
  }
}
