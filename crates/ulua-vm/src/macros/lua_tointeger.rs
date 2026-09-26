#[macro_export]
/// cpp `lualib.h:lua_tointeger(L,i)` = `lua_tointegerx(L,(i),NULL)`：非数值回报 0。
macro_rules! lua_tointeger {
  ($l:expr, $i:expr) => {
    $crate::functions::lua_tointegerx::lua_tointegerx($l, $i).unwrap_or(0)
  };
}

pub use lua_tointeger;
