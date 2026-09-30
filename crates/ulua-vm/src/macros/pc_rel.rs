#[macro_export]
macro_rules! pcRel {
  ($pc:expr, $p:expr) => {
    if !$pc.is_null() && $pc != (*$p).code {
      ((($pc as usize).wrapping_sub((*$p).code as usize) / core::mem::size_of::<u32>()) as i32) - 1
    } else {
      0
    }
  };
}

pub use pcRel;
