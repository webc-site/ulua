use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::as_mutable_type::as_mutable_type_id,
  records::{function_type::FunctionType, instantiation::Instantiation},
  type_aliases::type_id::TypeId,
};

impl Instantiation {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  pub unsafe fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
    let ftv = unsafe { (*self.base.base.log).txn_log_get_mutable::<FunctionType, TypeId>(ty) };
    LUAU_ASSERT!(!ftv.is_null());
    let ftv = unsafe { &*ftv };

    let mut clone = FunctionType::function_type_new(
      ftv.arg_types,
      ftv.ret_types,
      ftv.definition.clone(),
      ftv.has_self,
    );
    clone.level = self.level;
    clone.magic = ftv.magic.clone();
    clone.tags = ftv.tags.clone();
    clone.arg_names = ftv.arg_names.clone();
    clone.is_deprecated_function = ftv.is_deprecated_function;
    clone.deprecated_info = ftv.deprecated_info.clone();
    clone.is_checked_function = ftv.is_checked_function;

    let result = self.base.add_type(clone);

    self.reusable_replace_generics.reset_state(
      self.base.base.log,
      self.base.arena,
      self.builtin_types,
      self.level,
      self.scope,
      ftv.generics.clone(),
      ftv.generic_packs.clone(),
    );

    let result = self
      .reusable_replace_generics
      .substitute_type_id(result)
      .unwrap_or(result);

    unsafe {
      (*as_mutable_type_id(result)).documentation_symbol = (*ty).documentation_symbol.clone();
    }

    result
  }
}
