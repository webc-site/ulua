use alloc::string::String;

pub fn has_file_extension(name: &str, extensions: &[String]) -> bool {
  for extension in extensions {
    let extension_len = extension.len();
    if name.len() >= extension_len
      && name.as_bytes()[name.len() - extension_len..] == *extension.as_bytes()
    {
      return true;
    }
  }
  false
}
