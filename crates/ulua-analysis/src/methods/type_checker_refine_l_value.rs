use alloc::vec::Vec;

use ulua_ast::records::location::Location;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    add_refinement::add_refinement, baseof::baseof, begin_type::begin_union_type, follow_type,
    get_type,
  },
  records::{
    arena_handle::alias_ref, field::Field, never_type::NeverType, type_checker::TypeChecker,
    union_type::UnionType,
  },
  type_aliases::{
    l_value::{LValue, LValueMember},
    refinement_map::RefinementMap,
    scope_ptr_type::ScopePtr,
    type_id::TypeId,
    type_id_predicate::TypeIdPredicate,
  },
};
impl TypeChecker {
  /// C++ `void TypeChecker::refineLVale(const LValue&, RefinementMap&, const ScopePtr&,
  /// TypeIdPredicate)` (`Analysis/src/TypeInfer.cpp:6211-6290`)。
  /// predicate 以泛型 + `&mut` 传入：上游 `std::function` 可复制，故能在多个调用点
  /// 反复使用；这里同样复用同一个闭包，不需要 `Rc` 包装再逐个新建转发 box。
  pub fn refine_l_value<P: TypeIdPredicate>(
    &mut self,
    lvalue: &LValue,
    refis: &mut RefinementMap,
    scope: ScopePtr,
    predicate: &mut P,
  ) {
    let mut target: *const LValue = lvalue;
    // If set, we know we took the base of the lvalue path and should be walking down each option of the base's type.
    let mut key: Option<LValue> = None;

    let mut ty = self.resolve_l_value_scope_ptr_l_value(scope.clone(), alias_ref(target));
    if ty.is_none() {
      return; // Do nothing. An error was already reported.
    }

    // If the provided lvalue is a local or global, then that's without a doubt the target.
    // However, if there is a base lvalue, then we'll want that to be the target iff the base is a union type.
    let base = baseof(lvalue);
    if !base.is_null() {
      let base_ty = self.resolve_l_value_scope_ptr_l_value(scope.clone(), alias_ref(base));
      if let Some(base_ty) = base_ty
        && get_type::get::<UnionType>(follow_type::follow(base_ty)).is_some()
      {
        ty = Some(base_ty);
        target = base;
        key = Some(lvalue.clone());
      }
    }

    // If we do not have a key, it means we're not trying to discriminate anything, so it's a simple matter of just filtering for a subset.
    if key.is_none() {
      let (result, _ok) = self.filter_map(
        ty.expect("开头 ty.is_none() 已早返，ty 此后仅被赋 Some(base_ty)，恒为 Some"),
        &mut *predicate,
      );
      add_refinement(
        refis,
        alias_ref(target),
        // 对照 cpp `LUAU_ASSERT(result.first)`：filterMap 空集仅发生于
        // 谓词全滤，本调用树谓词命中至少保留原始类型分支。
        result.expect("cpp LUAU_ASSERT(result.first)：过滤结果非空"),
      );
      return;
    }

    // Otherwise, we'll want to walk each option of ty, get its index type, and filter that.
    let utv = get_type::get::<UnionType>(follow_type::follow(
      ty.expect("开头 ty.is_none() 已早返，ty 恒为 Some"),
    ));
    LUAU_ASSERT!(utv.is_some());

    // Insertion-order dedup (not HashSet<TypeId>): the iteration order of a
    // pointer-keyed HashSet is per-instance randomized, which would make the
    // refined union's option order — and diagnostics derived from it —
    // nondeterministic. Preserving the source union's order keeps it stable.
    let mut viable_target_options: Vec<TypeId> = Vec::new();
    // There may be additional refinements that apply. We add those here too.
    let mut viable_child_options: Vec<TypeId> = Vec::new();

    let key_ref = key
      .as_ref()
      .expect("上方 key.is_none() 分支已 return，此处必为 Some");
    // C++ `for (TypeId option : utv)` — UnionTypeIterator 防环展平并 follow,
    // 裸遍历 options 会漏掉嵌套 union 的分支。
    for option in
      begin_union_type(utv.expect("cpp LUAU_ASSERT(utv)：ty 取自 base 分支时已判定为 UnionType"))
    {
      let discriminant_ty: Option<TypeId>;
      if let Some(field) = <Field as LValueMember>::get_if(key_ref) {
        discriminant_ty = self.get_index_type_from_type(
          scope.clone(),
          option,
          &field.key,
          &Location::default(),
          false,
        );
      } else {
        LUAU_ASSERT!(false); // "Unhandled LValue alternative?"
        discriminant_ty = None;
      }

      let discriminant_ty = match discriminant_ty {
        Some(d) => d,
        None => return, // Do nothing. An error was already reported, as per usual.
      };

      let (result, _ok) = self.filter_map(discriminant_ty, &mut *predicate);
      let result = result.expect("cpp LUAU_ASSERT(result.first)：过滤结果非空");
      if get_type::get::<NeverType>(result).is_none() {
        if !viable_target_options.contains(&option) {
          viable_target_options.push(option);
        }
        if !viable_child_options.contains(&result) {
          viable_child_options.push(result);
        }
      }
    }

    let into_type = |this: &mut TypeChecker, s: &[TypeId]| -> Option<TypeId> {
      if s.is_empty() {
        return None;
      }

      // TODO: allocate UnionType and just normalize.
      let options: Vec<TypeId> = s.to_vec();
      if options.len() == 1 {
        return Some(options[0]);
      }

      // 上游 `addType(UnionType{std::move(options)})`（TypeInfer.cpp:6274）。
      Some(this.add_type(&UnionType { options }))
    };

    if let Some(viable_target_type) = into_type(self, &viable_target_options) {
      add_refinement(refis, alias_ref(target), viable_target_type);
    }

    if let Some(viable_child_type) = into_type(self, &viable_child_options) {
      add_refinement(refis, lvalue, viable_child_type);
    }
  }
}
