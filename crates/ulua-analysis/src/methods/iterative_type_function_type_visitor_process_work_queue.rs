use crate::records::iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor;

impl IterativeTypeFunctionTypeVisitor {
  pub fn process_work_queue(&mut self) {
    while self.work_cursor < self.work_queue.len() as u32 {
      let item = &self.work_queue[self.work_cursor as usize];
      self.parent_cursor = self.work_cursor as i32;

      // 经判别式 Option 取值，unsafe 解引用收敛到 WorkItem 内部的指针转换。
      if let Some(ty) = item.type_function_type_id() {
        if self.is_cyclic(ty) {
          self.cycle_type_function_type_id(ty);
        } else {
          self.process_type_function_type_id(ty);
        }
      } else if let Some(tp) = item.type_function_type_pack_id() {
        if self.is_cyclic(tp) {
          self.cycle_type_function_type_pack_id(tp);
        } else {
          self.process_type_function_type_pack_id(tp);
        }
      } else {
        ulua_common::LUAU_ASSERT!(false);
      }

      self.work_cursor += 1;
    }
  }
}
