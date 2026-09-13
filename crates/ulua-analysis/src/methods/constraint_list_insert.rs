use crate::{
  records::constraint_list::ConstraintList, type_aliases::constraint_vertex::ConstraintVertex,
};

impl ConstraintList {
  pub fn insert(&mut self, vertex: ConstraintVertex) {
    let (entry, fresh) = self.present.try_insert(vertex.clone(), true);
    if fresh {
      self.order.push(vertex);
      self.entries += 1;
    } else if !*entry {
      *entry = true;
      self.entries += 1;
    }
    // If the entry was *not* fresh and its value was already true, then do
    // nothing: the set state has not changed.
  }
}
