use crate::functions::is_supported::is_supported;

pub fn luau_codegen_supported() -> i32 {
  if is_supported() { 1 } else { 0 }
}
