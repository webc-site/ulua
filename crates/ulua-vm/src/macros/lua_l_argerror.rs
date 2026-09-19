#[macro_export]
macro_rules! luaL_argerror {
  ($l:expr, $narg:expr, $extramsg:expr) => {
    $crate::functions::lua_l_argerror_l::luaL_argerrorL($l, $narg, $extramsg)
  };
}

pub use luaL_argerror;
