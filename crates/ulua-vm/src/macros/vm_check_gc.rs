//! Source: `VM/src/lvmexecute.cpp:73-77` (hand-ported)
//!
//! cpp 在 VM 关键路径用的精简版 GC 检查：不经 VM_PROTECT，直接落 savedpc
//! 并在步进后刷新 `base` 局部缓存。仅可在 `luau_execute_impl` 内展开。

#[macro_export]
macro_rules! VM_CHECK_GC {
  ($l:expr, $pc:expr, $base:expr) => {{
    if $crate::macros::lua_c_needs_gc::luaC_needsGC!($l) {
      (*(*$l).ci).savedpc = $pc;
      $crate::functions::lua_c_step::luaC_step($l, true);
      $base = (*$l).base;
    }
  }};
}

pub use VM_CHECK_GC;
