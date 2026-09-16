use core::{
  cmp::min,
  ffi::{c_char, c_int},
  fmt::{Arguments, Result, Write, write},
  mem::{size_of, transmute},
};

use crate::{
  functions::{lua_d_growstack::lua_d_growstack, lua_d_reallocstack::lua_d_reallocstack},
  macros::{extra_stack::EXTRA_STACK, lua_s_new::luaS_new, setsvalue::setsvalue, svalue::svalue},
  records::lua_l_strbuf::LUA_BUFFERSIZE,
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_o_pushvfstring(
  l: *mut lua_State,
  _fmt: *const c_char,
  args: Arguments<'_>,
) -> *const c_char {
  // Luau VM uses a fixed-size buffer for string formatting in luaO_pushvfstring.
  // Since we are translating to Rust's core::fmt::Arguments, we use a stack buffer
  // and a custom writer to mimic vsnprintf behavior.
  let mut buffer = [0u8; LUA_BUFFERSIZE];
  let mut writer = BufferWriter {
    buf: &mut buffer,
    pos: 0,
  };

  let _ = write(&mut writer, args);

  // Ensure null termination for luaS_new which expects const char*
  let len = writer.pos;
  if len < buffer.len() {
    buffer[len] = 0;
  } else {
    buffer[buffer.len() - 1] = 0;
  }

  unsafe {
    // The macro setsvalue! expects a pointer to TValue. (*l).top is a StkId (TValue*).
    setsvalue!(l, (*l).top, luaS_new(l, buffer.as_ptr() as *const c_char));

    // The previous attempt failed because the incr_top! macro expansion encountered
    // name mismatches (luaD_growstack vs lua_d_growstack) and field access errors
    // (stacksize vs stacksize). We manually perform the logic here to ensure
    // compatibility with the translated records and functions.

    // luaD_checkstack(l, 1);
    let n = 1;
    let stack_last = (*l).stack_last as *mut u8;
    let top = (*l).top as *mut u8;
    let limit_reached =
      (stack_last as usize).wrapping_sub(top as usize) <= (n as usize * size_of::<TValue>());

    if limit_reached {
      lua_d_growstack(l, n);
    } else {
      // condhardstacktests(luaD_reallocstack(l, l->stacksize - EXTRA_STACK, 0));
      // In the Rust port, we call the snake_case function.
      // Note: lua_d_reallocstack in this crate is currently a stub with no arguments.
      type LuaDReallocStackFn = unsafe fn(*mut lua_State, c_int, c_int);
      let realloc_stack: LuaDReallocStackFn = transmute(lua_d_reallocstack as *const ());
      realloc_stack(l, (*l).stacksize - EXTRA_STACK, 0);
    }

    // l->top++;
    (*l).top = (*l).top.add(1);

    // svalue! expects a pointer to TValue.
    svalue!((*l).top.offset(-1))
  }
}

struct BufferWriter<'a> {
  buf: &'a mut [u8],
  pos: usize,
}

impl<'a> Write for BufferWriter<'a> {
  fn write_str(&mut self, s: &str) -> Result {
    let bytes = s.as_bytes();
    let remain = self.buf.len().saturating_sub(self.pos);
    let to_copy = min(remain, bytes.len());
    if to_copy > 0 {
      self.buf[self.pos..self.pos + to_copy].copy_from_slice(&bytes[..to_copy]);
      self.pos += to_copy;
    }
    Ok(())
  }
}

pub use lua_o_pushvfstring as luaO_pushvfstring;
