#[macro_export]
macro_rules! ttisboolean {
  ($o:expr) => {
    $crate::macros::ttype::ttype!($o) == ($crate::enums::lua_type::LuaType::Boolean as i32)
  };
}

pub use ttisboolean;
