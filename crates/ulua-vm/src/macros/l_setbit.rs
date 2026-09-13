#[macro_export]
macro_rules! l_setbit {
  ($x:expr, $b:expr) => {
    $crate::macros::setbits::setbits!($x, $crate::macros::bitmask::bitmask($b))
  };
}

pub use l_setbit;
