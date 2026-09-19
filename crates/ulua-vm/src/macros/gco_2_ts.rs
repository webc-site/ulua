#[macro_export]
macro_rules! gco2ts {
  ($o:expr) => {
    $crate::macros::check_exp::check_exp!(
      (*$o).gch.tt == ($crate::enums::lua_type::LuaType::String as u8),
      &(*$o).ts
    )
  };
}

pub use gco2ts;
