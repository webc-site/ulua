#[macro_export]
macro_rules! lvalue {
  ($o:expr) => {
    $crate::macros::check_exp::check_exp!(
      $crate::macros::ttisinteger::ttisinteger!($o),
      (*$o).value.l
    )
  };
}

pub use lvalue;
