use crate::{
  functions::{lua_l_checkbuffer::lua_l_checkbuffer, lua_pushlstring::lua_pushlstring},
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向本次 buffer 库调用的存活 `LuaState`，索引/长度实参按约定可读，栈顶有压入结果的余量。
pub(crate) unsafe extern "C-unwind" fn buffer_tostring(l: *mut LuaState) -> i32 {
  // Safety: 契约保证 `l` 存活且 buffer 数据界自洽，压回的串取 buffer 全长拷贝
  unsafe {
    let mut len: usize = 0;
    let data = lua_l_checkbuffer(l, 1, &mut len);

    lua_pushlstring(l, data.cast(), len);

    1
  }
}
