use core::{
  ffi::{c_char, c_int},
  ptr::null_mut,
};

use crate::{
  enums::lua_status::LuaStatus, functions::lua_tolstring::lua_tolstring,
  records::lua_state::lua_State,
};

#[repr(C)]
#[derive(Debug)]
pub struct lua_exception {
  pub(crate) l: *mut lua_State,
  pub(crate) status: c_int,
}

impl lua_exception {
  pub fn new(l: *mut lua_State, status: c_int) -> Self {
    Self { l, status }
  }

  pub fn what(&self) -> *const c_char {
    // LUA_ERRRUN 的错误对象在栈顶
    if self.status == LuaStatus::ErrRun as c_int {
      unsafe {
        let val = lua_tolstring(self.l, -1, null_mut());
        if !val.is_null() {
          return val;
        }
      }
    }

    // 状态到 what 消息的映射（与 cpp 消息文本一致）
    match self.status {
      s if s == LuaStatus::ErrRun as c_int => c"lua_exception: runtime error".as_ptr(),
      s if s == LuaStatus::ErrSyntax as c_int => c"lua_exception: syntax error".as_ptr(),
      s if s == LuaStatus::ErrMem as c_int => {
        c"lua_exception: memory allocation error: block too big".as_ptr()
      }
      s if s == LuaStatus::ErrErr as c_int => c"lua_exception: error in error handling".as_ptr(),
      _ => c"lua_exception: unexpected exception status".as_ptr(),
    }
  }

  pub fn get_status(&self) -> c_int {
    self.status
  }

  pub fn get_thread(&self) -> *const lua_State {
    self.l as *const lua_State
  }
}

// The exception unwinds within one thread (C++ throw/catch semantics);
// panic_any requires Send, which the raw lua_State pointer doesn't derive.
unsafe impl Send for lua_exception {}
