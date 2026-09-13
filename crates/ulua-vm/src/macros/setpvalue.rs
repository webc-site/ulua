#[macro_export]
macro_rules! setpvalue {
  ($obj:expr, $x:expr, $tag:expr) => {{
    let i_o = $obj;
    (*i_o).value.p = $x;
    (*i_o).extra[0] = $tag;
    (*i_o).set_tt($crate::enums::lua_type::LuaType::LightUserData as i32);
  }};
}

pub use setpvalue;
