#[macro_export]
macro_rules! lua_isinteger_64 {
  ($l:expr, $n:expr) => {
    $crate::functions::lua_type::lua_type($l, $n)
      == ($crate::enums::lua_type::LuaType::Integer as i32)
  };
}

pub use lua_isinteger_64;
