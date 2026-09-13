use core::ffi::{CStr, c_char};

use ulua_ast::records::location::Location;

pub fn report(name: *const c_char, location: &Location, r#type: &str, message: &str) {
  let name = unsafe { CStr::from_ptr(name).to_string_lossy() };

  eprintln!(
    "{}({},{}): {}: {}",
    name,
    location.begin.line + 1,
    location.begin.column + 1,
    r#type,
    message
  );
}
