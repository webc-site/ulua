use crate::enums::polarity::Polarity;

pub fn is_known(p: Polarity) -> bool {
  p != Polarity::Unknown
}
