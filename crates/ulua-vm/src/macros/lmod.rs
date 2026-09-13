#[macro_export]
macro_rules! lmod {
  ($s:expr, $size:expr) => {
    $crate::macros::check_exp::check_exp!(
      ($size & ($size - 1)) == 0,
      $crate::macros::cast_to::cast_to!(core::ffi::c_int, (($s) as i64) & ((($size) as i64) - 1))
    )
  };
}

pub use lmod;
