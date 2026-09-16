#[macro_export]
macro_rules! ttisuserdata {
  ($o:expr) => {
    $crate::macros::ttype::ttype!($o) == ($crate::enums::lua_type::LuaType::UserData as i32)
  };
}

pub use ttisuserdata;
