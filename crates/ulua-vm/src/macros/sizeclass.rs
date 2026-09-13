#[macro_export]
macro_rules! sizeclass {
  ($sz:expr) => {{
    // (size_t((sz) - 1) < K_MAX_SMALL_SIZEUsed ? K_SIZE_CLASS_CONFIG.class_for_size[sz] : -1)
    let __sz = $sz;
    let __idx = (__sz as usize).wrapping_sub(1);
    if __idx < $crate::records::size_class_config::K_MAX_SMALL_SIZE as usize {
      $crate::records::size_class_config::K_SIZE_CLASS_CONFIG.class_for_size[__sz as usize]
    } else {
      -1_i8
    }
  }};
}

pub use sizeclass;
