use crate::{records::type_searcher::TypeSearcher, type_aliases::type_id::TypeId};

impl TypeSearcher {
  pub fn visit_type_id(&mut self, ty: TypeId) -> bool {
    if ty == self.needle {
      self.count += 1;
      self.result |= self.current;
    }

    true
  }
}
