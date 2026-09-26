use core::ptr::NonNull;

use ulua_ast::{records::ast_stat_block::AstStatBlock, visit::AstVisitable};
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::arc_as_mut::arc_as_mut,
  records::{constraint_generator::ConstraintGenerator, global_prepopulator::GlobalPrepopulator},
  type_aliases::scope_ptr_type::ScopePtr,
};
impl ConstraintGenerator {
  // ConstraintGenerator::prepopulateGlobalScopeForFragmentTypecheck(
  //     const ScopePtr&, const ScopePtr&, AstStatBlock*) (ConstraintGenerator.cpp:4957).
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn prepopulate_global_scope_for_fragment_typecheck(
    &mut self,
    _global_scope: &ScopePtr,
    _resume_scope: &ScopePtr,
    program: *mut AstStatBlock,
  ) {
    // Handle type function globals as well, without preparing a module scope since
    // they have a separate environment.
    // Safety: `self.type_function_runtime.as_ptr()` 对应 C++ `NotNull<TypeFunctionRuntime*>`，
    // 构造期接线、非空且比本生成器长寿，其 `root_scope: ScopePtr` 是存活
    // `Arc<Scope>` 字段；`arc_as_mut` 取 Arc 内嵌 Scope 的裸地址（非空对齐），
    // ScopePtr 由 runtime 持有、本次预填充期间存活，单线程串行独占写。
    let root_scope_raw = { arc_as_mut(&self.type_function_runtime.get_mut().root_scope) };

    let mut tfgp = GlobalPrepopulator {
      // Safety: `root_scope_raw` 即 `Arc::as_ptr` 输出，非空性由 Arc 不变量
      // 保证，new_unchecked 前提成立。
      global_scope: unsafe { NonNull::new_unchecked(root_scope_raw) },
      // Safety: `self.arena.as_ptr()` 对应 C++ `NotNull<TypeArena*>`，构造期接线、非空
      // 且比持有者长寿；类型 arena 块地址不移动。
      arena: unsafe { NonNull::new_unchecked(self.arena.as_ptr()) },
      // Safety: `self.dfg` 同为构造期接线的 NotNull 裸指针（非空、比持有者
      // 长寿），const→mut 视图仅恢复 C++ 非 const 成员函数语义，写入串行。
      dfg: unsafe { NonNull::new_unchecked(self.dfg as *mut _) },
      uninitialized_globals: DenseHashSet::default(),
    };

    unsafe {
      // Safety: `program` 由 fragment typecheck 入口传入，指向 parse 后存活
      // 至本次预填充结束的 AST arena 根块节点（C++ 非空形参）；visit 仅以
      // &mut tfgp 收集全局名，不改 AST。
      (*program).visit(&mut tfgp);
    }

    for name in tfgp.uninitialized_globals.iter() {
      self.uninitialized_globals.insert(name);
    }
  }
}
