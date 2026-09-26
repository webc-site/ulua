use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::replacer::Replacer,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Replacer {
  pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
    // Safety: replacements 由 Replacer::new 接线为调用方（instantiate 系列）局部
    // DenseHashMap 的地址，比 replacer 长寿；解引用只重建共享借用查表，借用止于
    // expect 表达式，随后按值拷出 TypeId（arena 裸指针，值语义），单线程无别名。
    let res = unsafe { (*self.replacements).find(&ty) }.expect("TypeId not found in replacements");
    LUAU_ASSERT!(!res.is_null());
    let cleaned = *res;
    self.base.dont_traverse_into_type_id(cleaned);
    cleaned
  }

  pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    // Safety: 同 clean_type_id——replacement_packs 构造期接线指向调用方存活的
    // DenseHashMap，非空且对齐；只读查表后按值拷贝，借用与后续 self 可变调用
    // 不重叠。
    let res = unsafe { (*self.replacement_packs).find(&tp) }
      .expect("TypePackId not found in replacement_packs");
    LUAU_ASSERT!(!res.is_null());
    let cleaned = *res;
    self.base.dont_traverse_into_type_pack_id(cleaned);
    cleaned
  }
}
