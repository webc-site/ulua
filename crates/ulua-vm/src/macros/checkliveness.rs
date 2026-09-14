#[macro_export]
macro_rules! checkliveness {
  ($g:expr, $obj:expr) => {
    ulua_common::LUAU_ASSERT!(
      !$crate::iscollectable!($obj)
        || (($crate::ttype!($obj) == (*(*$obj).value.gc).gch.tt as core::ffi::c_int)
          && !$crate::isdead!($g, (*$obj).value.gc))
    )
  };
}

pub use checkliveness;
