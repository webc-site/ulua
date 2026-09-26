#[macro_export]
macro_rules! lvalue {
  ($o:expr) => {
    $crate::macros::check_exp::check_exp!((*$o).is_integer(), (*$o).value.l)
  };
}

pub use lvalue;
