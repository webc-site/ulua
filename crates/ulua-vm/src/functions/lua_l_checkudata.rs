use alloc::{ffi::CString, vec::Vec};
use core::ffi::{c_int, c_char, c_void};

use crate::{
  functions::{
    lua_getfield::lua_getfield, lua_getmetatable::lua_getmetatable,
    lua_l_typeerror_l::lua_l_typeerror_l, lua_rawequal::lua_rawequal,
    lua_touserdata::lua_touserdata,
  },
  macros::{lua_pop::lua_pop, lua_registryindex::LUA_REGISTRYINDEX},
  type_aliases::lua_state::lua_State,
};

/// 栈缓冲容量：常规元表名（"Userdata"、"buffer" 等）远小于此值，
/// 走栈缓冲零分配；仅超长名回退堆分配。
const STACK_TNAME_CAP: usize = 128;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_l_checkudata")]
pub unsafe fn lua_l_checkudata(l: *mut lua_State, ud: c_int, tname: &str) -> *mut c_void {
  unsafe {
    let p = lua_touserdata(l, ud);
    if !p.is_null() && lua_getmetatable(l, ud) != 0 {
      // tname 需以 NUL 结尾传给 lua_getfield（cpp 接收 const char*）。
      // 常规短串拷入栈缓冲零分配；仅超长串回退堆分配，
      // 内含 NUL 时按 C 语义截断于首个 NUL，避免 CString::new 的 panic 路径。
      let bytes = tname.as_bytes();
      let mut stack_buf = [0 as c_char; STACK_TNAME_CAP];
      let heap_buf: Option<Vec<c_char>>;
      let c_tname: *const c_char = if bytes.len() < STACK_TNAME_CAP {
        // 栈缓冲初始化为 0，拷贝后天然 NUL 结尾
        for (dst, &b) in stack_buf.iter_mut().zip(bytes) {
          *dst = b as c_char;
        }
        heap_buf = None;
        stack_buf.as_ptr()
      } else {
        let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
        let mut v = Vec::with_capacity(end + 1);
        v.extend(bytes[..end].iter().copied());
        v.push(0);
        heap_buf = Some(v);
        heap_buf.as_ref().unwrap().as_ptr()
      };
      lua_getfield(l, LUA_REGISTRYINDEX, c_tname);

      if lua_rawequal(l, -1, -2) != 0 {
        lua_pop(l, 2);
        return p;
      }

      lua_pop(l, 2); // remove both metatables if they didn't match
    }

    // lua_l_typeerror_l is l_noret (returns !), so this call never returns.
    lua_l_typeerror_l(l, ud, tname);
  }
}
