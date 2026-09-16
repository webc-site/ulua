#[macro_export]
macro_rules! ttisbuffer {
  ($o:expr) => {
    $crate::macros::ttype::ttype!($o) == ($crate::enums::lua_type::LuaType::Buffer as i32)
  };
}

pub use ttisbuffer;
