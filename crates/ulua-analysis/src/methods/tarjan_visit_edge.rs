use crate::records::tarjan::Tarjan;

impl Tarjan {
  pub fn visit_edge(&mut self, index: i32, parent_index: i32) {
    let is_dirty = Tarjan::get_dirty(self, index);
    if is_dirty {
      Tarjan::set_dirty(self, parent_index, true);
    }
  }
}
