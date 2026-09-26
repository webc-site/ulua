//! `VM_PROTECT_PC` + 执行 `$x` + 刷新 `base` 的固定样板：可能触发 realloc栈/
//! 报错的调用点，执行前把 `pc` 落进 `ci->savedpc`，恢复路径据此续跑。
//!
//! # Safety（由展开点 unsafe 上下文承担）
//! `$l` 指向当前执行的存活 `lua_State` 且 `(*l).ci` 为活动 CallInfo（savedpc 为
//! 其普通字段写入）；`$pc` 落在当前 proto code 数组内；`$x` 内只允许再走
//! 同契约的 VM 原语。仅可在 `luau_execute_impl` 内展开（依赖局部 `base` 可写）。
#[macro_export]
macro_rules! vm_protect {
  ($l:expr, $pc:expr, $base:expr, $x:expr) => {
    (*(*$l).ci).savedpc = $pc;
    {
      $x;
    };
    $base = (*$l).base;
  };
}

pub use vm_protect;
