//! `overload_resolver` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;

use ulua_ast::records::location::Location;
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{
    begin_type::begin_intersection_type, flatten_type_pack::flatten_type_pack_id, follow_type,
    get_type, is_optional_type::is_optional_type,
  },
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes,
    internal_error_reporter::InternalErrorReporter, intersection_type::IntersectionType,
    normalizer::Normalizer, overload_resolution::OverloadResolution,
    overload_resolver::OverloadResolver, scope::Scope, subtyping::Subtyping, type_arena::TypeArena,
    type_check_limits::TypeCheckLimits, type_function_runtime::TypeFunctionRuntime,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl OverloadResolver<'_> {
  pub(crate) fn is_arity_compatible(
    &self,
    candidate: TypePackId,
    desired: TypePackId,
    builtin_types: Handle<BuiltinTypes>,
  ) -> bool {
    let (candidate_head, candidate_tail) = flatten_type_pack_id(candidate);
    let (desired_head, desired_tail) = flatten_type_pack_id(desired);

    if candidate_head.len() < desired_head.len() {
      if candidate_tail.is_some() {
        return true;
      }

      let all_unsatisfied_optional = desired_head[candidate_head.len()..].iter().all(|&d| {
        // builtin_types 源自 OverloadResolver 构造期布线的会话句柄（C++
        // NotNull<BuiltinTypes>），指向对象存活于 self 生命周期；is_optional_type 对其只读。
        is_optional_type(follow_type::follow(d), builtin_types)
      });
      if !all_unsatisfied_optional {
        return false;
      }
    }

    if candidate_head.len() > desired_head.len() {
      return desired_tail.is_some();
    }

    true
  }
}

impl<'a> OverloadResolver<'a> {
  /// C++ `OverloadResolver(NotNull<BuiltinTypes>, NotNull<TypeArena>, NotNull<Normalizer>,
  /// NotNull<TypeFunctionRuntime>, NotNull<Scope>, NotNull<InternalErrorReporter>,
  /// NotNull<TypeCheckLimits>, Location)`。形参全为受检句柄/引用后本构造器即为安全
  /// 函数：非空由 `Handle`/`&` 类型编码，`scope`/`limits` 为非拥有共享借用——
  /// **绝不拥有、绝不 drop**，其存活期由 `'a` 覆盖整个 resolver 使用点。
  /// 入参 `normalizer`/`type_function_runtime`/`reporter` 取独立借用期（不与
  /// `'a` 绑定）：构造器只把地址收进 `Handle`（指针值拷贝，借用即还），调用点
  /// 不必为 resolver 全程冻结宿主借用；`scope`/`limits` 为 `'a` 共享借用字段。
  pub fn new(
    builtin_types: Handle<BuiltinTypes>,
    arena: Handle<TypeArena>,
    normalizer: &Normalizer,
    type_function_runtime: &TypeFunctionRuntime,
    scope: &'a Scope,
    reporter: &InternalErrorReporter,
    limits: &'a TypeCheckLimits,
    call_location: Location,
  ) -> Self {
    Self {
      builtin_types,
      arena,
      normalizer: Handle::from_ref(normalizer),
      type_function_runtime: Handle::from_ref(type_function_runtime),
      scope,
      ice: Handle::from_ref(reporter),
      // C++ `NotNull<TypeCheckLimits> limits` 的非拥有直传；不再 clone，
      // 与上游共享同一 limits 对象（改写对原对象可见）。
      limits,
      subtyping: Subtyping::subtyping_owned(
        builtin_types,
        arena,
        Some(Handle::from_ref(normalizer)),
        type_function_runtime,
        reporter,
      ),
      call_loc: call_location,
    }
  }
}

// Source: `Analysis/src/OverloadResolver.cpp:175-196` (hand-ported)

impl OverloadResolver<'_> {
  /// C++ `OverloadResolution resolveOverload(TypeId ty, TypePackId argsPack, Location fnLocation, NotNull<DenseHashSet<TypeId>> uniqueTypes, bool useFreeTypeBounds)`.
  pub fn resolve_overload(
    &mut self,
    ty: TypeId,
    args_pack: TypePackId,
    fn_location: Location,
    unique_types: *mut DenseHashSet<TypeId>,
    _use_free_type_bounds: bool,
  ) -> OverloadResolution {
    let mut result = OverloadResolution {
      ok: Vec::new(),
      non_functions: Vec::new(),
      potential_overloads: Vec::new(),
      incompatible_overloads: Vec::new(),
      arity_mismatches: Vec::new(),
      metamethods: DenseHashSet::default(),
    };

    let ty = follow_type::follow(ty);

    if let Some(it) = get_type::get::<IntersectionType>(ty) {
      // C++ `for (TypeId component : it)`——IntersectionTypeIterator 展平
      // 嵌套 intersection 并 follow,裸遍历 parts 会漏掉嵌套成员。
      for component in begin_intersection_type(it) {
        self.test_function_or_union(&mut result, component, args_pack, fn_location, unique_types);
      }
    } else {
      self.test_function_or_union(&mut result, ty, args_pack, fn_location, unique_types);
    }

    result
  }
}
