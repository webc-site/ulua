use alloc::string::String;
use core::ffi::c_char;

pub fn escape_filename(filename: &str) -> String {
  let mut escaped = String::with_capacity(filename.len());

  for ch in filename.bytes() {
    let c = ch as c_char;
    match c {
      v if v == b'\\' as c_char => escaped.push('/'),
      v if v == b'"' as c_char => {
        escaped.push('\\');
        escaped.push('"');
      }
      _ => escaped.push(ch as char),
    }
  }

  escaped
}
