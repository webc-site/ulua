use crate::records::iterative_type_visitor::IterativeTypeVisitor;

impl IterativeTypeVisitor {
  pub fn iterative_type_visitor_process_work_queue(&mut self) {
    while (self.work_cursor as usize) < self.work_queue.len() {
      let item = &self.work_queue[self.work_cursor as usize];
      self.parent_cursor = self.work_cursor as i32;

      if !item.is_type {
        let tp_ptr = item.as_type_pack();
        let tp = unsafe { *tp_ptr };
        if self.iterative_type_visitor_is_cyclic() {
          self.cycle_type_pack_id(tp);
        } else {
          self.process_type_pack_id(tp);
        }
      } else {
        let ty_ptr = item.as_type();
        let ty = unsafe { *ty_ptr };
        if self.iterative_type_visitor_is_cyclic() {
          self.cycle_type_id(ty);
        } else {
          self.process_type_id(ty);
        }
      }

      self.work_cursor += 1;
    }
  }
}
