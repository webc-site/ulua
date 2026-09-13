#[macro_export]
macro_rules! resetbit {
  ($x:expr, $b:expr) => {
    $crate::macros::resetbits::resetbits!($x, $crate::macros::bitmask::bitmask($b))
  };
}

pub use resetbit;
