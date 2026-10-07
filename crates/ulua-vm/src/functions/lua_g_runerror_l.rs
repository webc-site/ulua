//! Source: `VM/src/ldebug.cpp:335-347` (hand-ported)
//!
//! DELIBERATE DEVIATION: cpp `luaG_runerror(L, fmt, ...)` 以 printf 变参现场二次扫描
//! `const char* fmt`；Rust 侧变参在宏 [`lua_g_runerror!`] 处即由 `format_args!` 预格式化
//! 成 [`Arguments`]，cpp 的 fmt 串从不被读，故把该恒为 `null` 的死形参从签名删除
//! （review.md §3「消灭未用 C 形参」）。错误串写缓冲也直接交 `pusherror_bytes` 的
//! `&[u8]` 切片形，免再折回宿主 C 串裸指针（§10）。

use core::fmt::{Arguments, Result, Write, write};

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_d_throw_ldo::lua_d_throw, lua_rawcheckstack::lua_rawcheckstack, pusherror::pusherror_bytes,
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
pub(crate) unsafe fn lua_g_runerror_l(l: *mut LuaState, args: Arguments<'_>) -> ! {
  // SAFETY: 契约保证 `l` 为存活调用帧；错误对象构造与抛出经该帧完成、不返回。格式化写入
  // 落在本函数局部字节缓冲，切片窗 `&result[..len]` 即 `pos` 前的全部字节（cpp 缓冲截断
  // 语义经 BufWriter 预留 1 字节 NUL 位保持），交 `pusherror_bytes` 后按帧形拼错压栈。
  unsafe {
    let mut result = [0u8; LUA_BUFFERSIZE];
    let mut w = BufWriter {
      buf: &mut result,
      pos: 0,
    };
    let _ = write(&mut w, args);
    let len = w.pos;

    lua_rawcheckstack(&mut *l, 1);

    pusherror_bytes(&mut *l, &result[..len]);
    lua_d_throw(l, LuaStatus::ErrRun as i32);
  }
}
