use alloc::vec::Vec;
use core::{
  cmp::{max, min},
  ptr::NonNull,
};

use ulua_common::{fflag, fint, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::{
    occurs_check_result::OccursCheckResult, table_state::TableState, unify_result::UnifyResult,
  },
  functions::{
    are_compatible::are_compatible,
    as_mutable_type_pack::as_mutable_type_pack,
    begin_type::{begin_intersection_type, begin_union_type},
    emplace_type_pack::emplace_type_pack,
    extend_type_pack::extend_type_pack,
    flatten_type_pack::flatten_type_pack_id,
    follow_type, follow_type_pack, get_mutable_type, get_type, get_type_pack,
    is_irresolvable_unifier_2::{is_irresolvable, is_irresolvable_type_pack},
    occurs_check_type_utils::occurs_check_type_pack_id_type_pack_id,
  },
  records::{
    any_type::AnyType, arena_handle::Handle, builtin_types::BuiltinTypes, free_type::FreeType,
    free_type_pack::FreeTypePack, function_type::FunctionType, generic_type::GenericType,
    generic_type_pack::GenericTypePack, intersection_type::IntersectionType,
    metatable_type::MetatableType, negation_type::NegationType, never_type::NeverType,
    non_exceptional_recursion_limiter::NonExceptionalRecursionLimiter,
    pack_subtype_constraint::PackSubtypeConstraint, replacer::Replacer,
    subtype_constraint::SubtypeConstraint, table_indexer::TableIndexer, table_type::TableType,
    type_pack::TypePack, unifier_2::Unifier2, union_type::UnionType, unknown_type::UnknownType,
  },
  type_aliases::{
    constraint_v::ConstraintV, type_id::TypeId, type_pack_id::TypePackId,
    type_pack_variant::TypePackVariant,
  },
};

impl Unifier2 {
  pub fn unify(&mut self, sub_ty: TypeId, super_ty: TypeId) -> UnifyResult {
    self.iteration_count = 0;
    self.unify_type_id_type_id(sub_ty, super_ty)
  }

  pub fn unify_pack(&mut self, sub_tp: TypePackId, super_tp: TypePackId) -> UnifyResult {
    self.iteration_count = 0;
    self.unify_type_pack_id_type_pack_id(sub_tp, super_tp)
  }

  pub(crate) fn unify_type_id_type_id(
    &mut self,
    mut sub_ty: TypeId,
    mut super_ty: TypeId,
  ) -> UnifyResult {
    if fint::LuauTypeInferIterationLimit.get() > 0
      && self.iteration_count >= fint::LuauTypeInferIterationLimit.get()
    {
      return UnifyResult::TooComplex;
    }

    self.iteration_count += 1;

    // NOTE: It's a little odd that we are doing something non-exceptional for
    // the core of unification but not for occurs check, which may throw an
    // exception. It would be nice if, in the future, this were unified.
    // C++ `NonExceptionalRecursionLimiter nerl{&recursionCount}` 无条件构造：
    // ctor `++*count`、drop `--*count`，守卫须贯穿整个函数体，嵌套调用才能
    // 看到累计深度。曾误加 LuauLimitUnificationRecursion 门控（C++ 无此开关）
    // 且手动 +1 与 ctor +1 重复计数，导致限位失效。
    let _nerl = NonExceptionalRecursionLimiter::new(&mut self.recursion_count);
    if !_nerl.is_ok(self.recursion_limit) {
      return UnifyResult::TooComplex;
    }

    sub_ty = follow_type::follow(sub_ty);
    super_ty = follow_type::follow(super_ty);

    if let Some(sub_gen) = self.generic_substitutions.find(&sub_ty) {
      let sub_gen = *sub_gen;
      return self.unify_type_id_type_id(sub_gen, super_ty);
    }

    if let Some(super_gen) = self.generic_substitutions.find(&super_ty) {
      let super_gen = *super_gen;
      return self.unify_type_id_type_id(sub_ty, super_gen);
    }

    if self.seen_type_pairings.contains(&(sub_ty, super_ty)) {
      return UnifyResult::Ok;
    }
    self.seen_type_pairings.insert((sub_ty, super_ty));

    if sub_ty == super_ty {
      return UnifyResult::Ok;
    }

    // We have potentially done some unifications while dispatching either `SubtypeConstraint` or `PackSubtypeConstraint`,
    // so rather than implementing backtracking or traversing the entire type graph multiple times, we could push
    // additional constraints as we discover blocked types along with their proper bounds.
    //
    // But we exclude these two subtyping patterns, they are tautological:
    //   - never <: *blocked*
    //   - *blocked* <: unknown
    if (is_irresolvable(sub_ty) || is_irresolvable(super_ty))
      && get_type::get::<NeverType>(sub_ty).is_none()
      && get_type::get::<UnknownType>(super_ty).is_none()
    {
      if !self.uninhabited_type_functions.is_null()
        // Safety: 字段由构造期从 `&mut ConstraintSolver.uninhabited_type_functions`
        // 注入（4 参重载传 null，故先经 is_null 短路），指向 solver 自有集合，
        // 活过本次 unify 借用窗口；Unifier2 全程只对该集合做只读 `contains`，
        // 从不写回，且注入后 solver 在 Unifier2 存活期不再触碰该字段。
        && unsafe {
          (*self.uninhabited_type_functions).contains(&(sub_ty as *const ()))
            || (*self.uninhabited_type_functions).contains(&(super_ty as *const ()))
        }
      {
        return UnifyResult::Ok;
      }

      self
        .incomplete_subtypes
        .push(ConstraintV::Subtype(SubtypeConstraint {
          sub_type: sub_ty,
          super_type: super_ty,
        }));
      return UnifyResult::Ok;
    }

    // C++ Unifier2.cpp:206-218：superFree 先合并 lowerBound，subFree 走 unifyFreeWithType
    if let Some(super_free) = get_mutable_type::get_mutable::<FreeType>(super_ty) {
      let instantiated = self.instantiate_with_bound_types(sub_ty);
      let new_lower = self.mk_union(super_free.lower_bound, instantiated);
      super_free.lower_bound = new_lower;
    }

    let sub_free = get_mutable_type::get_mutable::<FreeType>(sub_ty);
    if sub_free.is_some() {
      return self.unify_free_with_type(sub_ty, super_ty);
    }

    // subFree 已排除（否则上面已 return），仅剩 superFree 情形
    if sub_free.is_some() || get_mutable_type::get_mutable::<FreeType>(super_ty).is_some() {
      return UnifyResult::Ok;
    }

    let sub_fn = get_type::get::<FunctionType>(sub_ty);
    let super_fn = get_type::get::<FunctionType>(super_ty);
    if let (Some(_), Some(super_fn_ref)) = (sub_fn, super_fn) {
      return self.unify_type_id_function_type(sub_ty, super_fn_ref);
    }

    let sub_union = get_type::get::<UnionType>(sub_ty);
    let super_union = get_type::get::<UnionType>(super_ty);
    if let Some(sub_union) = sub_union {
      // Safety: `sub_union` 是刚经 get_type_id 从存活 arena 节点借出的
      // UnionType 引用，`super_ty` 为函数头 follow 后的活句柄；二者的
      // options/本体 TypeId 均指向同一存活 arena，满足被调方契约。
      return unsafe { self.unify_union_type_type_id(sub_union, super_ty) };
    } else if let Some(super_union) = super_union {
      return self.unify_type_id_union_type(sub_ty, super_union);
    }

    let sub_intersection = get_type::get::<IntersectionType>(sub_ty);
    let super_intersection = get_type::get::<IntersectionType>(super_ty);
    if let Some(sub_intersection) = sub_intersection {
      return self.unify_intersection_type_type_id(sub_intersection, super_ty);
    } else if let Some(super_intersection) = super_intersection {
      return self.unify_type_id_intersection_type(sub_ty, super_intersection);
    }

    let sub_never = get_type::get::<NeverType>(sub_ty);
    let super_never = get_type::get::<NeverType>(super_ty);
    if let (Some(_), Some(_)) = (sub_never, super_never) {
      return UnifyResult::Ok;
    } else if let (Some(_), Some(super_fn_ref)) = (sub_never, super_fn) {
      // If `never` is the subtype, then we can propagate that inward.
      // Safety: `builtin_types` 是构造期注入的 `NonNull<BuiltinTypes>`（TypeChecker
      // 持有，活过本 Unifier2），内建类型注册后只读；此处共享解引用不写，且
      // 引用并非自 `&mut self` 再借，无别名冲突。
      let builtin_types = unsafe { &*self.builtin_types.as_ptr() };
      let never_pack = builtin_types.never_type_pack;
      let arg_result = self.unify_type_pack_id_type_pack_id(super_fn_ref.arg_types, never_pack);
      let ret_result = self.unify_type_pack_id_type_pack_id(never_pack, super_fn_ref.ret_types);
      return arg_result & ret_result;
    } else if let (Some(sub_fn_ref), Some(_)) = (sub_fn, super_never) {
      // If `never` is the supertype, then we can propagate that inward.
      // Safety: 同上——构造期注入的共享 BuiltinTypes，注册后只读，此处仅取
      // `never_type_pack` Copy 句柄。
      let builtin_types = unsafe { &*self.builtin_types.as_ptr() };
      let never_pack = builtin_types.never_type_pack;
      let arg_result = self.unify_type_pack_id_type_pack_id(never_pack, sub_fn_ref.arg_types);
      let ret_result = self.unify_type_pack_id_type_pack_id(sub_fn_ref.ret_types, never_pack);
      return arg_result & ret_result;
    }

    let sub_any = get_type::get::<AnyType>(sub_ty);
    let super_any = get_type::get::<AnyType>(super_ty);

    let mut sub_table = get_mutable_type::get_mutable::<TableType>(sub_ty);
    let super_table = get_type::get::<TableType>(super_ty);

    if let (Some(_), Some(_)) = (sub_any, super_any) {
      return UnifyResult::Ok;
    } else if let (Some(sub_any), Some(super_fn_ref)) = (sub_any, super_fn) {
      return self.unify_any_type_function_type(sub_any, super_fn_ref);
    } else if let (Some(sub_fn_ref), Some(super_any)) = (sub_fn, super_any) {
      return self.unify_function_type_any_type(sub_fn_ref, super_any);
    } else if let (Some(sub_any), Some(super_table_ref)) = (sub_any, super_table) {
      return self.unify_any_type_table_type(sub_any, super_table_ref);
    } else if let (Some(sub_table_ref), Some(super_any)) = (sub_table.as_deref_mut(), super_any) {
      return self.unify_table_type_any_type(sub_table_ref, super_any);
    }

    if let (Some(sub_table), Some(super_table_ref)) = (sub_table, super_table) {
      // `bound_to` works like a bound type, and therefore we'd replace it
      // with the `bound_to` and try unification again.
      //
      // However, these pointers should have been chased already by follow().
      LUAU_ASSERT!(sub_table.bound_to.is_none());
      LUAU_ASSERT!(super_table_ref.bound_to.is_none());

      return self.unify_table_type_table_type(sub_table, super_table_ref);
    }

    let sub_metatable = get_type::get::<MetatableType>(sub_ty);
    let super_metatable = get_type::get::<MetatableType>(super_ty);
    if let (Some(sub_mt), Some(super_mt)) = (sub_metatable, super_metatable) {
      return self.unify_metatable_type_metatable_type(sub_mt, super_mt);
    } else if let (Some(sub_mt), Some(super_any_ref)) = (sub_metatable, super_any) {
      return self.unify_metatable_type_any_type(sub_mt, super_any_ref);
    } else if let (Some(sub_any_ref), Some(super_mt)) = (sub_any, super_metatable) {
      return self.unify_any_type_metatable_type(sub_any_ref, super_mt);
    } else if let Some(sub_mt) = sub_metatable {
      // if we only have one metatable, unify with the inner table
      let inner = sub_mt.table();
      return self.unify_type_id_type_id(inner, super_ty);
    } else if let Some(super_mt) = super_metatable {
      // if we only have one metatable, unify with the inner table
      let inner = super_mt.table();
      return self.unify_type_id_type_id(sub_ty, inner);
    }

    let sub_negation = get_type::get::<NegationType>(sub_ty);
    let super_negation = get_type::get::<NegationType>(super_ty);
    if let (Some(sub_neg), Some(super_neg)) = (sub_negation, super_negation) {
      return self.unify_type_id_type_id(sub_neg.ty, super_neg.ty);
    }

    // The unification failed, but we're not doing type checking.
    UnifyResult::Ok
  }

  pub fn unify_type_id_function_type(
    &mut self,
    sub_ty: TypeId,
    super_fn: &FunctionType,
  ) -> UnifyResult {
    // C++ Unifier2.cpp:408 get<FunctionType>(subTy) 无空检查，调用方保证 subTy 为函数类型
    let sub_fn = get_type::get::<FunctionType>(sub_ty)
      .expect("分派调用方保证 sub_ty 为函数类型（cpp get<> 无空检查同位）");

    let should_instantiate = (super_fn.generics.is_empty() && !sub_fn.generics.is_empty())
      || (super_fn.generic_packs.is_empty() && !sub_fn.generic_packs.is_empty());

    if should_instantiate {
      for &generic in sub_fn.generics.iter() {
        let generic = follow_type::follow(generic);
        if let Some(r#gen) = get_type::get::<GenericType>(generic) {
          let fresh = self.fresh_type(self.scope, r#gen.polarity);
          *self.generic_substitutions.get_or_insert(generic) = fresh;
        }
      }

      for &generic_pack in sub_fn.generic_packs.iter() {
        let generic_pack = follow_type_pack::follow(generic_pack);
        if let Some(r#gen) = get_type_pack::get::<GenericTypePack>(generic_pack) {
          let fresh = self.fresh_type_pack(self.scope, r#gen.polarity);
          *self.generic_pack_substitutions.get_or_insert(generic_pack) = fresh;
        }
      }
    }

    let arg_result = self.unify_type_pack_id_type_pack_id(super_fn.arg_types, sub_fn.arg_types);
    let ret_result = self.unify_type_pack_id_type_pack_id(sub_fn.ret_types, super_fn.ret_types);
    arg_result & ret_result
  }

  /// 对应 C++ `Unifier2::unify_(const UnionType* subUnion, TypeId superTy)`。
  ///
  /// # Safety
  /// - `sub_union`：必须借自存活 arena 中的 `UnionType` 节点，且其 `options`
  ///   里每个 `TypeId`（裸句柄）都指向同一仍在世且变体未被 emplace 改写前
  ///   失效的 arena 节点——函数体把 `*sub_option` 传入 `are_compatible` 与
  ///   `unify_type_id_type_id`，二者会沿句柄 `follow` 并瞬时解引用节点。
  /// - `super_ty`：同为指向存活类型节点的 `TypeId` 句柄（调用方在
  ///   `unify_type_id_type_id` 内已 `follow_type_id`），满足 `are_compatible`
  ///   的类型存活契约。
  ///
  /// 本仓库唯一调用点（同文件 `unify_type_id_type_id` 的 union 分支）在 follow
  /// 与 get_type_id 之后传参，契约天然成立；函数为 `pub`，外部复用须同守。
  pub unsafe fn unify_union_type_type_id(
    &mut self,
    sub_union: &UnionType,
    super_ty: TypeId,
  ) -> UnifyResult {
    let mut result = UnifyResult::Ok;

    for sub_option in sub_union.options.iter() {
      if are_compatible(*sub_option, super_ty) {
        result &= self.unify_type_id_type_id(*sub_option, super_ty);
      }
    }

    result
  }

  pub fn unify_type_id_union_type(
    &mut self,
    sub_ty: TypeId,
    super_union: &UnionType,
  ) -> UnifyResult {
    let sub_ty = follow_type::follow(sub_ty);

    // T <: T | U1 | U2 | ... | Un is trivially true, so we don't gain any information by unifying
    // C++ `for (const auto superOption : superUnion)` — UnionTypeIterator 防环展平
    // 并 follow,裸遍历 options 会漏掉嵌套 union。
    for super_option in begin_union_type(super_union) {
      let followed_super_option = follow_type::follow(super_option);
      if sub_ty == followed_super_option {
        return UnifyResult::Ok;
      }
    }

    let mut result = UnifyResult::Ok;

    // if the occurs check fails for any option, it fails overall
    for super_option in super_union.options.iter() {
      if are_compatible(sub_ty, *super_option) {
        result &= self.unify_type_id_type_id(sub_ty, *super_option);
      }
    }

    result
  }

  pub fn unify_intersection_type_type_id(
    &mut self,
    sub_intersection: &IntersectionType,
    super_ty: TypeId,
  ) -> UnifyResult {
    let super_ty = follow_type::follow(super_ty);

    // C++ `for (const auto subOption : subIntersection)` — IntersectionTypeIterator
    // 防环展平并 follow Bound,裸遍历 parts 会漏掉嵌套 intersection。
    for sub_option in begin_intersection_type(sub_intersection) {
      let followed_sub_option = follow_type::follow(sub_option);
      if super_ty == followed_sub_option {
        return UnifyResult::Ok;
      }
    }

    let mut result = UnifyResult::Ok;

    for sub_part in sub_intersection.parts.iter() {
      result &= self.unify_type_id_type_id(*sub_part, super_ty);
    }

    result
  }

  pub fn unify_type_id_intersection_type(
    &mut self,
    sub_ty: TypeId,
    super_intersection: &IntersectionType,
  ) -> UnifyResult {
    let mut result = UnifyResult::Ok;

    for super_part in super_intersection.parts.iter() {
      result &= self.unify_type_id_type_id(sub_ty, *super_part);
    }

    result
  }

  pub fn unify_table_type_table_type(
    &mut self,
    sub_table: &mut TableType,
    super_table: &TableType,
  ) -> UnifyResult {
    let mut result = UnifyResult::Ok;

    for (prop_name, sub_prop) in &sub_table.props {
      if let Some(super_prop_opt) = super_table.props.get(prop_name) {
        let super_prop = super_prop_opt;

        if let (Some(sub_read), Some(super_read)) = (sub_prop.read_ty, super_prop.read_ty) {
          result &= self.unify_type_id_type_id(sub_read, super_read);
        }

        // C++ 写属性方向反转：`unify_(*superProp.writeTy, *subProp.writeTy)`（写逆变）。
        if let (Some(sub_write), Some(super_write)) = (sub_prop.write_ty, super_prop.write_ty) {
          result &= self.unify_type_id_type_id(super_write, sub_write);
        }
      }
    }

    let mut sub_type_params_iter = sub_table.instantiated_type_params.iter();
    let mut super_type_params_iter = super_table.instantiated_type_params.iter();

    while let (Some(sub_tp), Some(super_tp)) =
      (sub_type_params_iter.next(), super_type_params_iter.next())
    {
      result &= self.unify_type_id_type_id(*sub_tp, *super_tp);
    }

    let mut sub_type_pack_params_iter = sub_table.instantiated_type_pack_params.iter();
    let mut super_type_pack_params_iter = super_table.instantiated_type_pack_params.iter();

    while let (Some(sub_tpp), Some(super_tpp)) = (
      sub_type_pack_params_iter.next(),
      super_type_pack_params_iter.next(),
    ) {
      result &= self.unify_type_pack_id_type_pack_id(*sub_tpp, *super_tpp);
    }

    if let (Some(sub_indexer), Some(super_indexer)) = (&sub_table.indexer, &super_table.indexer) {
      result &= self.unify_type_id_type_id(sub_indexer.index_type, super_indexer.index_type);
      result &= self.unify_type_id_type_id(
        sub_indexer.index_result_type,
        super_indexer.index_result_type,
      );

      result &= self.unify_type_id_type_id(super_indexer.index_type, sub_indexer.index_type);
      result &= self.unify_type_id_type_id(
        super_indexer.index_result_type,
        sub_indexer.index_result_type,
      );
    }

    if sub_table.indexer.is_none()
      && sub_table.state == TableState::Unsealed
      && let Some(super_indexer) = super_table.indexer.as_ref()
    {
      // FFlag 开启时走 instantiateWithBoundTypes，避免把泛型裸漏进 unsealed 表的
      // indexer（Unifier2.cpp:598 `LuauDoNotLeakGenericsInIndexer`）。
      let (index_type, index_result_type) = if fflag::LuauDoNotLeakGenericsInIndexer.get() {
        (
          self.instantiate_with_bound_types(super_indexer.index_type),
          self.instantiate_with_bound_types(super_indexer.index_result_type),
        )
      } else {
        let mut index_type = super_indexer.index_type;
        if let Some(subst) = self.generic_substitutions.find(&index_type) {
          index_type = *subst;
        }

        let mut index_result_type = super_indexer.index_result_type;
        if let Some(subst) = self.generic_substitutions.find(&index_result_type) {
          index_result_type = *subst;
        }
        (index_type, index_result_type)
      };

      sub_table.indexer = Some(TableIndexer {
        index_type,
        index_result_type,
        is_read_only: false,
      });
    }

    result
  }

  pub fn unify_metatable_type_metatable_type(
    &mut self,
    sub_metatable: &MetatableType,
    super_metatable: &MetatableType,
  ) -> UnifyResult {
    let metatable_result =
      self.unify_type_id_type_id(sub_metatable.metatable, super_metatable.metatable);
    if metatable_result != UnifyResult::Ok {
      return metatable_result;
    }
    self.unify_type_id_type_id(sub_metatable.table, super_metatable.table)
  }

  pub fn unify_any_type_function_type(
    &mut self,
    _sub_any: &AnyType,
    super_fn: &FunctionType,
  ) -> UnifyResult {
    // Safety: 构造期注入的共享 BuiltinTypes（TypeChecker 持有，活过 self），
    // 内建 any_type_pack 注册后只读，共享解引用无写冲突。
    let builtin_types = unsafe { &*self.builtin_types.as_ptr() };
    let arg_result =
      self.unify_type_pack_id_type_pack_id(super_fn.arg_types, builtin_types.any_type_pack);
    let ret_result =
      self.unify_type_pack_id_type_pack_id(builtin_types.any_type_pack, super_fn.ret_types);
    arg_result & ret_result
  }

  pub fn unify_function_type_any_type(
    &mut self,
    sub_fn: &FunctionType,
    _super_any: &AnyType,
  ) -> UnifyResult {
    // Safety: 同上——构造期注入的共享 BuiltinTypes，此处仅只读 any_type_pack。
    let builtin_types = unsafe { &*self.builtin_types.as_ptr() };
    let arg_result =
      self.unify_type_pack_id_type_pack_id(builtin_types.any_type_pack, sub_fn.arg_types);
    let ret_result =
      self.unify_type_pack_id_type_pack_id(sub_fn.ret_types, builtin_types.any_type_pack);
    arg_result & ret_result
  }

  pub fn unify_any_type_table_type(
    &mut self,
    _sub_any: &AnyType,
    super_table: &TableType,
  ) -> UnifyResult {
    let builtin_types_ptr: NonNull<BuiltinTypes> = self.builtin_types;
    // Safety: NonNull 字段在构造期由 `NonNull::new(self.builtin_types).unwrap()`
    // 注入（constraint_solver_try_dispatch 布线），保证非空且指向 TypeChecker
    // 持有的存活 BuiltinTypes；`as_ref` 得到只读视图，内建类型注册后不变。
    let builtin_types_ref: &BuiltinTypes = unsafe { builtin_types_ptr.as_ref() };
    let any_type_id: TypeId = builtin_types_ref.any_type;

    for prop in super_table.props.values() {
      if let Some(read_ty) = prop.read_ty {
        let _ = self.unify_type_id_type_id(any_type_id, read_ty);
      }

      if let Some(write_ty) = prop.write_ty {
        let _ = self.unify_type_id_type_id(write_ty, any_type_id);
      }
    }

    if let Some(indexer) = &super_table.indexer {
      let _ = self.unify_type_id_type_id(any_type_id, indexer.index_type);
      let _ = self.unify_type_id_type_id(any_type_id, indexer.index_result_type);
    }

    UnifyResult::Ok
  }

  pub fn unify_table_type_any_type(
    &mut self,
    sub_table: &TableType,
    _super_any: &AnyType,
  ) -> UnifyResult {
    // Safety: 构造期注入的共享 BuiltinTypes（TypeChecker 持有，活过 self，
    // 注册后只读），一次读出 Copy 句柄供下方各处使用，替代循环内多处重复
    // 解引用（C++ `&*builtinTypes` 传参同语义）。
    let any_type = unsafe { (*self.builtin_types.as_ptr()).any_type };

    for prop in sub_table.props.values() {
      if let Some(read_ty) = prop.read_ty {
        let _ = self.unify_type_id_type_id(read_ty, any_type);
      }

      if let Some(write_ty) = prop.write_ty {
        let _ = self.unify_type_id_type_id(any_type, write_ty);
      }
    }

    if let Some(indexer) = &sub_table.indexer {
      let _ = self.unify_type_id_type_id(indexer.index_type, any_type);
      let _ = self.unify_type_id_type_id(indexer.index_result_type, any_type);
    }

    UnifyResult::Ok
  }

  pub fn unify_metatable_type_any_type(
    &mut self,
    sub_metatable: &MetatableType,
    _super_any: &AnyType,
  ) -> UnifyResult {
    // Safety: 构造期注入的共享 BuiltinTypes，any_type 为注册后不变的 Copy 句柄，
    // 只读解引用。
    let builtin_types = unsafe { &*self.builtin_types.as_ptr() };
    let metatable_result =
      self.unify_type_id_type_id(sub_metatable.metatable, builtin_types.any_type);
    if metatable_result != UnifyResult::Ok {
      return metatable_result;
    }
    self.unify_type_id_type_id(sub_metatable.table, builtin_types.any_type)
  }

  pub fn unify_any_type_metatable_type(
    &mut self,
    _sub_any: &AnyType,
    super_metatable: &MetatableType,
  ) -> UnifyResult {
    // Safety: 构造期注入的共享 BuiltinTypes（存活且注册后只读），仅读取 any_type。
    let builtin_types = unsafe { &*self.builtin_types.as_ptr() };
    let metatable_result =
      self.unify_type_id_type_id(builtin_types.any_type, super_metatable.metatable);
    if metatable_result != UnifyResult::Ok {
      return metatable_result;
    }
    self.unify_type_id_type_id(builtin_types.any_type, super_metatable.table)
  }

  pub(crate) fn unify_type_pack_id_type_pack_id(
    &mut self,
    mut sub_tp: TypePackId,
    mut super_tp: TypePackId,
  ) -> UnifyResult {
    if fint::LuauTypeInferIterationLimit.get() > 0
      && self.iteration_count >= fint::LuauTypeInferIterationLimit.get()
    {
      return UnifyResult::TooComplex;
    }

    self.iteration_count += 1;

    // C++ `NonExceptionalRecursionLimiter nerl{&recursionCount}` 无条件构造：
    // ctor `++*count`、drop `--*count`，守卫须贯穿整个函数体，嵌套调用才能
    // 看到累计深度。曾误加 LuauLimitUnificationRecursion 门控（C++ 无此开关）
    // 且手动 +1 与 ctor +1 重复计数，导致限位失效。
    let _nerl = NonExceptionalRecursionLimiter::new(&mut self.recursion_count);
    if !_nerl.is_ok(self.recursion_limit) {
      return UnifyResult::TooComplex;
    }

    sub_tp = follow_type_pack::follow(sub_tp);
    super_tp = follow_type_pack::follow(super_tp);

    if self.seen_type_pack_pairings.contains(&(sub_tp, super_tp)) {
      return UnifyResult::Ok;
    }
    self.seen_type_pack_pairings.insert((sub_tp, super_tp));

    if sub_tp == super_tp {
      return UnifyResult::Ok;
    }

    // FIXME: CLI-188000: If we are _directly_ given a free type, we must
    // eagerly emplace it. Otherwise, later, we may generalize the underlying
    // free types incorrectly.
    if get_type_pack::get::<FreeTypePack>(sub_tp).is_some() {
      return self.emplace_free_type_pack(sub_tp, super_tp);
    }

    if get_type_pack::get::<FreeTypePack>(super_tp).is_some() {
      return self.emplace_free_type_pack(super_tp, sub_tp);
    }

    let sub_len = flatten_type_pack_id(sub_tp).0.len();
    let super_len = flatten_type_pack_id(super_tp).0.len();
    let max_length = max(sub_len, super_len);

    let arena = self.arena.as_ptr();
    let builtin_types_ptr = self.builtin_types.as_ptr();

    // Safety: `arena` 源自构造期注入的共享 TypeArena（NonNull 字段，TypeChecker
    // 持有，活过本 Unifier2），TypedAllocator 按节点独立给址，追加不搬移既有
    // 节点；`&mut *arena` 是仅在本次 `extend_type_pack` 调用内有效的临时再借，
    // 调用只向 arena 追加新 pack 节点并读 `builtin_types_ptr`（同为有效
    // NonNull 字段转裸）。
    let sub_extended = unsafe {
      extend_type_pack(
        &mut *arena,
        Handle::from_ptr(builtin_types_ptr),
        sub_tp,
        max_length,
        Vec::new(),
      )
    };
    // Safety: 同上；上一处临时 `&mut *arena` 再借已随调用结束析生，两次再借
    // 不共存，任一时刻指向 arena 的 &mut 至多一个存活。
    let super_extended = unsafe {
      extend_type_pack(
        &mut *arena,
        Handle::from_ptr(builtin_types_ptr),
        super_tp,
        max_length,
        Vec::new(),
      )
    };

    let sub_types = sub_extended.head;
    let sub_tail = sub_extended.tail;
    let super_types = super_extended.head;
    let super_tail = super_extended.tail;

    let limit = min(sub_types.len(), super_types.len());
    for (sub, sup) in sub_types.iter().zip(super_types.iter()).take(limit) {
      self.unify_type_id_type_id(*sub, *sup);
    }

    // At this point it should be the case that either:
    // - `subTypes` now has all of its types unified, and we are down to its tail
    // - `superTypes` now has all of its types unified, and we are down to its tail

    if sub_tail.is_none() && super_tail.is_none() {
      // If both types are missing a tail, we've done all we can.
      return UnifyResult::Ok;
    }

    // It should be the case that exclusively one of these packs can be reduced
    // to their tail for the rest of the function.
    if limit < sub_types.len() {
      LUAU_ASSERT!(limit == super_types.len());
      // If we have extra subtypes left over, construct a new type pack
      let new_sub_head: Vec<TypeId> = sub_types[super_types.len()..].to_vec();
      // Safety: 共享 arena 的又一次瞬时 &mut 再借（节点地址稳定，前文
      // sub_types/super_types 均为按值 Copy 的句柄快照，不受追加影响）；此前
      // 对 arena 的临时再借均已结束，栈上无第二个存活 &mut。
      sub_tp = unsafe { &mut *arena }.add_type_pack_t(TypePack::new(new_sub_head, sub_tail));
      super_tp = self.maybe_replace_tail(super_tail);
    } else if limit < super_types.len() {
      LUAU_ASSERT!(limit == sub_types.len() && limit < super_types.len());
      // If we have extra subtypes left over, construct a new type pack
      let new_super_head: Vec<TypeId> = super_types[sub_types.len()..].to_vec();
      // Safety: 同上——追加新 pack 节点的瞬时 &mut 再借，既有节点不搬移。
      super_tp = unsafe { &mut *arena }.add_type_pack_t(TypePack::new(new_super_head, super_tail));
      sub_tp = self.maybe_replace_tail(sub_tail);
    } else {
      sub_tp = self.maybe_replace_tail(sub_tail);
      super_tp = self.maybe_replace_tail(super_tail);
    }

    if is_irresolvable_type_pack(sub_tp) || is_irresolvable_type_pack(super_tp) {
      if !self.uninhabited_type_functions.is_null()
        // Safety: 同 unify_type_id_type_id 处的论证——构造期从
        // `&mut ConstraintSolver.uninhabited_type_functions` 注入，is_null 短路后
        // 非空，指向活过本次调用的 solver 集合，此处仅只读 contains。
        && unsafe {
          (*self.uninhabited_type_functions).contains(&(sub_tp as *const ()))
            || (*self.uninhabited_type_functions).contains(&(super_tp as *const ()))
        }
      {
        return UnifyResult::Ok;
      }

      self
        .incomplete_subtypes
        .push(ConstraintV::PackSubtype(PackSubtypeConstraint {
          sub_pack: sub_tp,
          super_pack: super_tp,
          returns: false,
        }));
      return UnifyResult::Ok;
    }

    // ... after doing all of our replacements, we may also need to check for
    // free types again.

    if get_type_pack::get::<FreeTypePack>(sub_tp).is_some() {
      return self.emplace_free_type_pack(sub_tp, super_tp);
    }

    if get_type_pack::get::<FreeTypePack>(super_tp).is_some() {
      return self.emplace_free_type_pack(super_tp, sub_tp);
    }

    UnifyResult::Ok
  }

  /// C++ `emplaceFreeTypePack` lambda from `unify_(TypePackId, TypePackId)`.
  fn emplace_free_type_pack(&mut self, target: TypePackId, bound_to: TypePackId) -> UnifyResult {
    LUAU_ASSERT!(get_type_pack::get::<FreeTypePack>(target).is_some());

    let bound_to = self.instantiate_with_bound_types_pack(bound_to);

    // Safety: 构造期注入的共享 BuiltinTypes（存活、注册后只读），只取 Copy
    // 句柄 error_type_pack。
    let error_pack = unsafe { (*self.builtin_types.as_ptr()).error_type_pack };

    // C++ 无条件调用 TypeUtils 自由函数 `occursCheck(target, boundTo)`；
    // 曾误加 LuauOccursCheckForAllBindings 门控（cpp 无此开关）走已废弃变体。
    if occurs_check_type_pack_id_type_pack_id(target, bound_to) == OccursCheckResult::Fail {
      // Safety: `target` 由函数头 LUAU_ASSERT 确认是 arena 中存活的
      // FreeTypePack，`as_mutable_type_pack` 返回该节点的可变裸指针；
      // emplace 将变体覆写为 Bound（C++ `ttv->emplace<BoundTypePack>` 同形），
      // 此刻栈上无对该节点的存活引用（类型读取都是句柄瞬时解引用）。
      unsafe {
        emplace_type_pack(
          as_mutable_type_pack(target),
          TypePackVariant::Bound(error_pack),
        )
      };
      return UnifyResult::OccursCheckFailed;
    }

    // Safety: 同上——target 仍是已断言存活的 free pack 节点，此处覆写为
    // Bound(bound_to)，写入窗口内无其他存活借用。
    unsafe {
      emplace_type_pack(
        as_mutable_type_pack(target),
        TypePackVariant::Bound(bound_to),
      )
    };
    UnifyResult::Ok
  }

  /// C++ `maybeReplaceTail` lambda from `unify_(TypePackId, TypePackId)`.
  fn maybe_replace_tail(&self, maybe_tp: Option<TypePackId>) -> TypePackId {
    let maybe_tp = match maybe_tp {
      // Safety: 构造期注入的共享 BuiltinTypes，empty_type_pack 为注册后不变的
      // Copy 句柄；只读解引用与 `&self` 借用不冲突。
      None => return unsafe { (*self.builtin_types.as_ptr()).empty_type_pack },
      Some(tp) => tp,
    };

    let tp = follow_type_pack::follow(maybe_tp);
    if let Some(replacement) = self.generic_pack_substitutions.find(&tp) {
      return follow_type_pack::follow(*replacement);
    }
    tp
  }

  /// `instantiateWithBoundTypes` for type packs (`Unifier2.cpp:307-314`, TypePackId
  /// instantiation of the `template<typename TID>` overload).
  fn instantiate_with_bound_types_pack(&mut self, tp: TypePackId) -> TypePackId {
    let mut r = Replacer::new(
      Handle::from_nonnull(self.arena),
      NonNull::from(&mut self.generic_substitutions).as_ptr(),
      NonNull::from(&mut self.generic_pack_substitutions).as_ptr(),
    );
    if let Some(new_tp) = r.substitute_type_pack_id(tp) {
      return new_tp;
    }
    tp
  }
}
