use crate::records::type_ids::TypeIds;

impl TypeIds {
  pub fn retain(&mut self, tys: &TypeIds) {
    // 单次原地压缩：与「逐元素 erase_type_id」的可观察副作用一致
    // （order 保序、types 槽位置 false、hash 撤销贡献），复杂度 O(n²) → O(n)。
    let Self { order, types, hash } = self;
    order.retain(|&ty| {
      let keep = tys.count(ty) > 0;
      if !keep {
        if let Some(entry) = types.find_mut(&ty) {
          *entry = false;
        }
        *hash ^= ty as usize;
      }
      keep
    });
  }
}
