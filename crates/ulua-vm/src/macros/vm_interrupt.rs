//! Source: `VM/src/lvmexecute.cpp:79-91` (hand-ported)
//!
//! C++ `goto exit` becomes a `return` from the enclosing dispatch function. 该
//! 函数的返回类型随派发层形态而变（解释环是 `()`，热/慢 handler 是续延
//! `Option<VmSt>`），故退出表达式由 `$exit` 形参给出：三参形写在环内，展开为
//! 四参形并传 `return`。
//!
//! # Safety（由展开点 unsafe 上下文承担）
//! `l` 为执行中的存活 `lua_State` 且 `(*l).ci` 活动、savedpc 已指向当前指令
//! （宏内对 savedpc 做 ±1 游走）；`interrupt` 回调遵守宿主注入协议（不得移动
//! ci 数组外的状态）。仅可在派发层（`tier_cold` 与其 handler）内展开。

#[macro_export]
macro_rules! VM_INTERRUPT {
  ($l:expr, $pc:expr, $base:expr) => {
    $crate::macros::vm_interrupt::VM_INTERRUPT!($l, $pc, $base, return)
  };
  ($l:expr, $pc:expr, $base:expr, $exit:expr) => {{
    let interrupt = (*(*$l).global).cb.interrupt;
    if let Some(interrupt) = interrupt {
      // the interrupt hook is called right before we advance pc
      $crate::macros::vm_protect::vm_protect!($l, $pc, $base, {
        (*(*$l).ci).savedpc = (*(*$l).ci).savedpc.add(1);
        interrupt($l, -1);
      });
      if (*$l).status != 0 {
        (*(*$l).ci).savedpc = (*(*$l).ci).savedpc.sub(1);
        $exit;
      }
    }
  }};
}

pub use VM_INTERRUPT;
