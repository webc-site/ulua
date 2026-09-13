macro_rules! luaD_checkstack {
  ($l:expr, $n:expr) => {
    if $crate::macros::stacklimitreached::stacklimitreached($l, $n) {
      $crate::functions::lua_d_growstack::lua_d_growstack($l, $n);
    } else {
      $crate::macros::condhardstacktests::condhardstacktests!(
        $crate::functions::lua_d_reallocstack::luaD_reallocstack(
          $l,
          $l.stacksize - $crate::macros::extra_stack::EXTRA_STACK,
          0
        )
      );
    }
  };
}

pub(crate) use luaD_checkstack;
