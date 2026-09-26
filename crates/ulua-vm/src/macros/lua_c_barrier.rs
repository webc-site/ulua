//! Source: `VM/src/lgc.h`

// VM/src/lgc.h:91 —
// #define lua_c_barrier(l, p, v)
//     { if (iscollectable(v) && isblack(obj2gco(p)) && iswhite(gcvalue(v)))
//           lua_c_barrierf(l, obj2gco(p), gcvalue(v)); }
// obj2gco on a typed GC pointer is a plain cast in C.
#[macro_export]
macro_rules! lua_c_barrier {
  ($l:expr, $p:expr, $v:expr) => {
    if $crate::macros::iscollectable::iscollectable!($v)
      && $crate::macros::isblack::isblack!($crate::macros::obj_2_gco::obj2gco!($p))
      && $crate::macros::iswhite::iswhite!($crate::macros::gcvalue::gcvalue!($v))
    {
      $crate::functions::lua_c_barrierf::lua_c_barrierf(
        $l,
        $crate::macros::obj_2_gco::obj2gco!($p),
        $crate::macros::gcvalue::gcvalue!($v),
      );
    }
  };
}

pub use lua_c_barrier;
