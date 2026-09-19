#[macro_export]
macro_rules! lua_tointeger {
  ($l:expr, $i:expr) => {
    $crate::functions::lua_tointegerx::lua_tointegerx($l, $i, core::ptr::null_mut())
  };
}

pub use lua_tointeger;
