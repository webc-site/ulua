//! `apply_type_function` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::{
  functions::{get_type, get_type_pack},
  macros::{substitution_entry::substitution_entry, substitution_vtable},
  records::{
    apply_type_function::ApplyTypeFunction, arena_handle::Handle, extern_type::ExternType,
    free_type::FreeType, generic_type::GenericType, generic_type_pack::GenericTypePack,
    substitution::Substitution, txn_log::TxnLog, type_arena::TypeArena,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

// TypePackId 侧三覆写槽全为真实转发（见 substitution_vtable 模块文档的统一安全论证）。
substitution_vtable!(real, ApplyTypeFunction, ic = ignore_children_type_id);
impl ApplyTypeFunction {
  pub fn new(arena: Handle<TypeArena>) -> Self {
    Self {
      base: Substitution::substitution_new(TxnLog::empty(), Some(arena)),
      encountered_forwarded_type: false,
      type_arguments: DenseHashMap::default(),
      type_pack_arguments: DenseHashMap::default(),
    }
  }

  substitution_entry!(id, pack);
}

impl ApplyTypeFunction {
  pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
    // cpp `find` 未命中返回 null 由紧随 LUAU_ASSERT 拦截；Rust 以 None 表意
    // 同一未命中，expect 确定化呈现。
    let arg = self
      .type_arguments
      .find(&ty)
      .expect("type_arguments 未命中即 cpp 断言拦截的同位置 null");
    LUAU_ASSERT!(!arg.is_null());
    *arg
  }

  pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    let arg = self
      .type_pack_arguments
      .find(&tp)
      .expect("TypePackId not found in type_pack_arguments");
    LUAU_ASSERT!(!arg.is_null());
    *arg
  }
}

impl ApplyTypeFunction {
  pub fn ignore_children_type_id(&mut self, ty: TypeId) -> bool {
    if get_type::get::<GenericType>(ty).is_some() {
      true
    } else {
      get_type::get::<ExternType>(ty).is_some()
    }
  }

  pub fn ignore_children_type_pack_id(&mut self, tp: TypePackId) -> bool {
    let gt = get_type_pack::get::<GenericTypePack>(tp);
    gt.is_some()
  }
}

impl ApplyTypeFunction {
  pub fn is_dirty_type_id(&mut self, ty: TypeId) -> bool {
    if self.type_arguments.find(&ty).is_some() {
      true
    } else if let Some(ftv) = get_type::get::<FreeType>(ty) {
      if ftv.forwarded_type_alias {
        self.encountered_forwarded_type = true;
      }
      false
    } else {
      false
    }
  }

  pub fn is_dirty_type_pack_id(&mut self, tp: TypePackId) -> bool {
    self.type_pack_arguments.find(&tp).is_some()
  }
}
