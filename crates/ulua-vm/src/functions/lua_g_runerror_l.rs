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
  records::{lua_l_strbuf::LUA_BUFFERSIZE, lua_state::LuaState},
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

/// # Safety
/// `l` 必须指向存活 `LuaState`，操作数 `*const TValue`/`StkId` 指针可读/可写且对齐，TM 调用协议（res 槽、栈余量）满足。
pub(crate) unsafe fn lua_g_runerror_l(
  l: *mut LuaState,
  _fmt: *const c_char,
  args: Arguments<'_>,
) -> ! {
  // Safety: 契约保证 `l` 为存活调用帧且 fmt 与可变参按转换符严格匹配（错配即 UB），错误对象构造与抛出经该帧完成、不返回
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
