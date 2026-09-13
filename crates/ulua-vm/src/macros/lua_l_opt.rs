#[macro_export]
macro_rules! luaL_opt {
  ($l:expr, $f:expr, $n:expr, $d:expr) => {
    if $crate::macros::lua_isnoneornil::lua_isnoneornil!($l, $n) {
      $d
    } else {
      $f($l, $n)
    }
  };
}

pub use luaL_opt;
