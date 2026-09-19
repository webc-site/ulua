#[macro_export]
macro_rules! CHECK_IS_SUBTYPE {
  ($left:expr, $right:expr) => {
    let left_ty = $left;
    let right_ty = $right;
    let result = $crate::is_subtype(left_ty, right_ty);
    $crate::CHECK_MESSAGE!(result.is_subtype, "Expected {} <: {}", left_ty, right_ty);
  };
}

pub use CHECK_IS_SUBTYPE;
