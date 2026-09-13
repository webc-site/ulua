#[macro_export]
macro_rules! lua_isnoneornil {
  ($l:expr, $n:expr) => {
    $crate::functions::lua_type::lua_type($l, $n)
      <= ($crate::enums::lua_type::LuaType::Nil as ::core::ffi::c_int)
  };
}

pub use lua_isnoneornil;
