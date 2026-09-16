#[macro_export]
macro_rules! ttisnumber {
  ($o:expr) => {
    $crate::macros::ttype::ttype!($o) == ($crate::enums::lua_type::LuaType::Number as i32)
  };
}

pub use ttisnumber;
