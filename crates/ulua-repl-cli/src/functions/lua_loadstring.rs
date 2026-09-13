use alloc::string::String;
use core::{
  ffi::{c_char, c_int},
  ptr::null_mut,
  slice::from_raw_parts,
  str::from_utf8_unchecked,
};

use ulua_ast::records::parse_options::ParseOptions;
use ulua_bytecode::records::bytecode_encoder::BytecodeEncoder;
use ulua_compiler::functions::compile::compile;
use ulua_vm::{
  functions::{
    lua_insert::lua_insert, lua_l_checklstring::lua_l_checklstring,
    lua_l_optlstring::lua_l_optlstring, lua_pushnil::lua_pushnil, lua_setsafeenv::lua_setsafeenv,
    luau_load::luau_load,
  },
  macros::lua_environindex::LUA_ENVIRONINDEX,
  type_aliases::lua_state::lua_State,
};

use crate::functions::copts::copts;

pub unsafe extern "C-unwind" fn lua_loadstring(l: *mut lua_State) -> i32 {
  unsafe {
    let mut len: usize = 0;
    let s = lua_l_checklstring(l, 1, &mut len as *mut usize);
    let chunkname = lua_l_optlstring(l, 2, s, null_mut());

    lua_setsafeenv(l, LUA_ENVIRONINDEX, false as c_int);

    let source_bytes = from_raw_parts(s as *const u8, len);
    let source: String = from_utf8_unchecked(source_bytes).into();

    struct NoopEncoder;
    impl BytecodeEncoder for NoopEncoder {
      fn encode(&mut self, _data: &mut [u32]) {}
    }
    let options = copts();
    let parse_options = ParseOptions::default();
    let mut encoder = NoopEncoder;
    let bytecode = compile(
      &source,
      &options,
      &parse_options,
      &mut encoder as *mut dyn BytecodeEncoder,
    );

    if luau_load(
      l,
      chunkname,
      bytecode.as_ptr() as *const c_char,
      bytecode.len(),
      0,
    ) == 0
    {
      return 1;
    }

    lua_pushnil(l);
    lua_insert(l, -2); // put before error message
    2 // return nil plus error message
  }
}
