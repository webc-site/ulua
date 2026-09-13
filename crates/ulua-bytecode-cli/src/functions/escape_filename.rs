use alloc::string::String;

pub(crate) fn escape_filename(filename: &str) -> String {
  let mut escaped = String::with_capacity(filename.len());

  for ch in filename.chars() {
    match ch {
      '\\' => {
        escaped.push('/');
      }
      '"' => {
        escaped.push('\\');
        escaped.push(ch);
      }
      _ => {
        escaped.push(ch);
      }
    }
  }

  escaped
}
