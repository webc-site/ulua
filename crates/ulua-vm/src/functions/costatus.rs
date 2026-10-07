//! Source: `VM/src/lcorolib.cpp:14`
//!
//! `coroutine.status` — push the textual status of the thread argument. The
//! index order matches `lua_costatus`: 0 running, 1 suspended, 2 normal, 3/4
//! dead (the C++ `statnames` table repeats "dead" for COERR/COFIN).

use crate::{
  enums::lua_co_status::LuaCoStatus, functions::lua_costatus::lua_costatus,
  macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState,
};

/// 首参 `l` 的存活/独占前提已由 `&mut` 接收者类型承载（本票收形为引用形，`@ref` 臂于 FFI 一处
/// 重建）；保留 `unsafe fn` 的真前提在协程帧指针 `co` 一侧：`co` 由 `to_thread` 自 `l` 栈槽 #1
/// 取回（调用方持有的 coroutine 线程），本函数须解引用它取 `&LuaState` 交 `lua_costatus` 读数。
/// 调用序契约：`l` 须为可抛错的受保护帧——非 thread 实参经 `type_error`（`-> !`）发散，故其后
/// `co` 必为非空存活协程；`lua_costatus` 仅只读 `l`/`co` 两侧，`push_str` 占用 `top` 之上 1 空槽。
/// cpp/VM/src/lcorolib.cpp:14 costatus。
pub(crate) unsafe fn costatus(l: &mut LuaState) -> i32 {
  // 非 thread 实参直接走 `type_error`（`-> !`）：let-else 让「取不到即报错」的控制流由类型承载。
  let Some(co) = l.to_thread(1) else {
    l.type_error(1, "thread")
  };

  // SAFETY: `co` 为 `to_thread` 自本帧栈槽 #1 取回的非空存活协程指针（非 thread 已由上方
  // type_error 发散）；此处仅取其只读引用 `&*co` 交 `lua_costatus`（`&LuaState` 形参）读数，
  // 借用窗止于本次调用；`&*l` 为接收者只读重借用，与 `co` 属不同 LuaState、无别名
  let status = unsafe { lua_costatus(&*l, &*co) };
  let name = LuaCoStatus::from_c_int(status).map_or("dead", LuaCoStatus::as_str);
  l.push_str(name);
  1
}

lua_lib_fn!(pub(crate) fn costatus @ref, costatus_arm);
