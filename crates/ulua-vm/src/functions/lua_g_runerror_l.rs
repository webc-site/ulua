//! Node: `cxx:Function:Luau.VM:VM/src/ldebug.cpp:335:luaG_runerrorL`
//! Source: `VM/src/ldebug.cpp:335-347` (hand-ported; C varargs follow the
//! project convention of `core::fmt::Arguments` with the C fmt string unused)

use core::{
  ffi::c_char,
  fmt::{Arguments, Result, Write, write},
};

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_d_throw_ldo::lua_d_throw, lua_rawcheckstack::lua_rawcheckstack, pusherror::pusherror,
  },
  records::lua_l_strbuf::LUA_BUFFERSIZE,
  type_aliases::lua_state::lua_State,
};

struct BufWriter<'a> {
  buf: &'a mut [u8],
  pos: usize,
}

impl Write for BufWriter<'_> {
  fn write_str(&mut self, s: &str) -> Result {
    let avail = self.buf.len().saturating_sub(self.pos + 1); // keep room for NUL
    let n = s.len().min(avail);
    self.buf[self.pos..self.pos + n].copy_from_slice(&s.as_bytes()[..n]);
    self.pos += n;
    Ok(())
  }
}

pub(crate) unsafe fn lua_g_runerror_l(
  l: *mut lua_State,
  _fmt: *const c_char,
  args: Arguments<'_>,
) -> ! {
  unsafe {
    let mut result = [0u8; LUA_BUFFERSIZE];
    let mut w = BufWriter {
      buf: &mut result,
      pos: 0,
    };
    let _ = write(&mut w, args);
    let len = w.pos;
    result[len] = 0;

    lua_rawcheckstack(l, 1);

    pusherror(l, result.as_ptr() as *const c_char);
    lua_d_throw(l, LuaStatus::ErrRun as i32);
  }
}
