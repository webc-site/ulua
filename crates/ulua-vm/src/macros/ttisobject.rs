#[macro_export]
macro_rules! ttisobject {
  ($o:expr) => {
    $crate::macros::ttype::ttype!($o) == ($crate::enums::lua_type::LuaType::Object as i32)
  };
}

pub use ttisobject;
