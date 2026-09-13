#[macro_export]
macro_rules! incr_top {
  ($l:expr) => {{
    $crate::macros::lua_d_checkstack::luaD_checkstack!($l, 1);
    (*$l).top = (*$l).top.add(1);
  }};
}

pub use incr_top;
