macro_rules! WRITE_NAME {
  ($fp:expr, $indent:expr, $name:ident) => {
    libc::fprintf(
      $fp,
      concat!($indent, "\"", stringify!($name), "\": ").as_ptr() as *const core::ffi::c_char,
    )
  };
}

pub(crate) use WRITE_NAME;
