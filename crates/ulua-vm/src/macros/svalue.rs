#[macro_export]
macro_rules! svalue {
  ($o:expr) => {
    $crate::macros::getstr::getstr((*$o).as_string())
  };
}

pub use svalue;
