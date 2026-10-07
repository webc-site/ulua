#[macro_export]
macro_rules! luaC_white {
  ($g:expr) => {
    (((*$g).currentwhite as i32) & $crate::macros::whitebits::WHITEBITS) as u8
  };
}

pub use luaC_white;
