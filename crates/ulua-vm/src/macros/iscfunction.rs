#[macro_export]
macro_rules! iscfunction {
  ($o:expr) => {
    $crate::macros::ttype::ttype!($o) == ($crate::enums::lua_type::LuaType::Function as i32)
      && (*$crate::macros::clvalue::clvalue!($o)).is_c != 0
  };
}

pub use iscfunction;
