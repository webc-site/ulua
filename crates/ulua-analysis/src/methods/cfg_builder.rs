//! `cfg_builder` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::{
  string::{String, ToString},
  vec::Vec,
};

use ulua_ast::{
  records::{
    ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal, ast_stat_block::AstStatBlock,
    node_handle::OptNode,
  },
  rtti::ast_node_try_as,
};
use ulua_common::{
  fflag,
  records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet},
};

use crate::{
  enums::block_kind::BlockKind,
  methods::block::block_set_reaching_definition,
  records::{
    arena_handle::Handle, block_registry::resolve_block_mut, cfg_allocator::CfgAllocator,
    cfg_builder::CfgBuilder, control_flow_graph::ControlFlowGraph, join::Join, symbol::Symbol,
  },
  type_aliases::{block_id::BlockId, def_id_control_flow_graph::DefId, instr_id::InstrId},
};

// Source: `Analysis/include/Luau/ControlFlowGraph.h:348-353` (hand-ported)
// C++ `CFGBuilder::BlockScope::BlockScope(CFGBuilder& builder, Block* target)`.

impl CfgBuilder {
  /// RAII enter. C++ ctor saves `builder.currentBlock` and sets it to `target`:
  ///   `: builder(builder), saved(builder.currentBlock.get())
  ///    { builder.currentBlock = NotNull{target}; }`
  /// Here we apply the builder mutation (`currentBlock = target`); the matching
  /// restore (the dtor's job) is performed by the lowering scope that holds the
  /// saved block (see `lower_ast_stat_if` / `lower_ast_stat_while`).
  pub fn block_scope_cfg_builder_block(&mut self, target: BlockId) {
    self.current_block = target;
  }
}

// Source: `Analysis/src/ControlFlowGraph.cpp:127-133` (hand-ported)
// C++ `explicit CFGBuilder::CFGBuilder(NotNull<CFGAllocator> allocator)`.

impl CfgBuilder {
  pub fn new(allocator: Handle<CfgAllocator>) -> Self {
    // C++ member-init order:
    //   cfg(std::make_unique<ControlFlowGraph>(allocator))
    //   allocator(allocator)
    //   currentBlock(cfg->newBlock(BlockKind::Entry, "Entry Block"))
    let mut cfg = ControlFlowGraph::new(allocator);
    let current_block = cfg.new_block(BlockKind::Entry, "Entry Block".to_string());

    let mut builder = Self {
      cfg: Some(cfg),
      allocator,
      current_block,
      // C++ `sealedBlocks{nullptr}`
      sealed_blocks: DenseHashSet::default(),
      // C++ `incompleteJoins{nullptr}`
      incomplete_joins: DenseHashMap::default(),
      // C++ `versionCounter{Symbol{}}`
      version_counter: DenseHashMap::new(Symbol::default()),
    };

    // C++ constructor body: seal(currentBlock);
    builder.seal(current_block);
    builder
  }
}

impl CfgBuilder {
  /// `Join* CFGBuilder::emitJoin(Block* block, Symbol sym)`.
  /// Reference: `ControlFlowGraph.cpp:258-265`. `block`/返回值自 #17 续起为
  /// `BlockId`/`InstrId` u32 句柄（Join 变体在补全侧经注册表甄别）。
  pub(crate) fn emit_join(&mut self, block: BlockId, sym: Symbol) -> InstrId {
    let def = self.new_definition(sym.clone());
    let j = self.emit::<Join, _>(block, def);
    let block_ref =
      resolve_block_mut(block).expect("BlockId 为本次构建期 register_block 发放的存活句柄");
    block_set_reaching_definition(block_ref, sym, def);
    self.incomplete_joins.get_or_insert(block).insert(j);
    j
  }
}

impl CfgBuilder {
  pub fn is_sealed(&self, b: BlockId) -> bool {
    self.sealed_blocks.contains(&b)
  }
}

// Source: `Analysis/src/ControlFlowGraph.cpp:135-143` (hand-ported)
// C++ `std::unique_ptr<ControlFlowGraph> CFGBuilder::makeCFG(NotNull<CFGAllocator> allocator, AstStatBlock* block)`.

impl CfgBuilder {
  /// 对应 C++ `CFGBuilder::makeCFG(NotNull<CFGAllocator>, AstStatBlock*) ->
  /// std::unique_ptr<ControlFlowGraph>`（`cpp/Analysis/src/ControlFlowGraph.cpp:135`）。
  /// - `allocator`：arena 属主的别名句柄（`Handle` 编码恒非空，对应 cpp
  ///   `NotNull`），须在返回的 `ControlFlowGraph` 整个使用期内存活——CFG 只存
  ///   句柄、不接管 arena 所有权；
  /// - `block`：借用类型承载非空/对齐/存活（cpp 第二参 `AstStatBlock*` 的首层
  ///   解引用收口在调用方）。
  ///
  /// 所有权形态：cpp 用 `unique_ptr` 把 CFG 交给调用方，Rust 侧以按值返回
  /// （move）作显式转手动作（§2 末条）。`ControlFlowGraph` 无自定义 `Drop`，
  /// 其析构只释放自身 `Vec`/`DenseHashMap`，节点内存始终由 `CfgAllocator`
  /// arena 属主负责，转手不会误释放 arena。
  pub fn make_cfg(allocator: Handle<CfgAllocator>, block: &AstStatBlock) -> ControlFlowGraph {
    // C++:
    //   CFGBuilder builder(allocator);
    //   builder.lower(block);
    let mut builder = CfgBuilder::new(allocator);
    // `block` is `AstStatBlock*`; C++ `lower(block)` dispatches to the
    // `AstStatBlock*` overload.
    builder.lower_ast_stat_block(block);

    // auto cfg = std::move(builder.cfg);
    // 不变式：`CfgBuilder::new` 构造期即置入 cfg（对应 C++ 构造里
    // `cfg(std::make_unique<ControlFlowGraph>(allocator))`），lowering 全程只经
    // `as_mut` 借用、从不置 None，故 take() 必为 Some。
    let cfg = builder
      .cfg
      .take()
      .expect("CfgBuilder 构造期置入 cfg，lowering 不清空，take 必 Some");

    // if (FFlag::DebugLuauFreezeArena) allocator->freeze();
    if fflag::DebugLuauFreezeArena.get() {
      allocator.get_mut().freeze();
    }

    // return cfg;  (unique_ptr -> 按值 move，所有权显式交给调用方)
    cfg
  }
}

// Source: `Analysis/src/ControlFlowGraph.cpp:245-251` (hand-ported)
// C++ `Block* CFGBuilder::newBlock(BlockKind kind, std::string debugName, Block* pred)`.

impl CfgBuilder {
  /// C++ default arg `Block* pred = nullptr`；缺省在前 §2 化为
  /// `Option<BlockId>`（不再是 `null_mut()` 哨兵）。
  pub fn new_block(
    &mut self,
    kind: BlockKind,
    debug_name: String,
    pred: Option<BlockId>,
  ) -> BlockId {
    // C++:
    //   Block* b = cfg->newBlock(kind, debugName);
    //   if (pred) pred->addSuccessor(b);
    //   return b;
    // 不变式：`CfgBuilder::new` 构造期置入 cfg 且 lowering 期间从不 take/置
    // None（与 make_cfg 的 take 只在 lowering 结束后发生一次相互印证）。
    let b = self
      .cfg
      .as_mut()
      .expect("构造期置入 cfg，lowering 期间恒 Some")
      .new_block(kind, debug_name);
    if let Some(pred) = pred {
      pred.add_successor(b);
    }
    b
  }
}

impl CfgBuilder {
  pub fn new_definition(&mut self, sym: Symbol) -> DefId {
    let version = self.next_version_index(sym.clone());
    // arena 句柄经 `get_mut` 物化本次调用的独占借用（arena_handle 模块契约）。
    self.allocator.get_mut().new_definition(sym, version)
  }
}

impl CfgBuilder {
  pub fn next_version_index(&mut self, sym: Symbol) -> usize {
    if !self.version_counter.contains(&sym) {
      *self.version_counter.get_or_insert(sym) = 0;
      return 0;
    }

    // 上方 `!contains` 分支已早退，此处 sym 恒已登记，find_mut 命中 Some。
    let ref_mut = self
      .version_counter
      .find_mut(&sym)
      .expect("上方 !contains 早退蕴含 sym 已登记");
    *ref_mut += 1;
    *ref_mut
  }
}

impl CfgBuilder {
  /// `void CFGBuilder::seal(Block* b)`. Reference: `ControlFlowGraph.cpp`.
  pub(crate) fn seal(&mut self, b: BlockId) {
    // C++:
    //   auto joinsToFill = incompleteJoins.find(b);
    //   if (joinsToFill != nullptr)
    //       for (auto j : *joinsToFill) fillJoinOperands(b, j);
    //   sealedBlocks.insert(b);
    if let Some(joins) = self.incomplete_joins.find(&b) {
      let joins: Vec<_> = joins.iter().copied().collect();
      for j in joins {
        self.fill_join_operands(b, j);
      }
    }
    self.sealed_blocks.insert(b);
  }
}

impl CfgBuilder {
  pub fn trim_trivial_join(&mut self, _j: InstrId) {}
}

// Source: `Analysis/src/ControlFlowGraph.cpp:258-279` (hand-ported)
// C++ `bool CFGBuilder::tryLowerAssertion(AstExprCall* call)`.
// 把 `assert(...)` 调用下推为断言精化：逐个下推实参以登记读用，并对首个实参
// 解析出精化树后发射 Refine 指令。C++ `bool` -> Rust `bool`（true 表示已下推）。

impl CfgBuilder {
  /// C++ `bool CFGBuilder::tryLowerAssertion(AstExprCall* call)` (`cpp/Analysis/src/ControlFlowGraph.cpp:258`)。
  /// 把 `assert(...)` 调用下推为断言精化：逐个下推实参以登记读用，并对首个实参
  /// 解析出精化树后发射 Refine 指令。C++ `bool` -> Rust `bool`（true 表示已下推）。
  pub fn try_lower_assertion(&mut self, call: &AstExprCall) -> bool {
    // if (call->args.size == 0) return false;
    let args = call.args;
    if args.is_empty() {
      return false;
    }

    // auto global = call->func->as<AstExprGlobal>();
    // if (!global || global->name != "assert") return false;
    // cpp `call->func->as<AstExprGlobal>()` 判型+判空折叠为 Option 门面。
    // `func` 仍是 records 引用化波次前的裸指针字段：经句柄门面
    // `OptNode::from_ptr` 折叠可空性，下转走生命周期正确的 [`ast_node_try_as`]，
    // 借用半径由本函数局部句柄供给，不锻造假 'static。
    let func = OptNode::from_ptr(call.func);
    let Some(global) = func.get().and_then(|f| ast_node_try_as::<AstExprGlobal>(f)) else {
      return false;
    };
    if global.name != "assert" {
      return false;
    }

    // AstExpr* cond = call->args.data[0];
    let cond = args.as_slice()[0];
    for (i, &arg) in args.iter().enumerate() {
      self.lower_expr_ast_expr(arg);
      // if (i == 0) { if (auto ref = resolveCondition(cond))
      //     emitRefineInstruction(currentBlock, *ref); }
      if i == 0
        && let Some(refinement) = self.resolve_condition(cond)
      {
        let current_block = self.current_block;
        self.emit_refine_instruction(current_block, refinement);
      }
    }

    true
  }
}
