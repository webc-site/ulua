#[macro_export]
macro_rules! l_isfalse {
  ($o:expr) => {
    (*$o).is_nil() || ((*$o).is_boolean() && !(*$o).as_boolean())
  };
}

pub use l_isfalse;
