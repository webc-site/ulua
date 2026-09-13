use crate::functions::traverse_directory_rec_file_utils_alt_b::traverse_directory_rec_string_function_void_const_string_name;

pub fn traverse_directory_mut(path: &str, callback: &dyn Fn(&str)) -> bool {
  // 两平台走 byte 递归 helper
  traverse_directory_rec_string_function_void_const_string_name(path, callback)
}
