//! `widen` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::string::String;
use std::panic::panic_any;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::get_type,
  macros::substitution_vtable,
  records::{
    arena_handle::Handle, boolean_singleton::BooleanSingleton, builtin_types::BuiltinTypes,
    extern_type::ExternType, internal_compiler_error::InternalCompilerError,
    singleton_type::SingletonType, string_singleton::StringSingleton, substitution::Substitution,
    txn_log::TxnLog, type_arena::TypeArena, union_type::UnionType, widen::Widen,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Widen {
  pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
    LUAU_ASSERT!(self.is_dirty_type_id(ty));

    let stv = get_type::get::<SingletonType>(ty);
    LUAU_ASSERT!(stv.is_some());

    // 紧邻 LUAU_ASSERT(stv.is_some()) 蕴含 Some。
    let stv_ref = stv.expect("紧邻 LUAU_ASSERT(stv.is_some()) 蕴含");

    if stv_ref.variant.get_if::<StringSingleton>().is_some() {
      self.builtin_types.get().string_type
    } else {
      // If this assert trips, it's likely we now have number singletons.
      LUAU_ASSERT!(stv_ref.variant.get_if::<BooleanSingleton>().is_some());
      self.builtin_types.get().boolean_type
    }
  }

  pub fn clean_type_pack_id(&mut self, _tp: TypePackId) -> TypePackId {
    // C++ `Unifier.cpp:287`：`throw InternalCompilerError(...)`。载荷携带类型而非
    // 渲染串，消息文本由 ICE 的 `Display` 单源产生，与原 `panic!("{}", ..)` 逐字一致。
    panic_any(InternalCompilerError::new(
      String::from("Widen attempted to clean a dirty type pack?"),
      None,
      None,
    ));
  }
}

impl Widen {
  pub fn widen_ignore_children(&self, ty: TypeId) -> bool {
    let et = get_type::get::<ExternType>(ty);
    if et.is_some() {
      return true;
    }

    let ut = get_type::get::<UnionType>(ty);
    ut.is_none()
  }
}

impl Widen {
  pub fn is_dirty_type_id(&mut self, ty: TypeId) -> bool {
    unsafe { (*self.base.base.log).txn_log_is::<SingletonType, _>(ty) }
  }

  pub fn is_dirty_type_pack_id(&mut self, _tp: TypePackId) -> bool {
    false
  }
}

// C++ 未覆写 `ignoreChildren(TypePackId)`，保持基类默认 false（pack 侧三槽中
// isDirty/clean 仍为真实转发，见 substitution_vtable 模块文档的统一安全论证）。
substitution_vtable!(false_tp, pub(crate) Widen, ic = widen_ignore_children);
impl Widen {
  pub fn widen_widen(arena: Handle<TypeArena>, builtin_types: Handle<BuiltinTypes>) -> Self {
    Widen {
      base: Substitution::substitution_new(TxnLog::empty(), Some(arena)),
      builtin_types,
    }
  }

  pub fn widen_type(&mut self, ty: TypeId) -> TypeId {
    self.install_substitution_vtable();
    self.base.substitute_type_id(ty).unwrap_or(ty)
  }

  pub fn widen_type_pack(&mut self, tp: TypePackId) -> TypePackId {
    self.install_substitution_vtable();
    self.base.substitute_type_pack_id(tp).unwrap_or(tp)
  }
}
