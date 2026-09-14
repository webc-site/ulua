use core::ffi::c_int;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::type_aliases::proto::Proto;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaG_getline")]
pub unsafe fn lua_g_getline(p: *mut Proto, pc: c_int) -> c_int {
  unsafe {
    LUAU_ASSERT!(pc >= 0 && pc < (*p).sizecode);

    if (*p).lineinfo.is_null() {
      return 0;
    }

    let abs_index = (pc >> (*p).linegaplog2) as usize;
    let line_index = pc as usize;

    let abs_line = *((*p).abslineinfo.add(abs_index));
    let line_offset = *((*p).lineinfo.add(line_index)) as c_int;

    abs_line + line_offset
  }
}

pub use lua_g_getline as luaG_getline;
