#[macro_export]
macro_rules! gco2u {
  ($o:expr) => {{
    $crate::macros::check_exp::check_exp!(
      (*$o).gch.tt == ($crate::enums::lua_type::LuaType::UserData as u8),
      core::ptr::addr_of_mut!((*$o).u) as *mut $crate::records::udata::Udata
    )
  }};
}

pub use gco2u;
