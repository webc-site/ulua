//! `type_ids` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::IntoIter;
use std::mem;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  functions::{follow_type, get_type},
  records::{never_type::NeverType, type_ids::TypeIds},
  type_aliases::type_id::TypeId,
};

impl TypeIds {
  pub fn begin(&self) -> IntoIter<TypeId> {
    self.order.clone().into_iter()
  }
}

impl TypeIds {
  pub fn clear(&mut self) {
    self.order.clear();
    self.types.clear();
    self.hash = 0;
  }
}

impl TypeIds {
  pub fn clear_without_realloc(&mut self) {
    // Clear the logical contents without forcing underlying allocations to be released.
    // - DenseHashMap: clear() preserves capacity; no threshold parameter needed.
    // - order: clear the vector (capacity is preserved).
    // - hash: reset.
    self.order.clear();
    self.types.clear();
    self.hash = 0;
  }
}

impl TypeIds {
  /// C++ `size_t count(TypeId ty) const`.
  pub fn count(&self, ty: TypeId) -> usize {
    let ty = follow_type::follow(ty);
    match self.types.find(&ty) {
      Some(entry) if *entry => 1,
      _ => 0,
    }
  }
}

impl TypeIds {
  pub fn empty(&self) -> bool {
    self.order.is_empty()
  }
}

impl TypeIds {
  pub fn end(&self) -> IntoIter<TypeId> {
    self.order.clone().into_iter()
  }
}

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

impl TypeIds {
  pub fn front(&self) -> TypeId {
    self.order[0]
  }
}

impl TypeIds {
  pub fn insert_type_id(&mut self, ty: TypeId) {
    let ty = follow_type::follow(ty);

    // get a reference to the slot for `ty` in `types`
    let entry = self.types.get_or_insert(ty);

    // if `ty` is fresh, we can set it to `true`, add it to the order and hash and be done.
    if !*entry {
      *entry = true;
      self.order.push(ty);
      self.hash ^= ty as usize;
    }
  }
}

impl TypeIds {
  pub fn is_never(&self) -> bool {
    // std::all_of(begin(), end(), [](TypeId i) { return get<NeverType>(i) != nullptr; })
    self.order.iter().all(|&i| {
      // If each typeid is never, then I guess typeid's is also never?
      get_type::get::<NeverType>(i).is_some()
    })
  }
}

impl TypeIds {
  pub fn reserve(&mut self, n: usize) {
    self.order.reserve(n);
  }
}

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

impl TypeIds {
  pub fn size(&self) -> usize {
    self.order.len()
  }
}

impl TypeIds {
  pub fn take(&mut self) -> Vec<TypeId> {
    self.hash = 0;
    self.types.clear();
    mem::take(&mut self.order)
  }
}

impl TypeIds {
  pub fn new() -> Self {
    Self {
      types: DenseHashMap::default(),
      order: Vec::new(),
      hash: 0,
    }
  }
}
impl Default for TypeIds {
  fn default() -> Self {
    Self::new()
  }
}
impl TypeIds {
  pub fn type_ids_initializer_list_type_id(&mut self, tys: &[TypeId]) {
    for ty in tys {
      self.insert_type_id(*ty);
    }
  }
}
