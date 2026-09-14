use core::{
  ffi::{c_int, c_void},
  ptr::null_mut,
};

use ulua_compiler::functions::luau_compile::luau_compile;
use ulua_vm::{
  functions::{
    lua_insert::lua_insert, lua_l_checklstring::lua_l_checklstring,
    lua_l_optlstring::lua_l_optlstring, lua_pushnil::lua_pushnil, lua_setsafeenv::lua_setsafeenv,
    luau_load::luau_load,
  },
  macros::lua_environindex::LUA_ENVIRONINDEX,
  records::lua_state::lua_State,
};

unsafe extern "C" {
  fn free(ptr: *mut c_void);
}

pub extern "C-unwind" fn lua_loadstring(l: *mut lua_State) -> c_int {
  unsafe {
    let mut len: usize = 0;
    let s = lua_l_checklstring(l, 1, &mut len);
    let chunkname = lua_l_optlstring(l, 2, s, null_mut());

    lua_setsafeenv(l, LUA_ENVIRONINDEX, 0);

    let mut bytecode_size: usize = 0;
    let bytecode = luau_compile(s, len, null_mut(), &mut bytecode_size);
    let status = luau_load(l, chunkname, bytecode, bytecode_size, 0);
    free(bytecode as *mut c_void);

    if status == 0 {
      return 1;
    }

    lua_pushnil(l);
    lua_insert(l, -2); // put before error message
    2 // return nil plus error message
  }
}
