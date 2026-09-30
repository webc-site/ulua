#[macro_export]
macro_rules! makewhite {
  ($g:expr, $x:expr) => {
    (*$x).gch.marked = (((*$x).gch.marked & $crate::macros::maskmarks::maskmarks!())
      | $crate::macros::lua_c_white::luaC_white!($g)) as u8;
  };
}

pub use makewhite;
