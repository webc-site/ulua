#[macro_export]
macro_rules! gco2object {
  ($o:expr) => {{
    $crate::macros::check_exp::check_exp!(
      (*$o).gch.tt == ($crate::enums::lua_type::LuaType::Object as u8),
      core::ptr::addr_of_mut!((*$o).lobject) as *mut $crate::records::luau_object::LuauObject
    )
  }};
}

pub use gco2object;
