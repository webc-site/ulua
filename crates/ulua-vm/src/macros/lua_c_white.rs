#[macro_export]
macro_rules! luaC_white {
  ($g:expr) => {
    $crate::macros::cast_to::cast_to!(
      u8,
      ((*$g).currentwhite as i32) & $crate::macros::whitebits::WHITEBITS
    )
  };
}

pub use luaC_white;
