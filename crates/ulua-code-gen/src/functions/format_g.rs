extern crate alloc;

use alloc::{format, string::String};

pub(crate) fn format_g(v: f64, precision: i32) -> String {
  if v.is_nan() {
    return String::from("nan");
  }

  if v.is_infinite() {
    return if v.is_sign_negative() {
      String::from("-inf")
    } else {
      String::from("inf")
    };
  }

  if v == 0.0 {
    return if v.is_sign_negative() {
      String::from("-0")
    } else {
      String::from("0")
    };
  }

  let sci = format!("{:.*e}", (precision - 1).max(0) as usize, v);
  let exp: i32 = sci[sci.find('e').map(|i| i + 1).unwrap_or(sci.len())..]
    .parse()
    .unwrap_or(0);

  if exp >= -4 && exp < precision {
    let frac = (precision - 1 - exp).max(0) as usize;
    let mut s = format!("{:.*}", frac, v);

    if s.contains('.') {
      while s.ends_with('0') {
        s.pop();
      }
      if s.ends_with('.') {
        s.pop();
      }
    }

    s
  } else {
    let (mant, exp_part) = sci.split_once('e').unwrap_or((sci.as_str(), "0"));
    let mut m = String::from(mant);

    if m.contains('.') {
      while m.ends_with('0') {
        m.pop();
      }
      if m.ends_with('.') {
        m.pop();
      }
    }

    let e: i32 = exp_part.parse().unwrap_or(0);
    format!("{}e{}{:02}", m, if e < 0 { "-" } else { "+" }, e.abs())
  }
}
