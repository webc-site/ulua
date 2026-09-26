use crate::records::lua_state::LuaState;

/// 设置单步执行开关。B 档契约前移（参照 `abs_index`/`lua_setthreaddata` 先例）：
/// 接收者取 `&mut LuaState`——`singlestep` 为普通 `bool` 字段（非 Cell），单字段
/// 写入经排他引用即可安全化，原 `l` 的存活契约改由调用点既有 unsafe 块内
/// `&mut *l` 引用重建承载（B 档纪律：净真实 unsafe 操作零增加）。仅写
/// `singlestep` 一个布尔字段，不触碰其它内存。cpp `ldebug.cpp:482`。
pub fn lua_singlestep(l: &mut LuaState, enabled: i32) {
  l.singlestep = enabled != 0;
}
