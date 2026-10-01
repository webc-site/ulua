use crate::{
  functions::{buffer_window::buffer_data_ref, lua_pushlstring::lua_pushlstring_bytes},
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向本次 buffer 库调用的存活 `LuaState`，索引/长度实参按约定可读，栈顶有压入结果的余量。
pub(crate) unsafe fn buffer_tostring(l: *mut LuaState) -> i32 {
  // SAFETY: 契约保证 `l` 存活且 buffer 数据界自洽（切片长度即数据界），压回的串取 buffer 全长拷贝
  unsafe {
    let data = buffer_data_ref(l, 1);

    lua_pushlstring_bytes(l, data);

    1
  }
}

lua_lib_fn!(pub(crate) fn buffer_tostring, buffer_tostring_arm);
