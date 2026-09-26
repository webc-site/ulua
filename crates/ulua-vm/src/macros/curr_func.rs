#[macro_export]
macro_rules! curr_func {
  ($l:expr) => {
    (*(*(*$l).ci).func).as_closure_ptr()
  };
}

pub use curr_func;
