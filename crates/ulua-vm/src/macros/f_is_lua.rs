#[macro_export]
macro_rules! f_isLua {
  ($ci:expr) => {
    // C: `#define f_isLua(ci) (!ci_func(ci)->is_c)` — logical NOT, i.e. is_c == 0.
    (*$crate::macros::ci_func::ci_func!($ci)).is_c == 0
  };
}

pub use f_isLua;
