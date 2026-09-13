#[macro_export]
macro_rules! setlvalue {
  ($obj:expr, $x:expr) => {{
    let i_o = $obj;
    (*i_o).value.l = $x;
    (*i_o).tt = $crate::enums::lua_type::LuaType::Integer as i32;
  }};
}

pub use setlvalue;
