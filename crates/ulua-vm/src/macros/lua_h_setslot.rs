//! Source: `VM/src/ltable.h:35`
//! C++: `(invalidateTMcache(t), (slot == LUA_O_NILOBJECT ? luaH_newkey(l, t, key) : cast_to(TValue*, slot)))`

#[macro_export]
macro_rules! lua_h_setslot {
  ($l:expr, $t:expr, $slot:expr, $key:expr) => {{
    // `invalidate_tmcache` 已收 safe（&mut 接收者）；此处 `&mut *$t` 解引用与下方
    // `lua_h_newkey` 转调同规：展开点须处于 unsafe 上下文（现状即如此）。
    $crate::macros::invalidate_t_mcache::invalidate_tmcache(&mut *$t);
    if $slot == $crate::macros::lua_o_nilobject::LUA_O_NILOBJECT {
      $crate::functions::lua_h_newkey::lua_h_newkey($l, $t, $key)
    } else {
      $slot as *mut $crate::type_aliases::t_value::TValue
    }
  }};
}

pub use lua_h_setslot;
