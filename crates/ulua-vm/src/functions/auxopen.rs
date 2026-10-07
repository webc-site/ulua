use crate::{records::lua_state::LuaState, type_aliases::lua_c_function::LuaCFunction};

/// # Safety
/// `l` 必须是库 open 期间存活的调用帧，且栈顶（-2 处）已压入待填充的库表，供 `set_field_bytes` 写入；
/// `f`/`u` 为合法 C 函数指针。`name` 为不含 `\0` 的名字切片，仅本次调用期借用
/// （debugname 与字段键均由 VM 当场复制/驻留）。cpp lbaselib.cpp:453。
pub(crate) unsafe fn auxopen(l: *mut LuaState, name: &'static [u8], f: LuaCFunction, u: LuaCFunction) {
  // SAFETY: 契约保证 `L` 为库 open 期的存活调用帧，luaL_requiref 式注册仅在当前栈顶进行
  unsafe {
    (*l).push_c_function(u, None);
    (*l).push_c_closure(f, Some(name), 1);
    (*l).set_field_bytes(-2, name);
  }
}
