//! cpp `createCliRequireContext`（`CLI/src/Repl.cpp`）。

use core::{
  ffi::c_void,
  mem::size_of,
  ptr::{drop_in_place, write},
};

use ulua_vm::{
  functions::{
    lua_insert::lua_insert, lua_newuserdatadtor::lua_newuserdatadtor, lua_settable::lua_settable,
  },
  macros::{
    lua_l_error::luaL_error, lua_pushlightuserdata::lua_pushlightuserdata,
    lua_registryindex::LUA_REGISTRYINDEX,
  },
  records::lua_state::LuaState,
};

use crate::{
  functions::{
    copts::copts, counters_active::counters_active, counters_track::counters_track,
    coverage_active::coverage_active, coverage_track::coverage_track,
    repl_main::repl_codegen_enabled,
  },
  records::repl_requirer::ReplRequirer,
};

/// 传给 `lua_newuserdatadtor` 的析构器，对应 cpp 的
/// `[](void* ptr) { static_cast<ReplRequirer*>(ptr)->~ReplRequirer(); }`。
unsafe extern "C-unwind" fn repl_requirer_dtor(ptr: *mut c_void) {
  // Safety: ptr 是 lua_newuserdatadtor 在析构回调中交还的 userdata 内存基址：create_cli_require_context 已用同一 size_of::<ReplRequirer>() 窗口 placement-write 构造过对象，GC 终结器在该块释放前恰好一次运行 ⇒ 指向存活的 ReplRequirer 且无其它持有者，drop_in_place 合法。
  unsafe {
    drop_in_place(ptr as *mut ReplRequirer);
  }
}

/// 在 `l` 上分配并构造 `ReplRequirer`，返回其指针作为 require 上下文。
///
/// # Safety
///
/// `l` 必须是已初始化的有效 `LuaState`；返回的上下文由该状态的 userdata
/// 生命周期持有。
pub unsafe fn create_cli_require_context(l: *mut LuaState) -> *mut c_void {
  // Safety: l 由 setup_state 的调用契约保证为有效状态；lua_newuserdatadtor 返回
  // 判空（失败即 luaL_error 发散），失败不继续。
  let ctx = unsafe { lua_newuserdatadtor(l, size_of::<ReplRequirer>(), Some(repl_requirer_dtor)) };

  if ctx.is_null() {
    // Safety: l 存活；luaL_error 仅格式化抛出并发散，不触碰 ctx。
    unsafe { luaL_error!(l, "unable to allocate ReplRequirer") }
  }

  // placement 构造：对应 cpp `new (ctx) ReplRequirer{ copts, coverageActive,
  // []{ return codegen; }, coverageTrack, countersActive, countersTrack }`，
  // 六个探针均为 Rust 内部调用的普通函数指针，无需适配层（构造参数为安全代码）。
  let requirer = ReplRequirer::new(
    copts,
    coverage_active,
    repl_codegen_enabled,
    coverage_track,
    counters_active,
    counters_track,
  );
  // Safety: 构造写入的目标块正是上一行分配的 size_of::<ReplRequirer>() 空间、
  // 未初始化且独占（判空分支已排除 null），对齐由 lua_newuserdatadtor 保证。
  unsafe { write(ctx as *mut ReplRequirer, requirer) };

  // 以内存地址为键存入 registry，使 ReplRequirer 与该 LuaState 同生命周期。
  // Safety: l 存活；ctx 为上方刚分配的非空 userdata 基址，lightuserdata+settable
  // 仅登记地址值，push/insert/settable 栈操作配平。
  unsafe {
    lua_pushlightuserdata(l, ctx);
    lua_insert(l, -2);
    lua_settable(l, LUA_REGISTRYINDEX);
  }

  ctx
}
