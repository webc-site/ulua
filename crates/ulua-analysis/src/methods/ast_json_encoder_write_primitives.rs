//! Source: `Analysis/src/AstJsonEncoder.cpp` (lines 89-210, hand-ported)
//! The C++ overloaded `write(...)` family, expressed as the Rust shape of
//! overloading: a `WriteJson` trait + a generic `write(prop_name, value)`.
//! PROP!(emitter, node, prop) expands to `emitter.write("prop", &node.prop)`.

use alloc::{format, string::String};

use ulua_ast::records::ast_name::AstName;

use crate::records::ast_json_encoder::AstJsonEncoder;

pub trait WriteJson {
  fn write_json(&self, enc: &mut AstJsonEncoder);
}

impl AstJsonEncoder {
  /// C++ template write(std::string_view prop_name, const T& value)
  pub fn write<T: WriteJson + ?Sized>(&mut self, prop_name: &str, value: &T) {
    if self.comma {
      self.write_raw_string_view(",");
    }
    self.comma = true;
    self.write_raw_string_view("\"");
    self.write_raw_string_view(prop_name);
    self.write_raw_string_view("\":");
    value.write_json(self);
  }
}

impl WriteJson for bool {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    enc.write_raw_string_view(if *self { "true" } else { "false" });
  }
}

impl WriteJson for f64 {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    enc.write_f64(*self);
  }
}

macro_rules! write_json_int {
    ($($t:ty),*) => {$(
        impl WriteJson for $t {
            fn write_json(&self, enc: &mut AstJsonEncoder) {
                let s = format!("{}", self);
                enc.write_raw_string_view(&s);
            }
        }
    )*};
}
// NB: no i8/u8 here -- C++ `char` writes as a one-char STRING (write(char),
// AstJsonEncoder.cpp:156) and AstArray<char> as a string; c_char resolves to
// i8 or u8 per target, so neither may be a JSON integer.
write_json_int!(i32, i64, u32, u64, usize, isize, u16, i16);

impl WriteJson for str {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    enc.write_string(self);
  }
}

impl WriteJson for String {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    enc.write_string(self);
  }
}

impl WriteJson for AstName {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    enc.write_string(self.as_str_or_empty());
  }
}

impl<T: WriteJson> WriteJson for Option<T> {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    match self {
      Some(v) => v.write_json(enc),
      None => enc.write_raw_string_view("null"),
    }
  }
}
