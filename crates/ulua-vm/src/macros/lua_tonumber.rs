#[macro_export]
/// cpp `lualib.h:lua_tonumber(L,i)` = `lua_tonumberx(L,(i),NULL)`：非数值回报 0。
macro_rules! lua_tonumber {
  ($l:expr, $i:expr) => {
    $crate::functions::lua_tonumberx::lua_tonumberx($l, $i).unwrap_or(0.0)
  };
}

pub use lua_tonumber;
