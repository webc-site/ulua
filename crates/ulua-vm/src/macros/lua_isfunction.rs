#[macro_export]
macro_rules! lua_isfunction {
  ($l:expr, $n:expr) => {
    $crate::functions::lua_type::lua_type($l, $n)
      == ($crate::enums::lua_type::LuaType::Function as i32)
  };
}

pub use lua_isfunction;
