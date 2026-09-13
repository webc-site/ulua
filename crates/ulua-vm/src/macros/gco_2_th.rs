#[macro_export]
macro_rules! gco2th {
  ($o:expr) => {{
    $crate::macros::check_exp::check_exp!(
      (*$o).gch.tt == ($crate::enums::lua_type::LuaType::Thread as u8),
      core::ptr::addr_of_mut!((*$o).th) as *mut $crate::records::lua_state::lua_State
    )
  }};
}

pub use gco2th;
