#[macro_export]
macro_rules! uvalue {
  ($o:expr) => {
    $crate::macros::check_exp::check_exp!(
      $crate::macros::ttisuserdata::ttisuserdata!($o),
      &(*(*$o).value.gc).u
    )
  };
}

pub use uvalue;
