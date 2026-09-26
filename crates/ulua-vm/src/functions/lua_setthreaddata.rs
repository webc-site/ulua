use core::ffi::c_void;

use crate::records::lua_state::LuaState;

/// 把 `data` 原样存入该线程的 `userdata` 数据槽。B 档契约前移（参照
/// `abs_index`/`lua_mainthread` 先例）：接收者取 `&mut LuaState`——本槽为普通
/// `*mut c_void` 字段（非 Cell），单字段写入经排他引用即可安全化，原 `l` 的
/// 存活+无别名契约改由调用点既有 unsafe 块内 `&mut *l` 引用重建承载（B 档
/// 纪律：净真实 unsafe 操作零增加）。`data` 允许为 NULL，仅原样存入——本端不
/// 追踪其生命周期，调用方须保证该指针在被读取前有效（或为 NULL）。
/// cpp `lapi.cpp:1393`。
pub fn lua_setthreaddata(l: &mut LuaState, data: *mut c_void) {
  l.userdata = data;
}
