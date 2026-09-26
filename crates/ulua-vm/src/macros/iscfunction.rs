#[macro_export]
macro_rules! iscfunction {
  ($o:expr) => {
    (*$o).is_function() && (*$o).as_closure().is_c != 0
  };
}

pub use iscfunction;
