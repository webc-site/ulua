use core::ffi::c_void;

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_tobuffer::lua_tobuffer, tag_error::tag_error},
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须是正在执行的 C 函数帧的存活 `LuaState`，`narg` 为其合法栈索引；槽位不是
/// buffer userdata 时经 `tag_error` 抛 Lua 错误、不返回。`len` 必须为可写 `usize` 槽
/// （lua_tobuffer 成功路径写入数据块长度）；返回指针指向该栈槽 userdata 的数据块，
/// 长度 `*len` 字节，在该 userdata 保持为栈槽值期间存活。cpp laux.cpp:150。
pub unsafe fn lua_l_checkbuffer(l: *mut LuaState, narg: i32, len: *mut usize) -> *mut c_void {
  // Safety: 契约保证 l 为存活调用帧、len 可写；非 buffer 槽位经 typeerror 抛错、不返回
  unsafe {
    let Some(b) = lua_tobuffer(l, narg, len) else {
      // Safety: 契约保证 l 为存活调用帧；非 buffer 槽位经 typeerror 抛错、发散不返回。
      tag_error(l, narg, LuaType::Buffer as i32);
    };

    b as *mut c_void
  }
}

// lualib.h name
