use alloc::string::String;

pub fn get_extension(path: &str) -> String {
  match path.rfind(['.', '\\', '/']) {
    Some(dot_index) => {
      if path.as_bytes()[dot_index] == b'.' {
        String::from(&path[dot_index..])
      } else {
        String::new()
      }
    }
    None => String::new(),
  }
}
