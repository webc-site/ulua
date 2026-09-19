#[macro_export]
macro_rules! luaL_argexpected {
  ($l:expr, $cond:expr, $arg:expr, $tname:expr) => {
    if !($cond) {
      $crate::luaL_typeerror!($l, $arg, $tname);
    }
  };
}

pub use luaL_argexpected;
