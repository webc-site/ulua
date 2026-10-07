#[macro_export]
macro_rules! black2gray {
  ($x:expr) => {
    $crate::macros::resetbit::resetbit!((*$x).gch.marked, $crate::macros::maskmarks::BLACKBIT)
  };
}

pub use black2gray;
