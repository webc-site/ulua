//! `instantiation` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;
use core::ptr::{from_ref, null_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{as_mutable_type::as_mutable_type_id, get_type},
  macros::{substitution_entry::substitution_entry, substitution_vtable},
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, extern_type::ExternType,
    function_type::FunctionType, instantiation::Instantiation, replace_generics::ReplaceGenerics,
    scope::Scope, substitution::Substitution, txn_log::TxnLog, type_arena::TypeArena,
    type_level::TypeLevel,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl Instantiation {
  pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
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
      self.scope_opt_ref(),
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

impl Instantiation {
  pub fn ignore_children(&self, ty: TypeId) -> bool {
    // 对齐 cpp Instantiation.cpp:49-55：FunctionType 读取改走 log 的 pending 态
    // （txn_log_get_mutable）；ExternType 在 cpp 中本就是 plain get，保持不变。
    let log = self.base.base.log;
    let ft = unsafe { (*log).txn_log_get_mutable::<FunctionType, TypeId>(ty) };
    if !ft.is_null() {
      return true;
    }

    get_type::get::<ExternType>(ty).is_some()
  }
}

// Source: `Analysis/include/Luau/Instantiation.h` (Instantiation.h:66-73, hand-ported)

// Instantiation 退化形态：pack 侧三槽全为 C++ 基类默认（isDirty=false、clean 恒等
// 透传、ignoreChildren=false），见 substitution_vtable 模块文档的统一安全论证。
substitution_vtable!(degen_tp, Instantiation, ic = ignore_children);
impl Instantiation {
  /// C++ `Instantiation(const TxnLog* log, TypeArena* arena, NotNull<BuiltinTypes> builtinTypes,
  /// TypeLevel level, Scope* scope) : Substitution(log, arena), builtinTypes(builtinTypes),
  /// level(level), scope(scope), reusableReplaceGenerics(log, arena, builtinTypes, level, scope, {}, {})`.
  pub fn instantiation_new(
    log: *const TxnLog,
    arena: Option<Handle<TypeArena>>,
    builtin_types: Handle<BuiltinTypes>,
    level: TypeLevel,
    scope: Option<&Scope>,
  ) -> Self {
    // 边界收口：`scope` 在 cpp 即可空 `Scope*`（旧 solver 传 nullptr），记录
    // 字段保持裸指针布局，`None` 折叠为 null 哨兵，链内传递均为 `Option<&Scope>`。
    let scope_raw = scope.map(|s| from_ref(s).cast_mut());
    Instantiation {
      base: Substitution::substitution_new(log, arena),
      builtin_types,
      level,
      scope: scope_raw.unwrap_or(null_mut()),
      reusable_replace_generics: ReplaceGenerics::replace_generics_new(
        log,
        arena,
        builtin_types,
        level,
        scope,
        Vec::new(),
        Vec::new(),
      ),
    }
  }

  substitution_entry!(id, pack);
}

impl Instantiation {
  pub fn is_dirty_type_id(&self, ty: TypeId) -> bool {
    let log = self.base.base.log;
    // Safety: log 是 Tarjan 构造期（Substitution::substitution_new）收下的当前
    // 求解事务 TxnLog 地址（C++ `const TxnLog* log` 形参直译），比 Instantiation
    // 的使用期长寿；此处只重建 &TxnLog 共享借用做只读查询（pending 映射 +
    // arena 节点），*const 类型即承诺无经此指针的可变访问，单线程无别名冲突。
    let ftv = unsafe { (*log).txn_log_get_mutable::<FunctionType, TypeId>(ty) };
    if !ftv.is_null() {
      // Safety: txn_log_get_mutable 的返回要么为 null，要么是 class-index 分派
      // 命中的 FunctionType 节点（type arena 或 log 内 pending 存储）的合法裸指针，
      // 两条路径的目标均比本次调用长寿；判空后只读 has_no_free_or_generic_types
      // 一个 bool 字段，无别名。
      if unsafe { (*ftv).has_no_free_or_generic_types } {
        return false;
      }
      return true;
    }
    false
  }

  pub fn is_dirty_type_pack_id(&self, _tp: TypePackId) -> bool {
    false
  }
}

impl Instantiation {
  pub fn reset_state(
    &mut self,
    log: *const TxnLog,
    arena: Handle<TypeArena>,
    builtin_types: Handle<BuiltinTypes>,
    level: TypeLevel,
    scope: Option<&Scope>,
  ) {
    let scope_raw = scope.map(|s| from_ref(s).cast_mut());
    Substitution::reset_state(&mut self.base, log, arena);

    self.builtin_types = builtin_types;
    self.level = level;
    self.scope = scope_raw.unwrap_or(null_mut());

    self.reusable_replace_generics.reset_state(
      log,
      arena,
      builtin_types,
      level,
      scope,
      Vec::new(),
      Vec::new(),
    );
  }
}
