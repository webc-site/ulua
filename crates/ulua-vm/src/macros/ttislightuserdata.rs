#[macro_export]
macro_rules! ttislightuserdata {
  ($o:expr) => {
    $crate::macros::ttype::ttype!($o) == ($crate::enums::lua_type::LuaType::LightUserData as i32)
  };
}

pub use ttislightuserdata;
