use crate::enums::polarity::Polarity;

pub fn invert(p: Polarity) -> Polarity {
  match p {
    Polarity::Positive => Polarity::Negative,
    Polarity::Negative => Polarity::Positive,
    _ => p,
  }
}

pub use invert as invert_polarity;
