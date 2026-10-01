#[macro_export]
macro_rules! registry {
  ($l:expr) => {
    &(*(*$l).global).registry
  };
}

pub use registry;
