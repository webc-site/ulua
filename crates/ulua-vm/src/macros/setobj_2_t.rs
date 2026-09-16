#[macro_export]
macro_rules! setobj2t {
  ($l:expr, $obj1:expr, $obj2:expr) => {
    $crate::macros::setobj::setobj!($l, $obj1, $obj2);
  };
}

pub use setobj2t;
