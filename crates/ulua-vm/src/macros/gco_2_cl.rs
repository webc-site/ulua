#[macro_export]
macro_rules! gco2cl {
  ($o:expr) => {{
    $crate::macros::check_exp::check_exp!(
      (*$o).gch.tt == ($crate::enums::lua_type::LuaType::Function as u8),
      core::ptr::addr_of_mut!((*$o).cl) as *mut $crate::records::closure::Closure
    )
  }};
}

pub use gco2cl;
