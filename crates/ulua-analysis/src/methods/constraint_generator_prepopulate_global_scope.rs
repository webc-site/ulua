use core::ptr::NonNull;

use ulua_ast::{records::ast_stat_block::AstStatBlock, visit::AstVisitable};
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::arc_as_mut::arc_as_mut,
  records::{constraint_generator::ConstraintGenerator, global_prepopulator::GlobalPrepopulator},
  type_aliases::scope_ptr_type::ScopePtr,
};
impl ConstraintGenerator {
  /// # Safety
  /// 直译 cpp `ConstraintGenerator::prepopulateGlobalScope(globalScope, program)`：
  /// - `global_scope` 必须是存活 `Arc<Scope>`（其 `arc_as_mut` 派生句柄充当 NotNull 全局作用域）；
  /// - `program` 必须非空并指向分析会话 arena 内存活的 `AstStatBlock`，且 `visit` 期间无人
  ///   并发改写该 AST（本 pass 由驱动者独占 arena）；
  /// - `self.arena.as_ptr()` / `self.dfg` / `self.type_function_runtime.as_ptr()` 须为构造期注入的非空句柄，
  ///   且 `type_function_runtime.root_scope` 已在本 pass 之前登记为存活 `Arc<Scope>`。
  pub unsafe fn prepopulate_global_scope(
    &mut self,
    global_scope: &ScopePtr,
    program: *mut AstStatBlock,
  ) {
    let global_scope_raw = arc_as_mut(global_scope);
    let mut gp = GlobalPrepopulator {
      // Safety: `global_scope_raw` 由存活 `Arc<Scope>` 经 `arc_as_mut` 派生，Arc 指针
      // 恒非空，new_unchecked 的 NotNull 前提成立，且该 Arc 在本作用域内存活。
      global_scope: unsafe { NonNull::new_unchecked(global_scope_raw) },
      // Safety: `self.arena.as_ptr()` 为构造期注入、与会话同寿的非空 TypeArena 句柄。
      arena: unsafe { NonNull::new_unchecked(self.arena.as_ptr()) },
      // Safety: `self.dfg` 为构造期注入的非空 DataFlowGraph 句柄；*const→*mut 仅做
      // 地址透传以满足 NonNull 字段签名，prepopulator visit 期只读该 dfg。
      dfg: unsafe { NonNull::new_unchecked(self.dfg as *mut _) },
      uninitialized_globals: DenseHashSet::default(),
    };

    if let Some(module) = &self.module {
      (self.prepare_module_scope)(&module.name, global_scope);
    }

    // Safety: `program` 按函数级契约非空且指向 arena 内存活 AstStatBlock，本 pass
    // 独占该 AST；`&mut gp` 为局部唯一借用，NonNull 三字段均由上面的存活句柄构成。
    unsafe {
      (*program).visit(&mut gp);
    }

    for name in gp.uninitialized_globals.iter() {
      self.uninitialized_globals.insert(name);
    }

    // Safety: `self.type_function_runtime.as_ptr()` 为构造期注入的非空存活句柄，解引用只读其
    // `root_scope`（已登记的存活 `Arc<Scope>`），arc_as_mut 派生的句柄存活至本语句结束。
    let root_scope_raw = { arc_as_mut(&self.type_function_runtime.get_mut().root_scope) };

    let mut tfgp = GlobalPrepopulator {
      // Safety: `root_scope_raw` 是上面自存活 `Arc<Scope>` 派生的句柄，Arc 指针恒非空。
      global_scope: unsafe { NonNull::new_unchecked(root_scope_raw) },
      // Safety: 同 gp.arena——构造期注入的非空 TypeArena 句柄。
      arena: unsafe { NonNull::new_unchecked(self.arena.as_ptr()) },
      // Safety: 同 gp.dfg——构造期非空句柄的 *const→*mut 地址透传，visit 期只读。
      dfg: unsafe { NonNull::new_unchecked(self.dfg as *mut _) },
      uninitialized_globals: DenseHashSet::default(),
    };

    // Safety: 同首次 visit——`program` 存活非空、arena 独占，`&mut tfgp` 为局部唯一借用。
    unsafe {
      (*program).visit(&mut tfgp);
    }

    for name in tfgp.uninitialized_globals.iter() {
      self.uninitialized_globals.insert(name);
    }
  }
}
