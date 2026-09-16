#[macro_export]
macro_rules! ttisclass {
  ($o:expr) => {
    $crate::macros::ttype::ttype!($o) == ($crate::enums::lua_type::LuaType::Class as i32)
  };
}

pub use ttisclass;
