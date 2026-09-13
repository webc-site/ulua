#[macro_export]
macro_rules! setnvalue {
  ($obj:expr, $x:expr) => {{
    let i_o = $obj;
    (*i_o).value.n = $x;
    (*i_o).tt = $crate::enums::lua_type::LuaType::Number as i32;
  }};
}

pub use setnvalue;
