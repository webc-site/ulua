//! Source: `VM/src/lcorolib.cpp:14`
//!
//! `coroutine.status` — push the textual status of the thread argument. The
//! index order matches `lua_costatus`: 0 running, 1 suspended, 2 normal, 3/4
//! dead (the C++ `statnames` table repeats "dead" for COERR/COFIN).

use crate::{
  enums::lua_co_status::LuaCoStatus, functions::lua_costatus::lua_costatus,
  macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向存活 `LuaState`，索引/长度/标签等参数满足各 API 注释约定，需压栈时栈顶预留由调用方保证。
pub(crate) unsafe fn costatus(l: *mut LuaState) -> i32 {
  // SAFETY: 契约保证 `L` 存活且实参 1 为 thread 槽位，读取其 status 字段不越出协程对象界
  unsafe {
    // 非 thread 实参直接走 `type_error`（`-> !`）：let-else 让「取不到即报错」的
    // 控制流由类型承载，不再需要 unwrap 的二次论证。
    let Some(co) = (*l).to_thread(1) else {
      (*l).type_error(1, "thread")
    };

    let name = LuaCoStatus::from_c_int(lua_costatus(l, co)).map_or("dead", LuaCoStatus::as_str);
    (*l).push_str(name);
    1
  }
}

lua_lib_fn!(pub(crate) fn costatus, costatus_arm);
