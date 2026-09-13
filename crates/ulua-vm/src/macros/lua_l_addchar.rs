//! Source: `VM/include/lualib.h:104` (hand-ported)
// #define luaL_addchar(b, c) ((void)((b)->p < (b)->end || luaL_prepbuffsize(b, 1)), (*(b)->p++ = (char)(c)))
#[macro_export]
macro_rules! luaL_addchar {
  ($b:expr, $c:expr) => {{
    if !((*$b).p < (*$b).end) {
      $crate::functions::lua_l_prepbuffsize::lua_l_prepbuffsize($b, 1);
    }
    *(*$b).p = $c as core::ffi::c_char;
    (*$b).p = (*$b).p.add(1);
  }};
}
pub use luaL_addchar;
pub use luaL_addchar as lua_l_addchar;
