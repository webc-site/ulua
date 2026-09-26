use crate::{
  functions::lua_tothread::lua_tothread, macros::lua_isthread::lua_isthread,
  records::lua_state::LuaState,
};

/// cpp `getthread`（`VM/src/ldblib.cpp:11-18`）的 Rust 版。
///
/// cpp 的 `int* arg` out 参数改为元组返回：`(目标线程, 参数基准)`，
/// 基准 1 表示首参是协程（真实参数从 2 开始），0 表示直接在当前线程上取。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`。
pub(crate) unsafe fn getthread(l: *mut LuaState) -> (*mut LuaState, i32) {
  if lua_isthread!(l, 1) {
    // Safety: 契约保证 `l` 存活且实参 1 已按 thread 校验，`lua_tothread` 读该栈槽落在帧界内；
    // `lua_isthread!` 已证其为 thread 值，故必为 `Some`。
    unsafe {
      (
        lua_tothread(l, 1).expect("lua_isthread! 已证槽 1 为 thread"),
        1,
      )
    }
  } else {
    (l, 0)
  }
}
