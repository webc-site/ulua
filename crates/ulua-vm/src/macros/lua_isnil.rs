#[macro_export]
macro_rules! lua_isnil {
  ($l:expr, $n:expr) => {
    $crate::functions::lua_type::lua_type($l, $n) == ($crate::enums::lua_type::LuaType::Nil as i32)
  };
}

pub use lua_isnil;
