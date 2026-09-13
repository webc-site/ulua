#[macro_export]
macro_rules! setvvalue {
  ($obj:expr, $x:expr, $y:expr, $z:expr, $w:expr) => {{
    let i_o: *mut $crate::type_aliases::t_value::TValue = $obj;
    let i_v = i_o as *mut f32;
    *i_v.add(0) = $x;
    *i_v.add(1) = $y;
    *i_v.add(2) = $z;
    *i_v.add(3) = $w;
    (*i_o).tt = $crate::enums::lua_type::LuaType::Vector as i32;
  }};
}

pub use setvvalue;
