//! Source: `VM/include/lua.h:442` (hand-ported)
// #define lua_rawsetp(l, idx, p) lua_rawsetptagged(l, idx, p, 0)
#[macro_export]
macro_rules! lua_rawsetp {
  ($l:expr, $idx:expr, $p:expr) => {
    $crate::functions::lua_rawsetptagged::lua_rawsetptagged($l, $idx, $p, 0)
  };
}
pub use lua_rawsetp;
