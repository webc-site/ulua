use crate::enums::polarity::Polarity;

pub fn is_positive(p: Polarity) -> bool {
  (p as u8 & Polarity::Positive as u8) != 0
}
