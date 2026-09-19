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
  type_aliases::lua_state::lua_State,
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
  unsafe {
    drop_in_place(ptr as *mut ReplRequirer);
  }
}

/// 在 `l` 上分配并构造 `ReplRequirer`，返回其指针作为 require 上下文。
///
/// # Safety
///
/// `l` 必须是已初始化的有效 `lua_State`；返回的上下文由该状态的 userdata
/// 生命周期持有。
pub unsafe fn create_cli_require_context(l: *mut lua_State) -> *mut c_void {
  unsafe {
    let ctx = lua_newuserdatadtor(l, size_of::<ReplRequirer>(), Some(repl_requirer_dtor));

    if ctx.is_null() {
      luaL_error!(l, "unable to allocate ReplRequirer");
    }

    // placement 构造：对应 cpp `new (ctx) ReplRequirer{ copts, coverageActive,
    // []{ return codegen; }, coverageTrack, countersActive, countersTrack }`，
    // 六个探针均为 Rust 内部调用的普通函数指针，无需适配层。
    write(
      ctx as *mut ReplRequirer,
      ReplRequirer::new(
        copts,
        coverage_active,
        repl_codegen_enabled,
        coverage_track,
        counters_active,
        counters_track,
      ),
    );

    // 以内存地址为键存入 registry，使 ReplRequirer 与该 lua_State 同生命周期。
    lua_pushlightuserdata(l, ctx);
    lua_insert(l, -2);
    lua_settable(l, LUA_REGISTRYINDEX);

    ctx
  }
}
