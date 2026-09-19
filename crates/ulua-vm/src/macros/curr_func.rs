#[macro_export]
macro_rules! curr_func {
  ($l:expr) => {
    $crate::macros::clvalue::clvalue!((*(*$l).ci).func)
  };
}

pub use curr_func;
