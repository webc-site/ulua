#[macro_export]
macro_rules! ttisthread {
  ($o:expr) => {
    $crate::macros::ttype::ttype!($o) == ($crate::enums::lua_type::LuaType::Thread as i32)
  };
}

pub use ttisthread;
