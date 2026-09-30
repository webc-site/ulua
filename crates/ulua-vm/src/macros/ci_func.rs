#[macro_export]
macro_rules! ci_func {
  ($ci:expr) => {
    (*(*$ci).func).as_closure_ptr()
  };
}

pub use ci_func;
