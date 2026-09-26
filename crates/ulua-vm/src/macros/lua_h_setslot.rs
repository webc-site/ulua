//! Source: `VM/src/ltable.h:35`
//! C++: `(invalidateTMcache(t), (slot == LUA_O_NILOBJECT ? luaH_newkey(l, t, key) : cast_to(TValue*, slot)))`

#[macro_export]
macro_rules! lua_h_setslot {
  ($l:expr, $t:expr, $slot:expr, $key:expr) => {{
    $crate::macros::invalidate_t_mcache::invalidate_tmcache($t);
    if $slot == $crate::macros::lua_o_nilobject::LUA_O_NILOBJECT {
      $crate::functions::lua_h_newkey::lua_h_newkey($l, $t, $key)
    } else {
      $slot as *mut $crate::type_aliases::t_value::TValue
    }
  }};
}

pub use lua_h_setslot;
