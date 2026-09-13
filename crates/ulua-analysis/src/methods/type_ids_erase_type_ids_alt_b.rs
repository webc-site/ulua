use crate::{records::type_ids::TypeIds, type_aliases::type_id::TypeId};

impl TypeIds {
  pub fn erase_type_id(&mut self, ty: TypeId) {
    // 快照迭代器定位到 ty 所在位置后，erase 按其指向的值（即 ty）擦除，
    // 与 cpp `Set::erase(key)` 行为一致。
    let it = self.order.iter().position(|x| x == &ty).map(|pos| {
      let mut iter = self.begin();
      for _ in 0..pos {
        iter.next();
      }
      iter
    });
    if let Some(it) = it {
      let _ = self.erase_type_ids_const_iterator(it);
    }
  }
}
