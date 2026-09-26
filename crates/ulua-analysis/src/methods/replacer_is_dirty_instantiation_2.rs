use crate::{
  records::replacer::Replacer,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Replacer {
  pub fn is_dirty_type_id(&self, ty: TypeId) -> bool {
    // Safety: `replacements` 由 `Replacer::new` 原样保存调用方传入的映射表指针，其源头一律
    // 是独占借用的地址（`&mut replacements as *mut DenseHashMap<..>` 或
    // `NonNull::from(&mut self.generic_substitutions).as_ptr()`），故非空、对齐且比本
    // Replacer 长寿。`find` 只按 `TypeId` 的指针值做只读哈希查找（不解引用 `ty`），
    // 与 `&self` 的共享借用一致，期间无人对该 map 做可变借用。
    unsafe { (*self.replacements).find(&ty).is_some() }
  }

  pub fn is_dirty_type_pack_id(&self, tp: TypePackId) -> bool {
    // Safety: 同上——`replacement_packs` 与 `replacements` 由同一构造调用接线，非空且活到
    // Replacer 结束之后；这里仅 `find` 只读查键（`TypePackId` 的指针值），不写 map。
    unsafe { (*self.replacement_packs).find(&tp).is_some() }
  }
}
