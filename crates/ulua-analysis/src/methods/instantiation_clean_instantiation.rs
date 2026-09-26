use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::as_mutable_type::as_mutable_type_id,
  records::{function_type::FunctionType, instantiation::Instantiation},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Instantiation {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  pub unsafe fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
    // Safety: self.base.base.log 是 Substitution 构造期（Replacer::new 等）接线的
    // TxnLog 指针——要么指向进程级单例 TxnLog::empty()，要么指向驱动本次实例化的
    // 模块 log，两者均非空且比 self 长寿；重建只读借用调 &self 安全方法，无别名。
    let ftv = unsafe { (*self.base.base.log).txn_log_get_mutable::<FunctionType, TypeId>(ty) };
    LUAU_ASSERT!(!ftv.is_null());
    // Safety: 调用链保证此处的 ty 刚被按 FunctionType 变体匹配分发（Tarjan 遍历
    // 快照即来自该变体的 pending），txn_log_get_mutable 的 RTTI 命中分支必非空，
    // LUAU_ASSERT 显式编码该不变量；返回指针指向 log 拥有的 Box 节点，地址稳定。
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
      self.base.wired_arena_handle(),
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

    // Safety: as_mutable_type_id(result) 是 TypeId(*mut Type) 的恒等转换，result 刚由
    // add_type 写入 types arena——arena bump 块地址不移动，该指针必有效；ty 是同
    // arena 中存活的输入类型节点。两对象不同（result 为新建），写 result 字段与
    // 读 ty 不冲突，借用均止于本语句。
    unsafe {
      (*as_mutable_type_id(result)).documentation_symbol = (*ty).documentation_symbol.clone();
    }

    result
  }

  pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    LUAU_ASSERT!(false);
    tp
  }
}
