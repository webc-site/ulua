#[macro_export]
macro_rules! makewhite {
  ($g:expr, $x:expr) => {
    (*$x).gch.marked = $crate::macros::cast_byte::cast_byte!(
      ((*$x).gch.marked & $crate::macros::maskmarks::maskmarks!())
        | $crate::macros::lua_c_white::luaC_white!($g)
    )
  };
}

pub use makewhite;
