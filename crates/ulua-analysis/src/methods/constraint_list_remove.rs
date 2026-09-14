use crate::{
  records::constraint_list::ConstraintList, type_aliases::constraint_vertex::ConstraintVertex,
};

impl ConstraintList {
  pub fn remove(&mut self, vertex: ConstraintVertex) {
    if let Some(entry) = self.present.find_mut(&vertex) {
      // If the entry is true then we also need to decrement the number of
      // entries in the constraint list.
      if *entry {
        self.entries -= 1;
      }
      *entry = false;
    }
  }
}
