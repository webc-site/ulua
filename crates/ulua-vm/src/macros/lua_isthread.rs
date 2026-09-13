#[macro_export]
macro_rules! lua_isthread {
  ($l:expr, $n:expr) => {{
    let (l, n) = ($l, $n);
    (unsafe { $crate::functions::lua_type::lua_type(l, n) })
      == $crate::enums::lua_type::LuaType::Thread as i32
  }};
}

pub use lua_isthread;
