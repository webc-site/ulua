#[macro_export]
macro_rules! lightuserdatatag {
  ($o:expr) => {
    $crate::macros::check_exp::check_exp!((*$o).is_lightuserdata(), (*$o).extra[0])
  };
}

pub use lightuserdatatag;
