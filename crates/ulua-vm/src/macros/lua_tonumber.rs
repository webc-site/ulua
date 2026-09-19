#[macro_export]
macro_rules! lua_tonumber {
  ($l:expr, $i:expr) => {
    $crate::functions::lua_tonumberx::lua_tonumberx($l, $i, core::ptr::null_mut())
  };
}

pub use lua_tonumber;
