use crate::{records::type_ids::TypeIds, type_aliases::type_id::TypeId};

impl TypeIds {
  pub fn erase_type_id(&mut self, ty: TypeId) {
    // 与 cpp `Set::erase(key)` 一致：定位首个匹配 → 标记 types 槽位 false →
    // 撤销 hash 贡献 → 从 order 移除。单次扫描即可完成；不再经由
    // `begin()` 克隆整个 order 快照再逐 next 推进、最后在
    // `erase_type_ids_const_iterator` 里二次定位（省一次堆分配与两轮遍历）。
    let Some(pos) = self.order.iter().position(|x| x == &ty) else {
      return; // 不存在则无操作，与 C++ erase 不命中时行为一致
    };
    self.order.remove(pos);
    if let Some(entry) = self.types.find_mut(&ty) {
      *entry = false;
    }
    self.hash ^= ty as usize;
  }
}
