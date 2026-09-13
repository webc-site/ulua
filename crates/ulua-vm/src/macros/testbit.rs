#[macro_export]
macro_rules! testbit {
  ($x:expr, $b:expr) => {
    $crate::macros::testbits::testbits($x, $crate::macros::bitmask::bitmask($b))
  };
}

pub use testbit;
