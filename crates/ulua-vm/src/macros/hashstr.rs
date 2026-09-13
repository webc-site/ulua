#[macro_export]
macro_rules! hashstr {
  ($t:expr, $str:expr) => {
    $crate::macros::hashpow_2::hashpow2!($t, (*$str).hash)
  };
}

pub use hashstr;
