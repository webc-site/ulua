pub const LUA_L_OPTSTRING: () = ();

#[macro_export]
macro_rules! luaL_optstring {
  ($l:expr, $n:expr, $d:expr) => {{
    let mut len: usize = 0;
    $crate::functions::lua_l_optlstring::lua_l_optlstring($l, $n, $d, &mut len as *mut usize)
  }};
}

pub use luaL_optstring;
