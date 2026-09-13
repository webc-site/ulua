//! Node: `cxx:Macro:Luau.VM:VM/src/lstate.h:incr_ci` (hand-checked)
//! C++ `incr_ci(l)` yields the NEW CallInfo: `luaD_growCI` advances `l->ci`
//! internally; the fast path advances it inline. Both branches end with the
//! macro evaluating to `(*l).ci` (the C++ ternary returned it directly; the
//! original Rust translation returned incompatible branch types and could
//! never have expanded).

#[macro_export]
macro_rules! incr_ci {
  ($l:expr) => {{
    let l = $l;
    if (*l).ci == (*l).end_ci {
      $crate::functions::lua_d_grow_ci::luaD_growCI(l);
    } else {
      $crate::macros::condhardstacktests::condhardstacktests!(
        $crate::functions::lua_d_realloc_ci::luaD_reallocCI(l, (*l).size_ci)
      );
      (*l).ci = (*l).ci.wrapping_add(1);
    }
    (*l).ci
  }};
}

pub use incr_ci;
