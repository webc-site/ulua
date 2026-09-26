#[macro_export]
macro_rules! lua_istable {
  ($l:expr, $n:expr) => {
    // Safety: 展开点须保证 $l 指向存活 lua_State 且 $n 为合法栈索引（lua_type 只读该槽 tag）
    unsafe {
      $crate::functions::lua_type::lua_type($l, $n)
        == ($crate::enums::lua_type::LuaType::Table as i32)
    }
  };
}

pub use lua_istable;
