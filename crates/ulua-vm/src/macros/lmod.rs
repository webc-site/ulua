#[macro_export]
macro_rules! lmod {
  ($s:expr, $size:expr) => {
    $crate::macros::check_exp::check_exp!(
      ($size & ($size - 1)) == 0,
      ((($s) as i64) & ((($size) as i64) - 1)) as i32
    )
  };
}

pub use lmod;
