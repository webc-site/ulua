use crate::{
  functions::invert_polarity::invert_polarity, records::free_type_searcher::FreeTypeSearcher,
};

impl FreeTypeSearcher {
  pub fn flip(&mut self) {
    self.polarity = invert_polarity(self.polarity);
  }
}
