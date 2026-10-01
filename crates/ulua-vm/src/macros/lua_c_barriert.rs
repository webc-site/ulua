//! Source: `VM/src/lgc.h`

/// lgc.h:97 写屏障的**谓词**部分：拆出来是为了让热派发 handler 只付判定成本，
/// 命中后再尾调用冷续延去跑 `lua_c_barriertable`（一次调用会把 `pc`/`base`/`k`/`cl`
/// 逼进入口栈帧，见 `luau_execute.rs` 的 `h_settablen`）。
#[macro_export]
macro_rules! luaC_barriert_pending {
  ($t:expr, $v:expr) => {
    // lgc.h:97
    $crate::macros::iscollectable::iscollectable!($v)
      && $crate::macros::isblack::isblack!($crate::macros::obj_2_gco::obj2gco!($t))
      && $crate::macros::iswhite::iswhite!($crate::macros::gcvalue::gcvalue!($v))
  };
}

#[macro_export]
macro_rules! luaC_barriert {
  ($l:expr, $t:expr, $v:expr) => {
    if $crate::macros::lua_c_barriert::luaC_barriert_pending!($t, $v) {
      $crate::functions::lua_c_barriertable::lua_c_barriertable(
        $l,
        $t,
        $crate::macros::gcvalue::gcvalue!($v),
      );
    }
  };
}

pub use luaC_barriert;
pub use luaC_barriert_pending;
