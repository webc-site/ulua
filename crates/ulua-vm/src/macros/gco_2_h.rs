#[macro_export]
macro_rules! gco2h {
  ($o:expr) => {{
    $crate::macros::check_exp::check_exp!(
      (*$o).gch.tt == ($crate::enums::lua_type::LuaType::Table as u8),
      core::ptr::addr_of_mut!((*$o).h) as *mut $crate::records::lua_table::LuaTable
    )
  }};
}

pub use gco2h;
