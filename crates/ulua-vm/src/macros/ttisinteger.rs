#[macro_export]
macro_rules! ttisinteger {
  ($o:expr) => {
    $crate::macros::ttype::ttype!($o) == ($crate::enums::lua_type::LuaType::Integer as i32)
  };
}

pub use ttisinteger;
