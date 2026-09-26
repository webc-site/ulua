use ulua_vm::{
  functions::{lua_pushnil::lua_pushnil, lua_setfield::lua_setfield},
  macros::{lua_l_checkstring::luaL_checkstring, lua_pop::lua_pop},
  records::lua_state::LuaState,
};

use crate::functions::{
  cache_table_keys::REQUIRED_CACHE_TABLE_KEY, registry_table::push_registry_table,
};

/// 对应 cpp `luarequire_clearcacheentry`（Require.cpp 壳 + RequireImpl.cpp 实体
/// 两段）：Rust 侧无 TU 分离需求，壳与实体合一（review.md §3「只包一层的壳
/// 合并」）。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`，栈顶参数布局由 C 调用约定保证。
pub unsafe extern "C-unwind" fn luarequire_clearcacheentry(l: *mut LuaState) -> i32 {
  // Safety: l 是 VM 以 Lua/C API 调 C 函数时传入的存活 state；cache_key 由
  // luaL_checkstring 校验为字符串并返回指向该 VM 字符串的 NUL 结尾有效指针
  // （非字符串则抛错发散），本次调用内栈不 GC 该串；push_registry_table 门面的
  // 表键指针仅闭包内有效。
  unsafe {
    // cpp 使用 luaL_checkstring：非字符串参数直接报错，而不是把空指针交给
    // lua_setfield（UB）
    let cache_key = luaL_checkstring!(l, 1);

    push_registry_table(l, REQUIRED_CACHE_TABLE_KEY);
    lua_pushnil(l);
    // cache_key 已是 NUL 结尾的 VM 串指针，本函数唯一的 C 串直用点，无需再补 NUL
    lua_setfield(l, -2, cache_key);
    // 弹掉缓存表
    lua_pop(l, 1);
  }
  0
}
