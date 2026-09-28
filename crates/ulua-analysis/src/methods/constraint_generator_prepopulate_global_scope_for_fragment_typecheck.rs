use core::ptr::{NonNull, from_ref};

use ulua_ast::{records::ast_stat_block::AstStatBlock, visit::AstVisitable};
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  records::{
    arena_handle::alias, constraint_generator::ConstraintGenerator,
    global_prepopulator::GlobalPrepopulator,
  },
  type_aliases::scope_ptr_type::ScopePtr,
};
impl ConstraintGenerator {
  /// 两处同构的「type function 环境」预填充段（cpp `prepopulateGlobalScope`
  /// 尾部与 `prepopulateGlobalScopeForFragmentTypecheck` 主体同体）：以
  /// `type_function_runtime.root_scope` 为全局作用域对 program 跑
  /// [`GlobalPrepopulator`]，收集未初始化全局名。
  ///
  /// 形参链已引用化（原 `# Safety` 契约由签名承担）：`program` 为分析会话
  /// arena 内存活的 `AstStatBlock` 共享借用，visit 期间无人并发改写该 AST；
  /// `GlobalPrepopulator` 的 NonNull 三字段由存活句柄经 `NonNull::from` 构造
  /// （记录布局不改），遍历所需的 `&mut AstStatBlock` 由 alias 门面即时物化。
  pub(super) fn prepopulate_type_function_globals(&mut self, program: &AstStatBlock) {
    // Handle type function globals as well, without preparing a module scope since
    // they have a separate environment.
    // `self.type_function_runtime` 对应 C++ `NotNull<TypeFunctionRuntime*>`，
    // 构造期接线、非空且比本生成器长寿，其 `root_scope: ScopePtr` 是存活
    // `Arc<Scope>` 字段；ScopePtr 由 runtime 持有、本次预填充期间存活。
    let root_scope_ref = self.type_function_runtime.get_mut().root_scope.clone();

    let mut tfgp = GlobalPrepopulator {
      global_scope: NonNull::from(&*root_scope_ref),
      // `self.arena` 对应 C++ `NotNull<TypeArena*>`，构造期接线、非空
      // 且比持有者长寿；类型 arena 块地址不移动。
      arena: NonNull::from(self.arena.get_mut()),
      // `self.dfg` 同为构造期接线的 NonNull 只读句柄（非空、比持有者长寿）。
      dfg: NonNull::from(self.dfg_ref()),
      uninitialized_globals: DenseHashSet::default(),
    };

    // visit 遍历 API 要求 `&mut AstStatBlock`（预填充仅以 &mut tfgp 收集
    // 全局名，不改 AST）；alias 门面从本函数参数引用即时物化、即用即弃。
    alias(from_ref(program).cast_mut()).visit(&mut tfgp);

    for name in tfgp.uninitialized_globals.iter() {
      self.uninitialized_globals.insert(name);
    }
  }

  // ConstraintGenerator::prepopulateGlobalScopeForFragmentTypecheck(
  //     const ScopePtr&, const ScopePtr&, AstStatBlock*) (ConstraintGenerator.cpp:4957).
  /// 本入口由 fragment typecheck 驱动，program 为 parse 后存活的 AST 根块
  /// 共享借用（调用方自记录字段经 alias_ref 收口交入）。
  pub fn prepopulate_global_scope_for_fragment_typecheck(
    &mut self,
    _global_scope: &ScopePtr,
    _resume_scope: &ScopePtr,
    program: &AstStatBlock,
  ) {
    self.prepopulate_type_function_globals(program)
  }
}
