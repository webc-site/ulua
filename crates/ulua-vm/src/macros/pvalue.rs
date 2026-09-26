//! Source: `VM/src/lobject.h:73` (hand-ported)
// #define pvalue(o) check_exp(ttislightuserdata(o), (o)->value.p)
#[macro_export]
macro_rules! pvalue {
  ($o:expr) => {
    $crate::macros::check_exp::check_exp!((*$o).is_lightuserdata(), (*$o).value.p)
  };
}
pub use pvalue;
