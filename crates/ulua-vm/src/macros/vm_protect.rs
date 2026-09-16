#[macro_export]
macro_rules! vm_protect {
  ($l:expr, $pc:expr, $base:expr, $x:expr) => {
    (*(*$l).ci).savedpc = $pc;
    {
      $x;
    };
    $base = (*$l).base;
  };
}

pub use vm_protect;
