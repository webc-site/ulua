#[macro_export]
macro_rules! ttisupval {
  ($o:expr) => {
    $crate::macros::ttype::ttype!($o) == ($crate::enums::lua_type::LuaType::Upval as i32)
  };
}

pub use ttisupval;
