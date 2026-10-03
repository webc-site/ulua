use crate::{functions::lua_l_checkvector::lua_l_checkvector, records::lua_state::LuaState};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；r16-v43 收形后
/// 判空经安全门面 `is_none_or_nil`、取向量经已收形的安全被调 `lua_l_checkvector`，体内无裸操作，
/// 故降为安全 `fn`）：`l` 须处于可抛错的受保护帧，`narg` 为合法栈索引：槽为 none/nil 时原样回退
/// `def`（可为 null，即 cpp `luaL_optvector(L, n, NULL)` 形——缺参路径不读该指针）；否则按
/// `lua_l_checkvector` 同形取向量（非 vector 抛错发散）。返回指针指向该栈槽 TValue 内联 vector
/// 数据，仅在该槽未被覆写、对象未被 GC 回收期间有效。cpp laux.cpp:274 `luaL_optvector`。
pub(crate) fn lua_l_optvector(l: &mut LuaState, narg: i32, def: *const f32) -> *const f32 {
  if l.is_none_or_nil(narg) {
    def
  } else {
    lua_l_checkvector(l, narg)
  }
}
