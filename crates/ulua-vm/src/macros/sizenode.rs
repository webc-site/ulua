#[macro_export]
macro_rules! sizenode {
  ($t:expr) => {
    $crate::macros::twoto::twoto!((*$t).lsizenode)
  };
}

pub use sizenode;
