#[macro_export]
macro_rules! thvalue {
  ($o:expr) => {
    $crate::macros::check_exp::check_exp!(
      $crate::macros::ttisthread::ttisthread!($o),
      &mut (*(*$o).value.gc).th
    )
  };
}

pub use thvalue;
