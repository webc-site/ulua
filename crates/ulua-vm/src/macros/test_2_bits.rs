#[macro_export]
macro_rules! test2bits {
  ($x:expr, $b1:expr, $b2:expr) => {
    $crate::macros::testbits::testbits($x, (1 << ($b1)) | (1 << ($b2)))
  };
}

pub use test2bits;
