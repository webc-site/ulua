#[macro_export]
macro_rules! luaM_reallocarray {
  ($l:expr, $v:expr, $oldn:expr, $n:expr, $t:ty, $memcat:expr) => {
    $v = $crate::macros::cast_to::cast_to!(
      *mut $t,
      $crate::functions::lua_m_realloc::lua_m_realloc_(
        $l,
        $v as *mut u8,
        ($oldn) * core::mem::size_of::<$t>(),
        $crate::macros::lua_m_arraysize::lua_m_arraysize!($l, $n, core::mem::size_of::<$t>()),
        $memcat as u8
      )
    )
  };
}
pub use luaM_reallocarray;
