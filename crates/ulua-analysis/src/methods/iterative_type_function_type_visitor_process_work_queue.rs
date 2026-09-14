use crate::records::iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor;

impl IterativeTypeFunctionTypeVisitor {
  pub fn process_work_queue(&mut self) {
    while self.work_cursor < self.work_queue.len() as u32 {
      let item = &self.work_queue[self.work_cursor as usize];
      self.parent_cursor = self.work_cursor as i32;

      if item.is_type {
        let ty = unsafe { *item.as_type() };
        if self.is_cyclic(ty) {
          self.cycle_type_function_type_id(ty);
        } else {
          self.process_type_function_type_id(ty);
        }
      } else {
        let tp = unsafe { *item.as_type_pack() };
        if self.is_cyclic(tp) {
          self.cycle_type_function_type_pack_id(tp);
        } else {
          self.process_type_function_type_pack_id(tp);
        }
      }

      self.work_cursor += 1;
    }
  }
}
