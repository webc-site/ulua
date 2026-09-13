#[macro_export]
macro_rules! gco2uv {
  ($o:expr) => {{
    $crate::macros::check_exp::check_exp!(
      (*$o).gch.tt == ($crate::enums::lua_type::LuaType::Upval as u8),
      core::ptr::addr_of_mut!((*$o).uv) as *mut $crate::records::up_val::UpVal
    )
  }};
}

pub use gco2uv;
