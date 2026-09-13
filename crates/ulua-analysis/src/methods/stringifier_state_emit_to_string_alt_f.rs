use crate::{enums::polarity::Polarity, records::stringifier_state::StringifierState};

impl StringifierState {
  pub fn emit_polarity(&mut self, p: Polarity) {
    let s = match p {
      Polarity::None => "  ",
      Polarity::Negative => " -",
      Polarity::Positive => "+ ",
      Polarity::Mixed => "+-",
      _ => "!!",
    };
    self.emit_string(s);
  }
}
