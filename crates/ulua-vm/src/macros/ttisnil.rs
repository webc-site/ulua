#[macro_export]
macro_rules! ttisnil {
  ($o:expr) => {
    $crate::macros::ttype::ttype!($o) == ($crate::enums::lua_type::LuaType::Nil as i32)
  };
}

pub use ttisnil;
