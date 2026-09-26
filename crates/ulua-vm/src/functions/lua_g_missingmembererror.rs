use crate::{
  functions::{cstr_cow, lua_t_objtypename::lua_t_objtypename},
  macros::{getstr::getstr, lua_g_runerror::lua_g_runerror},
  records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};

/// # Safety
/// `l` 须为存活调用帧，`p1/p2` 须为可读对齐 `TValue`（cpp ldebug.cpp 报错路径）：仅读取二者
/// tag/值以取类型名与键串，`p2` 为串时才经 `tsvalue!` 解引用取内容；末尾抛错不返回。
pub unsafe fn lua_g_missingmembererror(
  l: *mut LuaState,
  p1: *const TValue,
  p2: *const TValue,
) -> ! {
  // Safety: 契约保证 `l` 为存活调用帧、self 的类元信息可读、name 为存活串；错误经 luaG 路径抛出不返回
  unsafe {
    if !(*p2).is_string() {
      let t1 = lua_t_objtypename(l, p1);
      let t2 = lua_t_objtypename(l, p2);
      lua_g_runerror!(l, "cannot index {} with a {}", cstr_cow(t1), cstr_cow(t2),)
    } else {
      let t1 = lua_t_objtypename(l, p1);
      let key = (*p2).as_string_ptr();
      lua_g_runerror!(
        l,
        "this {} does not have a key named '{}'",
        cstr_cow(t1),
        cstr_cow(getstr(key)),
      )
    }
  }
}
