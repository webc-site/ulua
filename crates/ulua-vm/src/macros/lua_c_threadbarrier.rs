#[macro_export]
macro_rules! lua_c_threadbarrier {
  ($l:expr) => {{
    let obj = $crate::macros::obj_2_gco::obj2gco!($l);
    if $crate::macros::isblack::isblack!(obj) {
      $crate::functions::lua_c_barrierback::lua_c_barrierback($l, obj, &mut (*$l).gclist);
    }
  }};
}

pub use lua_c_threadbarrier;
// C name
pub use lua_c_threadbarrier as luaC_threadbarrier;
