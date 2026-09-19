#[macro_export]
macro_rules! setnilvalue {
  ($obj:expr) => {
    (*$obj).tt = $crate::enums::lua_type::LuaType::Nil as i32;
  };
}

pub use setnilvalue;
