#[macro_export]
macro_rules! equalobj {
  ($l:expr, $o1:expr, $o2:expr) => {
    ($crate::macros::ttype::ttype!($o1) == $crate::macros::ttype::ttype!($o2)
      && $crate::functions::lua_v_equalval::lua_v_equalval($l, $o1, $o2) != 0)
  };
}

pub use equalobj;
