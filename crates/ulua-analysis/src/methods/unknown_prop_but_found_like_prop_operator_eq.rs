use crate::records::unknown_prop_but_found_like_prop::UnknownPropButFoundLikeProp;

impl UnknownPropButFoundLikeProp {
  #[inline]
  pub fn operator_eq(&self, rhs: &UnknownPropButFoundLikeProp) -> bool {
    self.table == rhs.table
      && self.key == rhs.key
      && self.candidates.len() == rhs.candidates.len()
      && self.candidates == rhs.candidates
  }
}
