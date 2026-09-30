#[macro_export]
macro_rules! isgray {
  ($x:expr) => {
    $crate::macros::testbits::testbits(
      (*$x).gch.marked as i32,
      $crate::macros::whitebits::WHITEBITS
        | $crate::macros::bitmask::bitmask($crate::macros::blackbit::BLACKBIT),
    ) == 0
  };
}

pub use isgray;
