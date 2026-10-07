//! `replacer` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::{
  functions::{follow_type, follow_type_pack, get_type},
  macros::{substitution_entry::substitution_entry, substitution_vtable},
  records::{
    arena_handle::{Handle, alias_ref},
    extern_type::ExternType,
    function_type::FunctionType,
    replacer::Replacer,
    substitution::Substitution,
    txn_log::TxnLog,
    type_arena::TypeArena,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Replacer {
  fn check_replacement_keys(&self) -> bool {
    let replacements = alias_ref(self.replacements);
    for (k, _) in replacements.iter() {
      let followed = follow_type::follow(*k);
      if *k != followed {
        return false;
      }
    }

    let replacement_packs = alias_ref(self.replacement_packs);
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
    let res = alias_ref(self.replacements)
      .find(&ty)
      .expect("TypeId not found in replacements");
    LUAU_ASSERT!(!res.is_null());
    let cleaned = *res;
    self.base.dont_traverse_into_type_id(cleaned);
    cleaned
  }

  pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    let res = alias_ref(self.replacement_packs)
      .find(&tp)
      .expect("TypePackId not found in replacement_packs");
    LUAU_ASSERT!(!res.is_null());
    let cleaned = *res;
    self.base.dont_traverse_into_type_pack_id(cleaned);
    cleaned
  }
}

impl Replacer {
  pub fn ignore_children(&self, ty: TypeId) -> bool {
    if get_type::get::<ExternType>(ty).is_some() {
      return true;
    }

    if let Some(ftv) = get_type::get::<FunctionType>(ty).as_ref() {
      if ftv.has_no_free_or_generic_types {
        return false;
      }

      for &generic in &ftv.generics {
        if alias_ref(self.replacements).find(&generic).is_some() {
          return true;
        }
      }

      for &generic in &ftv.generic_packs {
        if alias_ref(self.replacement_packs).find(&generic).is_some() {
          return true;
        }
      }
    }

    false
  }
}

impl Replacer {
  pub fn is_dirty_type_id(&self, ty: TypeId) -> bool {
    alias_ref(self.replacements).find(&ty).is_some()
  }

  pub fn is_dirty_type_pack_id(&self, tp: TypePackId) -> bool {
    alias_ref(self.replacement_packs).find(&tp).is_some()
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
