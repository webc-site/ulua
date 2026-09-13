#[macro_export]
macro_rules! luaL_checkstring {
  ($l:expr, $n:expr) => {{
    let mut len: usize = 0;
    $crate::functions::lua_l_checklstring::lua_l_checklstring($l, $n, &mut len as *mut usize)
  }};
}

pub use luaL_checkstring;
