#[macro_export]
macro_rules! setupvalue {
  ($l:expr, $obj:expr, $x:expr) => {{
    let i_o: *mut $crate::type_aliases::t_value::TValue = $obj;
    (*i_o).value.gc =
      $crate::macros::cast_to::cast_to!(*mut $crate::records::gc_object::GCObject, $x);
    (*i_o).tt = $crate::enums::lua_type::LuaType::Upval as core::ffi::c_int;
    $crate::macros::checkliveness::checkliveness!((*$l).global, i_o);
  }};
}

pub use setupvalue;
