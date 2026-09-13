#[macro_export]
macro_rules! luaL_typeerror {
  ($l:expr, $narg:expr, $tname:expr) => {
    $crate::functions::lua_l_typeerror_l::lua_l_typeerror_l($l, $narg, $tname)
  };
}

pub use luaL_typeerror;
