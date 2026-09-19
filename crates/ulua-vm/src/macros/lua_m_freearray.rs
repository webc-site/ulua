#[macro_export]
macro_rules! luaM_freearray {
  ($l:expr, $b:expr, $n:expr, $t:ty, $memcat:expr) => {
    $crate::functions::lua_m_free::luaM_free_(
      $l,
      $b as *mut u8,
      ($n as usize) * core::mem::size_of::<$t>(),
      $memcat as u8,
    )
  };
}
pub use luaM_freearray;
