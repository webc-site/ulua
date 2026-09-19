#[macro_export]
macro_rules! ttisfunction {
  ($o:expr) => {
    $crate::macros::ttype::ttype!($o) == ($crate::enums::lua_type::LuaType::Function as i32)
  };
}

pub use ttisfunction;
