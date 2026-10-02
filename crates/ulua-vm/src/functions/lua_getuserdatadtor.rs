use crate::{
  macros::{api_check::api_check, lua_utag_limit::LUA_UTAG_LIMIT},
  records::lua_state::LuaState,
  type_aliases::lua_destructor::LuaDestructor,
};

/// `lua_getuserdatadtor`（cpp/VM/src/lapi.cpp 同名）：读回 `tag` 登记的全局 userdata
/// 析构函数。调用序契约（正确性，非内存安全；r16-v4 引用形前移，`l` 存活由
/// `&LuaState` 类型承载）：`tag` 须 `< LUA_UTAG_LIMIT`（`api_check` debug 断言；
/// release 越界由数组安全索引 panic 兜住，即调用方违约当场响亮失败，无静默错读）。
/// 本体为纯读数：`gs_ref` 只读视图一句一借取回注册槽值，unsafe 消亡。
pub fn lua_getuserdatadtor(l: &LuaState, tag: i32) -> LuaDestructor {
  api_check!(l, (tag as u32) < LUA_UTAG_LIMIT as u32);

  // r16-b1 收编形制保持：读数经 gs_ref 只读视图，同指针同值（见其契约）；
  // r16-v4：`(*l)` 裸解引用由 `&LuaState` 类型承载消亡，一句一借、视图不出本句
  l.gs_ref().udatagc[tag as usize]
}
