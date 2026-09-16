pub const WHITE0BIT: i32 = 0;
pub const WHITE1BIT: i32 = 1;

#[macro_export]
macro_rules! stringmark {
  ($s:expr) => {
    $crate::macros::reset_2_bits::reset2bits!(
      // tstring embeds CommonHeader as `hdr`; C++ reads ts->marked directly
      (*$s).hdr.marked,
      $crate::macros::stringmark::WHITE0BIT,
      $crate::macros::stringmark::WHITE1BIT
    )
  };
}

pub use stringmark;
