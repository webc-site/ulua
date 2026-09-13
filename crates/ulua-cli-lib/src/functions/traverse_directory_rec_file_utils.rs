use alloc::string::String;

use crate::functions::{
  join_paths_file_utils::join_paths_basic_string_ch_ch_ch,
  join_paths_file_utils_alt_b::join_paths_string_view_string_view, to_utf_8::to_utf_8,
  traverse_directory_rec_file_utils_alt_b::traverse_directory_rec_string_function_void_const_string_name,
};

pub fn traverse_directory_rec_wstring_function_void_const_string_name(
  path: &[u16],
  callback: &dyn Fn(&str),
) -> bool {
  // Native-only implementation depends on Win32 APIs (FindFirstFileW/FindNextFileW/FindClose).
  // This crate targets wasm32-unknown-unknown, so keep this stubbed.
  let _ = (
    path,
    callback,
    join_paths_basic_string_ch_ch_ch as fn(&mut String, &str, &str),
    join_paths_string_view_string_view as fn(&str, &str) -> String,
    to_utf_8 as fn(&[u16]) -> String,
    traverse_directory_rec_string_function_void_const_string_name
      as fn(&str, &dyn Fn(&str)) -> bool,
  );
  false
}
