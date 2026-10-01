#[macro_export]
macro_rules! isLua {
  ($ci:expr) => {
    (*(*$ci).func).is_function() && $crate::macros::f_is_lua::f_isLua!($ci)
  };
}

pub use isLua;
