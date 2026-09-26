//! Source: `VM/src/lvmexecute.cpp:73-77` (hand-ported)
//!
//! cpp 在 VM 关键路径用的精简版 GC 检查：不经 VM_PROTECT，直接落 savedpc
//! 并在步进后刷新 `base` 局部缓存。仅可在 `luau_execute_impl` 内展开。
//!
//! # Safety（由展开点 unsafe 上下文承担）
//! `l` 为执行中的存活 `lua_State` 且 `(*l).ci` 活动（savedpc 普通字段写入）；
//! `lua_c_step` 可能 realloc 栈，刷新 `base` 后调用方不得再使用刷新前的栈指针。

#[macro_export]
macro_rules! VM_CHECK_GC {
  ($l:expr, $pc:expr, $base:expr) => {{
    if $crate::macros::lua_c_needs_gc::luaC_needsGC!($l) {
      (*(*$l).ci).savedpc = $pc;
      $crate::functions::lua_c_step::lua_c_step($l, true);
      $base = (*$l).base;
    }
  }};
}

pub use VM_CHECK_GC;
