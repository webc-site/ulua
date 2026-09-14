//! Node: `cxx:Macro:Luau.VM:VM/src/lmem.h:15:lua_m_newarray`
//! Source: `VM/src/lmem.h:15` (hand-fixed: was a `()` placeholder)

#[macro_export]
macro_rules! luaM_newarray {
  ($l:expr, $n:expr, $t:ty, $memcat:expr) => {
    $crate::functions::lua_m_new::luaM_new_(
      $l,
      $crate::macros::lua_m_arraysize::luaM_arraysize!($l, $n as usize, core::mem::size_of::<$t>()),
      $memcat,
    ) as *mut $t
  };
}

pub use luaM_newarray;
