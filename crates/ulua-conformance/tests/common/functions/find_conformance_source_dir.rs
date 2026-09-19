use core::ffi::{CStr, c_char, c_int};
use std::{
  fs::metadata,
  path::{Path, PathBuf},
};

use crate::common::macros::get_cwd::get_cwd;
pub fn find_conformance_source_dir() -> String {
  // Scripts vendored alongside the crate (standalone / published-repo layout).
  // Checked first and cwd-independent; falls through to the cwd-walk below when
  // building inside the original workspace (where luau/tests/conformance exists).
  let vendored = concat!(env!("CARGO_MANIFEST_DIR"), "/conformance");
  if Path::new(vendored).is_dir() {
    return vendored.to_owned();
  }

  let mut buf = [0 as c_char; 4096];
  unsafe {
    if get_cwd(buf.as_mut_ptr(), buf.len() as c_int).is_null() {
      return String::new();
    }
  }

  let cwd = unsafe { CStr::from_ptr(buf.as_ptr()).to_string_lossy().into_owned() };

  let mut dir = PathBuf::from(&cwd);

  for _ in 0..20 {
    let local_conformance = dir.join("luau/tests/conformance");
    if let Ok(meta) = metadata(&local_conformance)
      && meta.is_dir()
    {
      return local_conformance.to_string_lossy().into_owned();
    }

    if let Ok(meta) = metadata(dir.join("Client/content"))
      && meta.is_dir()
    {
      return dir
        .join("Client/Luau/tests/conformance")
        .to_string_lossy()
        .into_owned();
    }

    if !dir.pop() {
      break;
    }
  }

  String::new()
}
