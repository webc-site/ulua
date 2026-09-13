use alloc::string::String;

pub fn rep(s: &str, n: usize) -> String {
  let mut r = String::with_capacity(s.len() * n);
  for _ in 0..n {
    r.push_str(s);
  }
  r
}
