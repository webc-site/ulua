//! Source: `VM/src/lcorolib.cpp:219`
//!
//! `coroutine.close` — close a dead/suspended thread: error if it is running or
//! normal, otherwise push `true` (and reset) for a clean thread, or `false` plus
//! the error object for an errored one, then reset it.

use crate::{
  enums::{lua_co_status::LuaCoStatus, lua_status::LuaStatus},
  functions::{lua_costatus::lua_costatus, lua_resetthread::lua_resetthread, lua_xmove::lua_xmove},
  macros::{
    lua_errerrmsg::LUA_ERRERRMSG_STR, lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn,
    lua_memerrmsg::LUA_MEMERRMSG_STR,
  },
  records::lua_state::LuaState,
};

/// 首参 `l` 的存活/独占前提已由 `&mut` 接收者类型承载（本票收形为引用形，`@ref` 臂于 FFI 一处
/// 重建）；保留 `unsafe fn` 的真前提在协程帧指针 `co` 一侧：`co` 由 `to_thread` 自 `l` 栈槽 #1
/// 取回（调用方持有的 coroutine 线程），须被解引用只读 `status`、并被 `lua_resetthread`/`lua_xmove`
/// 以独占借用写改。
/// 调用序契约：`l` 须处于可抛错、可分配/GC 的受保护帧——非 thread 实参经 `type_error`（`-> !`）
/// 发散，其后 `co` 必非空存活；`running`/`normal` 态经 `luaL_error!` 抛出发散；余下分支只重排
/// `l` 栈（push_boolean/push_str）与搬移 `co`→`l` 的错误值，末置 `lua_resetthread(co)`。
/// cpp/VM/src/lcorolib.cpp:219 coclose。
pub(crate) unsafe fn coclose(l: &mut LuaState) -> i32 {
  // 非 thread 实参直接走 `type_error`（`-> !`）：let-else 让「取不到即报错」的控制流由类型承载。
  let Some(co) = l.to_thread(1) else {
    l.type_error(1, "thread")
  };

  // SAFETY: `co` 为 `to_thread` 取回的非空存活协程（非 thread 已 type_error 发散）；此处只与
  // `&*l` 同作只读引用交 `lua_costatus`，借用窗止于本次调用
  let status = unsafe { lua_costatus(&*l, &*co) };
  if status != LuaCoStatus::CoFin as i32
    && status != LuaCoStatus::CoErr as i32
    && status != LuaCoStatus::CoSus as i32
  {
    let sname = LuaCoStatus::from_c_int(status).map_or("dead", LuaCoStatus::as_str);
    // SAFETY: `l` 由接收者保证存活可抛错帧，`l.as_mut_ptr()` 借用于本次发散调用
    unsafe { luaL_error!(l.as_mut_ptr(), "cannot close {} coroutine", sname) };
  }

  // SAFETY: `co` 为存活协程，此处只读其 `status` 字段。本函数直到 `resetthread` 前不写 `co.status`，
  // 与原逐点位重复读取同值（观测等价，无副作用），故收敛为单次现读
  let co_status = unsafe { (*co).status as i32 };
  if co_status == LuaStatus::Ok as i32 || co_status == LuaStatus::Yield as i32 {
    l.push_boolean(true);
    // SAFETY: `co` 为存活协程，独占重借用交 `lua_resetthread` 复位
    unsafe { lua_resetthread(&mut *co) };
    1
  } else {
    l.push_boolean(false);

    if co_status == LuaStatus::ErrMem as i32 {
      l.push_str(LUA_MEMERRMSG_STR);
    } else if co_status == LuaStatus::ErrErr as i32 {
      l.push_str(LUA_ERRERRMSG_STR);
    } else {
      // SAFETY: `co` 存活，`get_top` 只读其栈顶槽距
      if unsafe { (*co).get_top() } != 0 {
        // SAFETY: `co` 为存活协程独占借用、`l` 为接收者独占借用，二者不同 LuaState 无别名；
        // `lua_xmove` 移 1 个错误值 co→l
        unsafe { lua_xmove(&mut *co, l, 1) }; // move error message
      }
    }

    // SAFETY: `co` 为存活协程独占借用，交 `lua_resetthread` 复位
    unsafe { lua_resetthread(&mut *co) };
    2
  }
}

lua_lib_fn!(pub(crate) fn coclose @ref, coclose_arm);
