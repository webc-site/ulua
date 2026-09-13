use std::{fs::read_dir, path::Path};

pub fn traverse_directory_rec_string_function_void_const_string_name(
  path: &str,
  callback: &dyn Fn(&str),
) -> bool {
  let path_obj = Path::new(path);
  if let Ok(entries) = read_dir(path_obj) {
    for entry in entries.flatten() {
      let file_type = entry.file_type();
      if let Ok(ft) = file_type {
        let file_name = entry.file_name();
        let name_str = file_name.to_string_lossy();
        if name_str == "." || name_str == ".." {
          continue;
        }

        let full_path = path_obj.join(&file_name);
        let path_str = full_path.to_string_lossy();

        if ft.is_dir() {
          traverse_directory_rec_string_function_void_const_string_name(&path_str, callback);
        } else if ft.is_file() {
          callback(&path_str);
        }
      }
    }
    true
  } else {
    false
  }
}
