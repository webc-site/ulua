use core::ffi::c_void;

use crate::records::lua_state::LuaState;

/// 读该线程的 `userdata` 数据槽（与 `lua_setthreaddata` 成对：读侧 `&`、写侧
/// `&mut`，借用互斥天然衔接）。B 档契约前移（参照 `abs_index`/`lua_mainthread`/
/// `lua_status` 先例）：原 `*mut LuaState` 存活契约改由 `&` 接收者的引用有效性
/// 规则在调用点承载；本体仅读 `userdata` 一个字段、零 unsafe 操作，签名转 safe
/// fn。返回的裸指针不追踪生命周期，调用方按各数据槽布线的注册契约解引用。
/// cpp `lapi.cpp:1388`。
pub fn lua_getthreaddata(l: &LuaState) -> *mut c_void {
  l.userdata
}
