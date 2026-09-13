#[macro_export]
macro_rules! fastmemset {
  ($dst:expr, $val:expr, $size:expr, $sizefast:expr) => {
    $crate::macros::check_exp::check_exp!(
      ($size) <= $sizefast,
      core::ptr::write_bytes($dst as *mut u8, b'0', $sizefast as usize)
    )
  };
}

pub use fastmemset;
