//! `subtyping_environment` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;

use ulua_common::{records::dense_hash_map::DenseHashMap, LUAU_ASSERT};

use crate::{
  functions::follow_type::follow,
  methods::subtyping_bind_generic::{
    dense_hash_map_find_mut_no_default, dense_hash_map_find_no_default,
  },
  records::{
    apply_mapped_generics::ApplyMappedGenerics,
    arena_handle::Handle,
    builtin_types::BuiltinTypes,
    generic_bounds::GenericBounds,
    internal_error_reporter::InternalErrorReporter,
    mapped_generic_environment::MappedGenericEnvironment,
    subtyping_environment::{GenericScope, SubtypingEnvironment},
    subtyping_result::SubtypingResult,
    substitution::Substitution,
    txn_log::TxnLog,
    type_arena::TypeArena,
  },
  type_aliases::{lookup_result::LookupResult, type_id::TypeId, type_pack_id::TypePackId},
};

impl GenericScope {
  fn new() -> Self {
    Self {
      mapped_generics: DenseHashMap::default(),
      mapped_generic_packs: MappedGenericEnvironment {
        frames: Vec::new(),
        current_scope_index: None,
      },
      substitutions: DenseHashMap::default(),
      seen_set_cache: DenseHashMap::default(),
      iteration_count: 0,
    }
  }
}

impl SubtypingEnvironment {
  /// 根作用域构造：cpp `SubtypingEnvironment env;`（`Subtyping.cpp:618/665`）。
  pub(crate) fn new() -> Self {
    Self {
      scopes: alloc::vec![GenericScope::new()],
    }
  }

  /// 当前（栈顶）作用域，对应 cpp 传入的「当前环境」本体。
  pub(crate) fn current(&self) -> &GenericScope {
    self.scopes.last().expect("不变量：根帧恒在栈")
  }

  /// [`current`](Self::current) 的可变形态。
  pub(crate) fn current_mut(&mut self) -> &mut GenericScope {
    self.scopes.last_mut().expect("不变量：根帧恒在栈")
  }

  /// 进入 nested generic bounds 作用域：cpp `SubtypingEnvironment boundsEnv;
  /// boundsEnv.parent = &env;`（`Subtyping.cpp:3088-3089`）。
  pub(crate) fn push_scope(&mut self) {
    self.scopes.push(GenericScope::new());
  }

  /// 离开作用域并销毁该帧，对应 cpp 子环境离开作用域即析构。
  pub(crate) fn pop_scope(&mut self) {
    LUAU_ASSERT!(self.scopes.len() > 1);
    self.scopes.pop();
  }

  /// 压入一帧并返回 RAII 守卫：守卫析构（含提前 return / panic 展开）时弹栈，
  /// 对应 cpp `boundsEnv` 离开函数作用域即析构（`Subtyping.cpp:3088-3117`）。
  pub(crate) fn push_scope_guard(&mut self) -> SubtypingScopeGuard<'_> {
    self.push_scope();
    SubtypingScopeGuard { env: self }
  }
}

/// [`SubtypingEnvironment::push_scope_guard`] 的弹栈守卫。
pub(crate) struct SubtypingScopeGuard<'a> {
  /// 守卫持有的环境可变借用；作用域内调用方经 `&mut *env` 再借用传入。
  pub(crate) env: &'a mut SubtypingEnvironment,
}

impl Drop for SubtypingScopeGuard<'_> {
  fn drop(&mut self) {
    self.env.pop_scope();
  }
}

impl SubtypingEnvironment {
  /// C++ `SubtypingEnvironment::applyMappedGenerics` (`Subtyping.cpp:504-513`):
  ///
  /// ```cpp
  /// ApplyMappedGenerics amg{builtinTypes, arena, *this, iceReporter};
  /// return amg.substitute(ty);
  /// ```
  ///
  /// `ApplyMappedGenerics` extends `Substitution` and inherits `substitute`,
  /// whose traversal virtual-dispatches into the overridden `isDirty` /
  /// `clean` / `ignoreChildren`. The Rust `ApplyMappedGenerics` now embeds
  /// `base: Substitution` and installs those overrides into the
  /// `SubstitutionVtable` from its `substitute_type_id` wrapper.
  pub fn apply_mapped_generics(
    &mut self,
    builtin_types: Handle<BuiltinTypes>,
    arena: Handle<TypeArena>,
    ty: TypeId,
    ice_reporter: Handle<InternalErrorReporter>,
  ) -> Option<TypeId> {
    let mut amg = ApplyMappedGenerics {
      base: Substitution::substitution_new(TxnLog::empty(), Some(arena)),
      builtin_types,
      arena,
      ice_reporter,
      env: Handle::from_mut(self),
    };
    amg.substitute_type_id(ty)
  }
}

impl SubtypingEnvironment {
  /// cpp `containsMappedPack`（`Subtyping.cpp:565-574`）：本帧 pack 映射命中即真，
  /// 否则沿父链外行；栈形态下等价于「任意帧 lookup 到 TypePackId」。
  pub fn contains_mapped_pack(&self, tp: TypePackId) -> bool {
    self.lookup_generic_pack(tp).get_if::<TypePackId>().is_some()
  }
}

impl SubtypingEnvironment {
  /// cpp `containsMappedType`（`Subtyping.cpp:553-563`）：自栈顶向外逐帧探测
  /// 本帧 `mapped_generics` 非空界。
  pub fn contains_mapped_type(&self, ty: TypeId) -> bool {
    let ty = follow(ty);
    self.scopes.iter().rev().any(|scope| {
      dense_hash_map_find_no_default(&scope.mapped_generics, &ty)
        .is_some_and(|bounds| !bounds.is_empty())
    })
  }
}

impl SubtypingEnvironment {
  /// 查找 `ty`（经 `follow`）在作用域栈中对应的泛型约束边界，自栈顶向外取
  /// 首个非空 bounds 的最后一项。
  ///
  /// 前提由 `SubtypingEnvironment` 构造不变量保证：`ice_reporter` 非空
  /// （`Handle` 类型编码）且在本调用返回前存活；`ty` 为类型 arena 中存活节点
  /// （与任意 `TypeId` 用法同契约）。
  pub(crate) fn get_mapped_type_bounds(
    &mut self,
    ty: TypeId,
    ice_reporter: Handle<InternalErrorReporter>,
  ) -> &mut GenericBounds {
    let ty = follow(ty);
    // 先以共享遍历定位命中帧（可变迭代器元素引用无法逃逸出循环，故两趟；
    // 帧深为 generic 嵌套层数，O(层数) 探测，冷路径）。
    let offset_from_top = self.scopes.iter().rev().position(|scope| {
      dense_hash_map_find_no_default(&scope.mapped_generics, &ty)
        .is_some_and(|bounds| !bounds.is_empty())
    });
    let Some(offset_from_top) = offset_from_top else {
      LUAU_ASSERT!(false);
      // `ice_reporter` 为 Handle（非空由类型编码，对应 C++ 引用形参）；`ice_string`
      // 只上报一条诊断消息并 panic 发散，不产生并存别名。
      ice_reporter
        .get()
        .ice_string("Trying to access bounds for a type with no in-scope bounds");
      unreachable!()
    };
    let from_top = self.scopes.len() - 1 - offset_from_top;
    let scope = &mut self.scopes[from_top];
    let bounds = dense_hash_map_find_mut_no_default(&mut scope.mapped_generics, &ty)
      .expect("共享探测趟已命中该键，可变趟无删改必再命中");
    // 命中判据含 `!bounds.is_empty()`，蕴含 last_mut() 命中 Some。
    bounds.last_mut().expect("判据 !bounds.is_empty() 蕴含非空")
  }
}

impl SubtypingEnvironment {
  /// cpp `lookupGenericPack`（`Subtyping.cpp:590-600`）：自栈顶向外取首个
  /// `TypePackId` 命中；全程未命中时返回最外层（根帧）自身的查找结果，
  /// 与原父链递归在最外层返回 `result` 逐字一致。
  pub fn lookup_generic_pack(&self, tp: TypePackId) -> LookupResult {
    let mut outermost: Option<LookupResult> = None;
    for scope in self.scopes.iter().rev() {
      let result = scope.mapped_generic_packs.lookup_generic_pack(tp);
      if result.get_if::<TypePackId>().is_some() {
        return result;
      }
      outermost = Some(result);
    }
    outermost.expect("不变量：根帧恒在栈")
  }
}

impl SubtypingEnvironment {
  pub fn try_find_substitution(&self, ty: TypeId) -> Option<TypeId> {
    self
      .scopes
      .iter()
      .rev()
      .find_map(|scope| scope.substitutions.find(&ty))
      .copied()
  }
}

impl SubtypingEnvironment {
  pub fn try_find_subtyping_result(
    &self,
    sub_and_super: (TypeId, TypeId),
  ) -> Option<&SubtypingResult> {
    self
      .scopes
      .iter()
      .rev()
      .find_map(|scope| scope.seen_set_cache.find(&sub_and_super))
  }
}
