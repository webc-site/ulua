#[macro_export]
macro_rules! luaL_argcheck {
  ($l:expr, $cond:expr, $arg:expr, $extramsg:expr) => {
    if !($cond) {
      $crate::macros::lua_l_argerror::luaL_argerror!($l, $arg, $extramsg);
    }
  };
}

pub use luaL_argcheck;
