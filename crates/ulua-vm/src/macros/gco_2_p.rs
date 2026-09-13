#[macro_export]
macro_rules! gco2p {
  ($o:expr) => {{
    $crate::macros::check_exp::check_exp!(
      (*$o).gch.tt == ($crate::enums::lua_type::LuaType::Proto as u8),
      core::ptr::addr_of_mut!((*$o).p) as *mut $crate::records::proto::Proto
    )
  }};
}

pub use gco2p;
