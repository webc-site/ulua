//! Source: `VM/src/lvmexecute.cpp:79-91` (hand-ported)
//!
//! C++ `goto exit` becomes `return` — the macro only expands inside
//! `luau_execute_impl`, whose `exit:` label is the function end.
//!
//! # Safety（由展开点 unsafe 上下文承担）
//! `l` 为执行中的存活 `lua_State` 且 `(*l).ci` 活动、savedpc 已指向当前指令
//! （宏内对 savedpc 做 ±1 游走）；`interrupt` 回调遵守宿主注入协议（不得移动
//! ci 数组外的状态）。仅可在 `luau_execute_impl` 内展开。

#[macro_export]
macro_rules! VM_INTERRUPT {
  ($l:expr, $pc:expr, $base:expr) => {{
    let interrupt = (*(*$l).global).cb.interrupt;
    if let Some(interrupt) = interrupt {
      // the interrupt hook is called right before we advance pc
      $crate::macros::vm_protect::vm_protect!($l, $pc, $base, {
        (*(*$l).ci).savedpc = (*(*$l).ci).savedpc.add(1);
        interrupt($l, -1);
      });
      if (*$l).status != 0 {
        (*(*$l).ci).savedpc = (*(*$l).ci).savedpc.sub(1);
        return;
      }
    }
  }};
}

pub use VM_INTERRUPT;
