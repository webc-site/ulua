//! Source: `VM/src/lapi.cpp:1477`
//!
//! `lua_newbuffer` — allocate a managed buffer object of `sz` bytes, push it on
//! the stack, and return a pointer to its data. Runs a GC step and the thread
//! write-barrier first, exactly like the C++ public API.

use core::ffi::c_void;

use crate::{
  functions::{
    ensure_stack::ensure_stack, lapi_barrier::lua_c_threadbarrier_lapi,
    lua_b_newbuffer::lua_b_newbuffer,
  },
  macros::{api_incr_top::api_incr_top, lua_c_check_gc::lua_c_check_gc, setbufvalue::setbufvalue},
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向本次 buffer 库调用的存活 `LuaState`，索引/长度实参按约定可读，栈顶有压入结果的余量。
///
/// r12 T9 消费面实测：返回裸指针零终端消费——vm 内 `buffer_create.rs:16`、
/// `buffer_fromstring.rs:27` 与 ulua-rt `buffer.rs` 转跳帧均以语句形调用弃返回值
/// （契约即压栈副作用，读回一律走 `lua_tobuffer*`）；ulua-capi 无引用；仅
/// ulua-conformance 门面 `safe_api.rs::newbuffer` 以 `-> *mut c_void` 尾表达式转发
/// 该指针（其终端调用点同样弃值）。门面文件在本票范围外不可动，故本期保留返回
/// 指针形；后续登记：门面收窄为单元形后，本函数返回值改 `()`。
pub unsafe fn lua_newbuffer(l: *mut LuaState, sz: usize) -> *mut c_void {
  // SAFETY: 契约保证 `l` 存活且 size 为合法缓冲长度，新建 buffer userdata 按 size 分配并置 len 字段自洽
  unsafe {
    lua_c_check_gc!(l);
    lua_c_threadbarrier_lapi(l);
    ensure_stack(l, 1);
    let b = lua_b_newbuffer(l, sz);
    setbufvalue!(l, (*l).top, b);
    api_incr_top!(l);
    (*b).data.as_mut_ptr() as *mut c_void
  }
}
