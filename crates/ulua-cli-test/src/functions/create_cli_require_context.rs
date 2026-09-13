use core::{
  ffi::{c_int, c_void},
  mem::size_of,
  ptr::{drop_in_place, null, null_mut, write},
};

use ulua_compiler::records::compile_options::CompileOptions;
use ulua_vm::{
  functions::{
    lua_insert::lua_insert, lua_newuserdatadtor::lua_newuserdatadtor,
    lua_pushlightuserdatatagged::lua_pushlightuserdatatagged, lua_settable::lua_settable,
  },
  macros::{lua_l_error::luaL_error, lua_registryindex::LUA_REGISTRYINDEX},
  records::lua_state::lua_State,
};

use crate::records::repl_requirer::ReplRequirer;

unsafe extern "C-unwind" fn replrequirer_dtor(ptr: *mut c_void) {
  unsafe {
    drop_in_place(ptr as *mut ReplRequirer);
  }
}

unsafe extern "C-unwind" fn copts_shim() -> *mut CompileOptions {
  null_mut()
}

unsafe extern "C-unwind" fn ret_false() -> bool {
  false
}

unsafe extern "C-unwind" fn noop_coverage(_l: *mut lua_State, _funcindex: c_int) {}

/// Creates a CLI require context for the given Lua state.
///
/// # Safety
///
/// `l` must be a valid pointer to an initialized `lua_State`.
pub unsafe fn create_cli_require_context(l: *mut lua_State) -> *mut c_void {
  unsafe {
    let ctx = lua_newuserdatadtor(l, size_of::<ReplRequirer>(), Some(replrequirer_dtor));

    if ctx.is_null() {
      luaL_error!(l, "unable to allocate ReplRequirer");
    }

    write(
      ctx as *mut ReplRequirer,
      ReplRequirer::repl_requirer_repl_requirer(
        copts_shim,
        ret_false,
        ret_false,
        noop_coverage,
        ret_false,
        noop_coverage,
        null(),
      ),
    );

    // Store ReplRequirer in the registry to keep it alive for the lifetime of
    // this lua_State. Memory address is used as a key to avoid collisions.
    lua_pushlightuserdatatagged(l, ctx, 0);
    lua_insert(l, -2);
    lua_settable(l, LUA_REGISTRYINDEX);

    ctx
  }
}
