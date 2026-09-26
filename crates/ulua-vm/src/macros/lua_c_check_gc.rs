//! Source: `VM/src/lgc.h:77` (hand-ported)
// #define lua_c_check_gc(l)
//     { condhardstacktests(...); if (luaC_needsGC(l)) { condhardmemtests(...); lua_c_step(l, true); }
//       else { condhardmemtests(...); } }
// condhard*tests are no-ops in default builds.
#[macro_export]
macro_rules! lua_c_check_gc {
  ($l:expr) => {
    if $crate::macros::lua_c_needs_gc::luaC_needsGC!($l) {
      $crate::functions::lua_c_step::lua_c_step($l, true);
    }
  };
}
pub use lua_c_check_gc;
