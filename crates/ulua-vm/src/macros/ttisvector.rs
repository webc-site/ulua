#[macro_export]
macro_rules! ttisvector {
  ($o:expr) => {
    $crate::macros::ttype::ttype!($o) == ($crate::enums::lua_type::LuaType::Vector as i32)
  };
}

pub use ttisvector;
