use alloc::{string::String, vec::Vec};
use core::ffi::{CStr, c_char};
use std::cell::RefCell;

use crate::functions::{
  get_extension::get_extension, is_directory::is_directory, normalize_path::normalize_path,
  traverse_directory_file_utils::traverse_directory_mut,
};

pub fn get_source_files_from_slice(args: &[impl AsRef<str>]) -> Vec<String> {
  let mut files = Vec::new();

  for arg in args.iter().skip(1) {
    let arg = arg.as_ref();

    if arg == "--program-args" || arg == "-a" {
      return files;
    }

    // Treat '-' as a special file whose source is read from stdin
    // All other arguments that start with '-' are skipped
    if arg.starts_with('-') && arg.len() > 1 {
      continue;
    }

    let normalized = normalize_path(arg);

    if is_directory(&normalized) {
      let files_ref = RefCell::new(&mut files);

      traverse_directory_mut(&normalized, &|name: &str| {
        let ext = get_extension(name);
        if ext == ".lua" || ext == ".luau" {
          files_ref.borrow_mut().push(String::from(name));
        }
      });
    } else {
      files.push(normalized);
    }
  }

  files
}

/// # Safety
///
/// `argv` must be valid for reading `argc` pointers to null-terminated C strings.
pub unsafe fn get_source_files(argc: i32, argv: *mut *mut c_char) -> Vec<String> {
  let mut args = Vec::new();
  for i in 0..argc as usize {
    let arg_ptr = unsafe { *argv.add(i) };
    if arg_ptr.is_null() {
      args.push(String::new());
    } else {
      let s = unsafe { CStr::from_ptr(arg_ptr).to_string_lossy().into_owned() };
      args.push(s);
    }
  }

  get_source_files_from_slice(&args)
}
