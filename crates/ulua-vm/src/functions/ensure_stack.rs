//! cpp `ensure_stack_impl` / `ensure_stack`（`VM/src/lapi.cpp:57-66`）。

use core::ffi::c_int;

use crate::{
  functions::{
    lua_checkstack::lua_checkstack, lua_error::lua_error, lua_o_pushfstring::lua_o_pushfstring,
  },
  type_aliases::lua_state::lua_State,
};

/// 栈槽不足时先扩容；扩不出来就在 `error_l`（xmove 场景下是 `from`）上抛
/// "stack overflow"。缺了这一步，`api_incr_top`/结果槽写入会直接越过
/// `ci->top` 写到栈数组之外。
///
/// `size <= 0` 恒不触发（invariant `top <= ci->top`），这里显式短路，以免对
/// `top` 做负向偏移。比较也走偏移量而非指针相加，指针相加本身就要求结果
/// 仍落在同一分配内。
///
/// # Safety
/// `l`/`error_l` 必须是存活且已初始化的 `lua_State`，`error_l` 与 `l` 同属
/// 一个 `global_State`。
pub unsafe fn ensure_stack_impl(l: *mut lua_State, error_l: *mut lua_State, size: c_int) {
  unsafe {
    if size > 0
      // cpp: `L->top + size > L->ci->top`
      && size as isize > (*(*l).ci).top.offset_from((*l).top)
      && lua_checkstack(l, size) == 0
    {
      lua_o_pushfstring(error_l, c"stack overflow".as_ptr(), format_args!(""));
      lua_error(error_l);
    }
  }
}

/// cpp `ensure_stack(L, size)`，即在当前线程上报错的 `ensure_stack_impl`。
///
/// # Safety
/// 同 [`ensure_stack_impl`]。
pub unsafe fn ensure_stack(l: *mut lua_State, size: c_int) {
  unsafe { ensure_stack_impl(l, l, size) };
}
