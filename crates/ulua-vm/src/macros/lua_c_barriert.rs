//! Source: `VM/src/lgc.h`

#[macro_export]
macro_rules! luaC_barriert {
  ($l:expr, $t:expr, $v:expr) => {
    // lgc.h:97
    if $crate::macros::iscollectable::iscollectable!($v)
      && $crate::macros::isblack::isblack!($t as *mut $crate::records::gc_object::GCObject)
      && $crate::macros::iswhite::iswhite!($crate::macros::gcvalue::gcvalue!($v))
    {
      $crate::functions::lua_c_barriertable::luaC_barriertable(
        $l,
        $t,
        $crate::macros::gcvalue::gcvalue!($v),
      );
    }
  };
}

pub use luaC_barriert;
