#[macro_export]
macro_rules! api_incr_top {
  ($l:expr) => {{
    $crate::macros::api_check::api_check!($l, (*$l).top < (*(*$l).ci).top);
    (*$l).top = (*$l).top.add(1);
  }};
}

pub use api_incr_top;
