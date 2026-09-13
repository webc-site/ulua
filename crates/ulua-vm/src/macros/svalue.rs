#[macro_export]
macro_rules! svalue {
  ($o:expr) => {
    $crate::macros::getstr::getstr($crate::macros::tsvalue::tsvalue!($o))
  };
}

pub use svalue;
