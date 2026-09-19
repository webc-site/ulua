#[macro_export]
macro_rules! gco2class {
  ($o:expr) => {{
    $crate::macros::check_exp::check_exp!(
      (*$o).gch.tt == ($crate::enums::lua_type::LuaType::Class as u8),
      core::ptr::addr_of_mut!((*$o).lclass) as *mut $crate::records::luau_class::LuauClass
    )
  }};
}

pub use gco2class;
