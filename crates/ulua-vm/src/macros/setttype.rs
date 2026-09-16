#[macro_export]
macro_rules! setttype {
  ($obj:expr, $tt:expr) => {
    (*$obj).set_tt($tt);
  };
}

pub use setttype;
