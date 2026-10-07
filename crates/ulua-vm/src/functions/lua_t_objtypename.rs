use core::ffi::c_char;

use crate::{
  functions::lua_t_objtypenamestr::lua_t_objtypenamestr, macros::getstr::getstr,
  records::lua_state::LuaState, type_aliases::t_value::TValue,
};

/// `l` 须为存活 `LuaState`；`o` 须为存活 `TValue` 的非空共享只读借用（转交
/// `lua_t_objtypenamestr` 只读类型 tag 与 gc 指针，对 userdata 还会读其 metatable/`name`
/// 字段，故该对象须仍在 GC 存活集内），返回其类型名 C 串指针。
/// cpp `ltm.cpp:178`。
///
/// r19-w4 收形并降 safe：首参 `*mut LuaState` → `&LuaState`（本函数只把 `l` 透传给只读的
/// `lua_t_objtypenamestr`，从不写穿）；`o` 本就是 `&TValue`。收形后签名不再出现调用方传入的
/// 裸指针，依 §2 判例降 `fn`；体内对返回 `*const tstring` 的 `getstr` 解引用包于 `unsafe` 块。
pub(crate) fn lua_t_objtypename(l: &LuaState, o: &TValue) -> *const c_char {
  // SAFETY: `lua_t_objtypenamestr` 依契约返回存活 TString，`getstr` 仅取其柔性数组成员地址、
  // 返回的 C 串随该 TString 存活。
  unsafe { getstr(lua_t_objtypenamestr(l, o)) }
}
