#[macro_export]
macro_rules! expandstacklimit {
  ($l:expr, $p:expr) => {{
    ulua_common::LUAU_ASSERT!(($p) <= (*$l).stack_last);
    if (*(*$l).ci).top < ($p) {
      (*(*$l).ci).top = ($p);
    }
  }};
}

pub use expandstacklimit;
