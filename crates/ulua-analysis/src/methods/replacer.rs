//! `replacer` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::{
  functions::{follow_type, follow_type_pack, get_type},
  macros::{substitution_entry::substitution_entry, substitution_vtable},
  records::{
    arena_handle::Handle, extern_type::ExternType, function_type::FunctionType, replacer::Replacer,
    substitution::Substitution, txn_log::TxnLog, type_arena::TypeArena,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Replacer {
  pub fn check_replacement_keys(&self) -> bool {
    // Safety: replacements 由 Replacer::new 收下驱动本次 replace 的调用方本地
    // DenseHashMap 地址（如 instantiate 的 `&mut replacements` 实参），非空且
    // 严格比 self 长寿；此处只重建共享借用做只读遍历，方法持 &self，单线程
    // 串行下无在册可变别名。
    let replacements = unsafe { &*self.replacements };
    for (k, _) in replacements.iter() {
      let followed = follow_type::follow(*k);
      if *k != followed {
        return false;
      }
    }

    // Safety: 同上——replacement_packs 与 replacements 同为构造期接线的调用方
    // 本地映射地址，存活覆盖本方法；只读共享借用无别名冲突。
    let replacement_packs = unsafe { &*self.replacement_packs };
    for (k, _) in replacement_packs.iter() {
      let followed = follow_type_pack::follow(*k);
      if *k != followed {
        return false;
      }
    }

    true
  }
}

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

impl Replacer {
  pub fn ignore_children(&self, ty: TypeId) -> bool {
    unsafe {
      if get_type::get::<ExternType>(ty).is_some() {
        return true;
      }

      if let Some(ftv) = get_type::get::<FunctionType>(ty).as_ref() {
        if ftv.has_no_free_or_generic_types {
          return false;
        }

        for &generic in &ftv.generics {
          if (*self.replacements).find(&generic).is_some() {
            return true;
          }
        }

        for &generic in &ftv.generic_packs {
          if (*self.replacement_packs).find(&generic).is_some() {
            return true;
          }
        }
      }
    }

    false
  }
}

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

// C++ 未覆写 `ignoreChildren(TypePackId)`，保持基类默认 false（pack 侧三槽中
// isDirty/clean 仍为真实转发，见 substitution_vtable 模块文档的统一安全论证）。
substitution_vtable!(false_tp, Replacer, ic = ignore_children);
impl Replacer {
  pub fn new(
    arena: Handle<TypeArena>,
    replacements: *mut DenseHashMap<TypeId, TypeId>,
    replacement_packs: *mut DenseHashMap<TypePackId, TypePackId>,
  ) -> Self {
    let this = Replacer {
      base: Substitution::substitution_new(TxnLog::empty(), Some(arena)),
      replacements,
      replacement_packs,
    };
    LUAU_ASSERT!(this.check_replacement_keys());
    this
  }

  substitution_entry!(id, pack);
}
