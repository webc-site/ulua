//! Source: `VM/src/lgc.h`

#[macro_export]
macro_rules! luaC_barriert {
  ($l:expr, $t:expr, $v:expr) => {
    // lgc.h:97
    if $crate::macros::iscollectable::iscollectable!($v)
      && $crate::macros::isblack::isblack!($crate::macros::obj_2_gco::obj2gco!($t))
      && $crate::macros::iswhite::iswhite!($crate::macros::gcvalue::gcvalue!($v))
    {
      $crate::functions::lua_c_barriertable::lua_c_barriertable(
        $l,
        $t,
        $crate::macros::gcvalue::gcvalue!($v),
      );
    }
  };
}

pub use luaC_barriert;
