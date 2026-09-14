use crate::{enums::polarity::Polarity, records::type_searcher::TypeSearcher};

impl TypeSearcher {
  pub fn flip(&mut self) {
    match self.current {
      Polarity::Positive => self.current = Polarity::Negative,
      Polarity::Negative => self.current = Polarity::Positive,
      _ => {}
    }
  }
}
