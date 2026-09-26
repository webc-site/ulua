#[macro_export]
macro_rules! iscollectable {
  ($o:expr) => {
    $crate::macros::ttype::ttype!($o) >= ($crate::enums::lua_type::LuaType::String as u32)
  };
}

pub use iscollectable;
