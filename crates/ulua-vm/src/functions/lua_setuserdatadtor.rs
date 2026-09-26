use crate::{
  macros::{api_check::api_check, lua_utag_limit::LUA_UTAG_LIMIT},
  records::lua_state::LuaState,
  type_aliases::lua_destructor::LuaDestructor,
};

/// # Safety
///
/// `l` 必须指向存活 `LuaState`，索引/长度/标签等参数满足各 API 注释约定，需压栈时栈顶预留由调用方保证。
pub unsafe fn lua_setuserdatadtor(l: *mut LuaState, tag: i32, dtor: LuaDestructor) {
  api_check!(l, (tag as u32) < LUA_UTAG_LIMIT as u32);
  // Safety: 契约保证 `l` 的 global 存活、tag 在 udatagc 注册界内且 dtor 为可空合法函数指针，仅覆写注册槽
  unsafe {
    (*(*l).global).udatagc[tag as usize] = dtor;
  }
}
