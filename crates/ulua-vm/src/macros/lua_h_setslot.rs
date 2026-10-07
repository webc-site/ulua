//! Source: `VM/src/ltable.h:35`
//! C++: `(invalidateTMcache(t), (slot == LUA_O_NILOBJECT ? luaH_newkey(l, t, key) : cast_to(TValue*, slot)))`

#[macro_export]
macro_rules! lua_h_setslot {
  ($l:expr, $t:expr, $slot:expr, $key:expr) => {{
    // `invalidate_tmcache` 已收 safe（`&LuaTable` 接收者，tmcache 为 Cell 内部可变）；
    // 此处 `&*$t` 解引用与下方
    // `lua_h_newkey` 转调同规：展开点须处于 unsafe 上下文（现状即如此）。
    $crate::macros::invalidate_t_mcache::invalidate_tmcache(&*$t);
    // 表写咽喉：活键复用槽（值将由调用层写，nil↔非 nil 改变存在性）失效 __index
    // 链多点缓存；newkey 分支由 lua_h_newkey 内的失效点覆盖，此处冗余一次自增无妨
    // （写侧慢路，成本可忽略）。index_chain_cache 正确性契约。
    $crate::functions::index_chain_cache::index_chain_write($t);
    if $slot == $crate::macros::lua_o_nilobject::LUA_O_NILOBJECT {
      $crate::functions::lua_h_newkey::lua_h_newkey($l, $t, $key)
    } else {
      $slot as *mut $crate::type_aliases::t_value::TValue
    }
  }};
}

pub use lua_h_setslot;
