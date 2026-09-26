use crate::{
  functions::{
    c_slice, c_slice_mut, ensure_stack::ensure_stack_impl, lapi_barrier::lua_c_threadbarrier_lapi,
  },
  macros::{api_check::api_check, api_checknelems::api_checknelems, setobj_2_s::setobj_2_s},
  records::{lua_state::LuaState, lua_t_value::TValue},
};

/// # Safety
/// `from`、`to` 须为同 `global_State` 下两棵存活 LuaState（`api_check` 断言 `(*from).global == (*to).global`）；
/// `n >= 0` 且 `from` 栈顶有 `n` 个可消费项（`api_checknelems`）；`ensure_stack_impl(to, from, n)` 保证 `to` 栈
/// 顶之后容 `n` 新槽（失败在 `from` 抛错）。`(*from).top - n..top` 与 `(*to).top..top + n` 互不重叠（异线程），
/// `setobj_2_s` 逐项搬移并同步 `top`。cpp/VM/src/lapi.cpp:203 lua_xmove。
pub unsafe fn lua_xmove(from: *mut LuaState, to: *mut LuaState, n: i32) {
  unsafe {
    api_check!(from, n >= 0);

    if from == to {
      return;
    }

    api_checknelems!(from, n);
    api_check!(from, (*from).global == (*to).global);

    lua_c_threadbarrier_lapi(to);

    // cpp `ensure_stack_impl(to, from, n)`：目标帧不够时扩容，失败在 from 上抛错
    ensure_stack_impl(to, from, n);

    let ttop = (*to).top;
    let ftop = (*from).top.offset(-(n as isize));

    // Safety:from 栈自 ftop 起 n 个值有效；to 栈保证容纳 n 个新值（checkstack 已过）。
    for (dst, src) in c_slice_mut(ttop, n as usize)
      .iter_mut()
      .zip(c_slice(ftop, n as usize))
    {
      setobj_2_s!(to, dst as *mut TValue, src as *const TValue as *mut TValue);
    }

    (*from).top = ftop;
    (*to).top = ttop.offset(n as isize);
  }
}
