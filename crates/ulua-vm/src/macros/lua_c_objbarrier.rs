//! Source: `VM/src/lgc.h:109` (hand-ported)
// #define lua_c_objbarrier(l, p, o)
//     { if (isblack(obj2gco(p)) && iswhite(obj2gco(o))) lua_c_barrierf(l, obj2gco(p), obj2gco(o)); }
// obj2gco on typed GC pointers is a plain cast in C.
#[macro_export]
macro_rules! lua_c_objbarrier {
  ($l:expr, $p:expr, $o:expr) => {
    if $crate::macros::isblack::isblack!($crate::macros::obj_2_gco::obj2gco!($p))
      && $crate::macros::iswhite::iswhite!($crate::macros::obj_2_gco::obj2gco!($o))
    {
      $crate::functions::lua_c_barrierf::lua_c_barrierf(
        $l,
        $crate::macros::obj_2_gco::obj2gco!($p),
        $crate::macros::obj_2_gco::obj2gco!($o),
      );
    }
  };
}
pub use lua_c_objbarrier;
