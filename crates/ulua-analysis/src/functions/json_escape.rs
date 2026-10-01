extern crate alloc;

use alloc::string::String;
use core::fmt::Write;

pub(crate) fn json_escape(s: &str) -> String {
  let mut out = String::with_capacity(s.len() + 2);
  for c in s.chars() {
    match c {
      '"' => out.push_str("\\\""),
      '\\' => out.push_str("\\\\"),
      '\x08' => out.push_str("\\b"),
      '\x0c' => out.push_str("\\f"),
      '\n' => out.push_str("\\n"),
      '\r' => out.push_str("\\r"),
      '\t' => out.push_str("\\t"),
      _ => {
        if (c as u32) < 0x20 {
          write!(out, "\\u{:04x}", c as u32).expect("String write failed");
        } else {
          out.push(c);
        }
      }
    }
  }
  out
}
