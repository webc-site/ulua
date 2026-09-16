//! Source: `VM/include/lua.h:447` (hand-ported)
// #define lua_tostring(l, i) lua_tolstring(l, (i), NULL)
#[macro_export]
macro_rules! lua_tostring {
  ($l:expr, $i:expr) => {
    $crate::functions::lua_tolstring::lua_tolstring($l, $i, core::ptr::null_mut())
  };
}
pub use lua_tostring;
