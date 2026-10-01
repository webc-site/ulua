#[macro_export]
macro_rules! reset2bits {
  ($x:expr, $b1:expr, $b2:expr) => {
    $crate::macros::resetbits::resetbits!($x, (1 << ($b1)) | (1 << ($b2)))
  };
}

pub use reset2bits;
