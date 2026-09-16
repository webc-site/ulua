#[macro_export]
macro_rules! setthvalue {
  ($l:expr, $obj:expr, $x:expr) => {{
    let i_o = $obj as *mut $crate::type_aliases::t_value::TValue;
    (*i_o).value.gc =
      $crate::macros::cast_to::cast_to!(*mut $crate::records::gc_object::GCObject, $x);
    (*i_o).tt = $crate::enums::lua_type::LuaType::Thread as i32;
    $crate::macros::checkliveness::checkliveness!((*$l).global, i_o);
  }};
}

pub use setthvalue;
