#[macro_export]
macro_rules! luaM_freegco {
  ($l:expr, $p:expr, $size:expr, $memcat:expr, $page:expr) => {
    $crate::functions::lua_m_freegco::luaM_freegco_(
      $l,
      $crate::macros::obj_2_gco::obj2gco!($p),
      $size,
      $memcat,
      $page,
    )
  };
}

pub use luaM_freegco;
