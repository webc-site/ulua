#[macro_export]
macro_rules! gco2buf {
  ($o:expr) => {
    $crate::macros::check_exp::check_exp!(
      (*$o).gch.tt == ($crate::enums::lua_type::LuaType::Buffer as u8),
      &(*$o).buf
    )
  };
}

pub use gco2buf;
