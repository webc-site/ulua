use crate::{
  macros::{api_check::api_check, lua_utag_limit::LUA_UTAG_LIMIT},
  records::lua_state::LuaState,
  type_aliases::lua_destructor::LuaDestructor,
};

/// `lua_setuserdatadtor`（cpp/VM/src/lapi.cpp 同名）：把 `dtor` 登记为 `tag` 的全局
/// userdata 析构。调用序契约（正确性，非内存安全；r16-v4 引用形前移，`l` 存活与
/// 独占由 `&mut LuaState` 类型承载）：`tag` 须 `< LUA_UTAG_LIMIT`（`api_check` debug
/// 断言；release 越界由数组安全索引 panic 兜住，即调用方违约当场响亮失败）；`dtor`
/// 为可空合法函数指针（登记 API 语义，写入即覆注册槽）。
pub fn lua_setuserdatadtor(l: &mut LuaState, tag: i32, dtor: LuaDestructor) {
  api_check!(l, (tag as u32) < LUA_UTAG_LIMIT as u32);
  // r16-b3 收编形制保持：写点经 gs_mut 一句一借（登记 API，无重入穿插）；
  // r16-v4：外层 `(*l)` 裸解引用与包裹 unsafe 由 `&mut LuaState` 类型承载消亡
  l.gs_mut().udatagc[tag as usize] = dtor;
}
