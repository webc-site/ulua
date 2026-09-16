//! Node: `cxx:Macro:Luau.VM:VM/src/ltable.h:35:luaH_setslot`
//! C++: `(invalidateTMcache(t), (slot == luaO_nilobject ? luaH_newkey(l, t, key) : cast_to(TValue*, slot)))`

#[macro_export]
macro_rules! luaH_setslot {
  ($l:expr, $t:expr, $slot:expr, $key:expr) => {{
    $crate::macros::invalidate_t_mcache::invalidate_tmcache($t);
    if $slot == $crate::macros::lua_o_nilobject::luaO_nilobject {
      $crate::functions::lua_h_newkey::lua_h_newkey($l, $t, $key)
    } else {
      $slot as *mut $crate::type_aliases::t_value::TValue
    }
  }};
}

pub use luaH_setslot;
pub use luaH_setslot as lua_h_setslot;
