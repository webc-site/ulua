use core::{
  ffi::{c_int, c_void},
  mem::size_of,
  ptr::{drop_in_place, write},
};

use ulua_compiler::records::compile_options::CompileOptions;
use ulua_vm::{
  functions::{
    lua_insert::lua_insert, lua_newuserdatadtor::lua_newuserdatadtor, lua_settable::lua_settable,
  },
  macros::{
    LUA_PUSHLIGHTUSERDATA::LUA_PUSHLIGHTUSERDATA, lua_l_error::luaL_error,
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
  methods::repl_requirer_repl_requirer::repl_requirer_repl_requirer,
  records::repl_requirer::ReplRequirer,
};

// Destructor passed to lua_newuserdatadtor, mirroring the C++ lambda that runs
// `static_cast<ReplRequirer*>(ptr)->~ReplRequirer()`.
unsafe extern "C-unwind" fn repl_requirer_dtor(ptr: *mut c_void) {
  unsafe {
    drop_in_place(ptr as *mut ReplRequirer);
  }
}

// `coverage_active` adapter: ReplRequirer expects an `extern "C-unwind" fn() -> bool`.
unsafe extern "C-unwind" fn coverage_active_cb() -> bool {
  coverage_active()
}

// `codegen_enable`: the C++ lambda `[]() { return codegen; }`.
unsafe extern "C-unwind" fn codegen_enabled_cb() -> bool {
  repl_codegen_enabled()
}

// `counters_active` adapter.
unsafe extern "C-unwind" fn counters_active_cb() -> bool {
  counters_active()
}

// `coverage_track` adapter: ReplRequirer expects `extern "C-unwind" fn(*mut c_void, c_int)`.
unsafe extern "C-unwind" fn coverage_track_cb(l: *mut c_void, funcindex: c_int) {
  coverage_track(l as *mut lua_State, funcindex);
}

// `counters_track` adapter.
unsafe extern "C-unwind" fn counters_track_cb(l: *mut c_void, funcindex: c_int) {
  counters_track(l as *mut lua_State, funcindex);
}

pub unsafe fn create_cli_require_context(l: *mut lua_State) -> *mut c_void {
  unsafe {
    let ctx = lua_newuserdatadtor(l, size_of::<ReplRequirer>(), Some(repl_requirer_dtor));

    if ctx.is_null() {
      luaL_error!(l, "unable to allocate ReplRequirer");
    }

    // placement-construct the ReplRequirer into the userdata Buffer
    write(
      ctx as *mut ReplRequirer,
      repl_requirer_repl_requirer(
        Some(copts as fn() -> CompileOptions),
        Some(coverage_active_cb),
        Some(codegen_enabled_cb),
        Some(coverage_track_cb),
        Some(counters_active_cb),
        Some(counters_track_cb),
      ),
    );

    // Store ReplRequirer in the registry to keep it alive for the lifetime of
    // this lua_State. Memory address is used as a key to avoid collisions.
    LUA_PUSHLIGHTUSERDATA(l as *mut c_void, ctx);
    lua_insert(l, -2);
    lua_settable(l, LUA_REGISTRYINDEX);

    ctx
  }
}
