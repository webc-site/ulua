#[macro_export]
macro_rules! api_checknelems {
  ($l:expr, $n:expr) => {
    $crate::macros::api_check::api_check!($l, ($n) as isize <= (*$l).top.offset_from((*$l).base));
  };
}

pub use api_checknelems;
