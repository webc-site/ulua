use crate::{
  macros::{api_check::api_check, lua_utag_limit::LUA_UTAG_LIMIT},
  records::lua_state::LuaState,
  type_aliases::lua_destructor::LuaDestructor,
};

/// # Safety
///
/// `l` 必须指向存活 `LuaState`，索引/长度/标签等参数满足各 API 注释约定，需压栈时栈顶预留由调用方保证。
pub unsafe fn lua_getuserdatadtor(l: *mut LuaState, tag: i32) -> LuaDestructor {
  api_check!(l, (tag as u32) < LUA_UTAG_LIMIT as u32);

  // Safety: 契约保证 `l` 的 global 存活且 tag 落在 udatagc 注册数组界内，取回的析构指针为登记原值
  unsafe { (*(*l).global).udatagc[tag as usize] }
}
