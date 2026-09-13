#[macro_export]
macro_rules! isLua {
  ($ci:expr) => {
    $crate::macros::ttisfunction::ttisfunction!((*$ci).func)
      && $crate::macros::f_is_lua::f_isLua!($ci)
  };
}

pub use isLua;
