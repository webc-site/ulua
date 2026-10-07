#[macro_export]
macro_rules! stringmark {
  ($s:expr) => {
    $crate::macros::reset_2_bits::reset2bits!(
      // tstring embeds CommonHeader as `hdr`; C++ reads ts->marked directly
      (*$s).hdr.marked,
      $crate::macros::whitebits::WHITE0BIT,
      $crate::macros::whitebits::WHITE1BIT
    )
  };
}

pub use stringmark;
