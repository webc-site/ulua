//! Source: `VM/src/lgc.h:77` (hand-ported)
// #define luaC_checkGC(l)
//     { condhardstacktests(...); if (luaC_needsGC(l)) { condhardmemtests(...); luaC_step(l, true); }
//       else { condhardmemtests(...); } }
// condhard*tests are no-ops in default builds.
#[macro_export]
macro_rules! luaC_checkGC {
  ($l:expr) => {
    if $crate::macros::lua_c_needs_gc::luaC_needsGC!($l) {
      $crate::functions::lua_c_step::luaC_step($l, true);
    }
  };
}
pub use luaC_checkGC;
pub use luaC_checkGC as lua_c_check_gc;
