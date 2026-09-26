use ulua_common::LUAU_ASSERT;

use crate::{records::type_cloner::TypeCloner, type_aliases::type_or_pack::TypeOrPack};

impl TypeCloner {
  pub fn run(&mut self) {
    while !self.queue.is_empty() {
      self.steps += 1;

      if self.has_exceeded_iteration_limit() {
        break;
      }

      // while 头 `!queue.is_empty()` 判定蕴含 pop 命中 Some。
      let kind: TypeOrPack = self
        .queue
        .pop()
        .expect("while 头 !queue.is_empty() 蕴含非空");

      LUAU_ASSERT!(!self.find_type_or_pack(kind.clone()).is_some());

      self.clone_children_type_or_pack(kind);
    }
  }
}
