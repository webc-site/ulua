#[macro_export]
macro_rules! tostring {
  ($l:expr, $o:expr) => {
    ($crate::macros::ttype::ttype!($o) == ($crate::enums::lua_type::LuaType::String as i32))
      || ($crate::functions::lua_v_tostring::lua_v_tostring($l, $o) != 0)
  };
}

pub use tostring;
