#[macro_export]
macro_rules! lightuserdatatag {
  ($o:expr) => {
    $crate::macros::check_exp::check_exp!(
      $crate::macros::ttislightuserdata::ttislightuserdata!($o),
      (*$o).extra[0]
    )
  };
}

pub use lightuserdatatag;
