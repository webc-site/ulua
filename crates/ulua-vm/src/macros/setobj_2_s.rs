#[macro_export]
macro_rules! setobj_2_s {
  ($l:expr, $obj1:expr, $obj2:expr) => {
    $crate::macros::setobj::setobj!($l, $obj1, $obj2)
  };
}

pub use setobj_2_s;
// C name
pub use setobj_2_s as setobj2s;
