#[macro_export]
macro_rules! ttistable {
  ($o:expr) => {
    $crate::macros::ttype::ttype!($o) == ($crate::enums::lua_type::LuaType::Table as i32)
  };
}

pub use ttistable;
