#[macro_export]
macro_rules! resetbits {
  ($x:expr, $m:expr) => {
    $x &= (!($m)) as u8;
  };
}

pub use resetbits;
