use alloc::string::String;

pub fn format(a: &str, b: &str, expected: usize, actual: usize) -> String {
  alloc::format!(
    "Distance of '{}' and '{}' : expected {}, got {}",
    a,
    b,
    expected,
    actual
  )
}
