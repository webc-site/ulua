#[macro_export]
macro_rules! setbvalue {
  ($obj:expr, $b:expr) => {{
    let i_o = $obj;
    (*i_o).value.b = ($b) as i32;
    (*i_o).tt = $crate::enums::lua_type::LuaType::Boolean as i32;
  }};
}

pub use setbvalue;
