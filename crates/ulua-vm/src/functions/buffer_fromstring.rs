use core::ptr::copy_nonoverlapping;

use crate::{
  functions::{lua_l_checklstring::lua_l_checklstring, lua_newbuffer::lua_newbuffer},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向本次 buffer 库调用的存活 `LuaState`，索引/长度实参按约定可读，栈顶有压入结果的余量。
pub(crate) unsafe fn buffer_fromstring(l: *mut LuaState) -> i32 {
  // Safety: 契约保证 `l` 存活且源串数据可读，新 buffer 按串长分配并整段复制
  unsafe {
    let mut len: usize = 0;
    let val = lua_l_checklstring(l, 1, &mut len);

    let data = lua_newbuffer(l, len);
    copy_nonoverlapping(val as *const u8, data as *mut u8, len);

    1
  }
}

lua_lib_fn!(pub(crate) fn buffer_fromstring, buffer_fromstring_arm);
