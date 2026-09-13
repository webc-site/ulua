use alloc::string::String;
use std::fs::read;

pub fn read_file(name: &str) -> Option<String> {
  // Use the standard library's filesystem layer rather than hand-rolled C
  // `fopen`/`_wfopen` FFI. `std::fs` handles cross-platform path encoding
  // (including UTF-16 wide paths with correct NUL termination on Windows) and
  // is free of the unsafe pointer plumbing the previous port carried. Files
  // are read as raw bytes, mirroring the C++ reader's binary (`"rb"`) mode.
  //
  // Several callers hand us a C-string-style path that still carries its
  // trailing NUL (e.g. `to_bytes_with_nul()` / `nul_terminated()`), relying on
  // the old `fopen` stopping at the first NUL. `std::fs` treats an interior
  // NUL as an error, so reproduce that termination behaviour here.
  let name = match name.find('\0') {
    Some(end) => &name[..end],
    None => name,
  };
  let bytes = read(name).ok()?;

  // SAFETY: Luau source may contain arbitrary bytes inside string literals;
  // the C++ reader stores them verbatim in a `std::string` and hands the raw
  // Buffer to the compiler. We mirror that — no UTF-8 validation/rewriting.
  let mut result = unsafe { String::from_utf8_unchecked(bytes) };

  // Skip first line if it's a shebang
  if result.len() > 2
    && result.as_bytes()[0] == b'#'
    && result.as_bytes()[1] == b'!'
    && let Some(newline_pos) = result.find('\n')
  {
    result.drain(0..=newline_pos);
  }

  Some(result)
}
