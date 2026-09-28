//! `normalizer` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use ulua_common::{fint, macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};

use crate::{
  enums::{normalization_result::NormalizationResult, solver_mode::SolverMode},
  functions::{
    begin_type::{begin_intersection_type, begin_union_type},
    follow_type, get_type,
    is_shallow_inhabited::is_shallow_inhabited,
  },
  methods::{
    fresh_normalized_type::fresh_normalized_type,
    normalized_string_type::normalized_string_type_reset_to_string,
    normalizer_combine_type_packs::PackOp,
  },
  records::{
    any_type::AnyType,
    boolean_singleton::BooleanSingleton,
    fuel_initializer::FuelInitializer,
    function_type::FunctionType,
    intersection_type::IntersectionType,
    negation_type::NegationType,
    never_type::NeverType,
    normalized_function_type::NormalizedFunctionType,
    normalized_string_type::NormalizedStringType,
    normalized_type::NormalizedType,
    normalizer::Normalizer,
    normalizer_hit_limits::NormalizerHitLimits,
    primitive_type::{PrimitiveType, Type},
    singleton_type::SingletonType,
    type_ids::TypeIds,
    union_type::UnionType,
    unknown_type::UnknownType,
  },
  type_aliases::{
    error_type::ErrorType, normalized_tyvars::NormalizedTyvars,
    seen_table_prop_pairs::SeenTablePropPairs, type_id::TypeId, type_pack_id::TypePackId,
  },
};

impl Normalizer {
  pub fn cache_type_ids(&mut self, tys: TypeIds) -> *const TypeIds {
    let tys_ptr = &tys as *const TypeIds;
    if let Some(found) = self.cached_type_ids.get(&tys_ptr) {
      return found.as_ref() as *const TypeIds;
    }

    let uniq = Box::new(tys);
    let result = uniq.as_ref() as *const TypeIds;
    self.cached_type_ids.insert(result, uniq);
    result
  }
}

impl Normalizer {
  pub fn clear_caches(&mut self) {
    self.cached_normals.clear();
    self.cached_intersections.clear();
    self.cached_unions.clear();
    self.cached_type_ids.clear();
  }
}

impl Normalizer {
  pub fn clear_fuel(&mut self) {
    self.fuel = None;
  }
}

impl Normalizer {
  pub fn clear_normal(&mut self, norm: &mut NormalizedType) {
    let builtin_types = norm.builtin_types;
    let never_type = builtin_types.get().never_type;

    norm.tops = never_type;
    norm.booleans = never_type;
    norm.extern_types.reset_to_never();
    norm.errors = never_type;
    norm.nils = never_type;
    norm.numbers = never_type;
    norm.integers = never_type;
    norm.strings.reset_to_never();
    norm.threads = never_type;
    norm.buffers = never_type;
    norm.tables.clear();
    norm.functions.reset_to_never();
    norm.tyvars.clear();
  }
}

impl Normalizer {
  pub fn consume_fuel(&mut self) {
    if let Some(fuel) = self.fuel.as_mut() {
      *fuel -= 1;
      if *fuel <= 0 {
        resume_unwind(Box::new(NormalizerHitLimits));
      }
    }
  }
}

impl Normalizer {
  pub fn initialize_fuel(&mut self) -> bool {
    if self.fuel.is_some() {
      return false;
    }

    self.fuel = Some(fint::LuauNormalizerInitialFuel.get());
    true
  }
}

impl Normalizer {
  pub fn intersect_functions(
    &mut self,
    heres: &mut NormalizedFunctionType,
    theres: &NormalizedFunctionType,
  ) {
    self.consume_fuel();

    if heres.is_never() {
    } else if theres.is_never() {
      heres.reset_to_never();
    } else {
      for there in theres.parts.order.iter() {
        let there = *there;
        self.intersect_functions_with_function(heres, there);
      }
    }
  }
}

impl Normalizer {
  pub fn intersect_functions_with_function(
    &mut self,
    heres: &mut NormalizedFunctionType,
    there: TypeId,
  ) {
    self.consume_fuel();

    if heres.is_never() {
      return;
    }

    heres.is_top = false;

    let current_parts = heres.parts.order.clone();
    for here in current_parts {
      let error_ptr = get_type::get::<ErrorType>(here);
      if error_ptr.is_some() {
        continue;
      }

      if let Some(tmp) = self.intersection_of_functions(here, there) {
        heres.parts.erase_type_id(here);
        heres.parts.insert_type_id(tmp);
        return;
      }
    }

    let mut tmps = TypeIds::new();
    for here in &heres.parts.order {
      if let Some(tmp) = self.union_saturated_functions(*here, there) {
        tmps.insert_type_id(tmp);
      }
    }
    heres.parts.insert_type_id(there);
    for ty in tmps.order {
      heres.parts.insert_type_id(ty);
    }
  }
}

impl Normalizer {
  pub fn intersect_normal_with_negation_ty(
    &mut self,
    to_negate: TypeId,
    intersect: &mut NormalizedType,
  ) -> NormalizationResult {
    self.consume_fuel();

    // C++ 中归一化失败返回空指针：视为不可驻留，返回 False
    let Some(normal) = self.try_normalize(to_negate) else {
      return NormalizationResult::False;
    };
    let negated = self.negate_normal(&normal);

    match negated {
      Some(negated_type) => self.intersect_normals(intersect, &negated_type, 0),
      None => NormalizationResult::False,
    }
  }
}

impl Normalizer {
  pub fn intersect_tables(&mut self, heres: &mut TypeIds, theres: &TypeIds) {
    self.consume_fuel();

    let mut tmp = TypeIds::new();
    for &here in &heres.order {
      for &there in &theres.order {
        if let Some(inter) = self.intersection_of_tables(here, there) {
          tmp.insert_type_id(inter);
        }
      }
    }

    heres.retain(&tmp);
    for ty in tmp.order {
      heres.insert_type_id(ty);
    }
  }
}

impl Normalizer {
  pub fn intersect_tables_with_table(
    &mut self,
    heres: &mut TypeIds,
    there: TypeId,
    _seen_table_prop_pairss: &mut SeenTablePropPairs,
    _seen_set_typeses: &mut DenseHashSet<TypeId>,
  ) {
    self.consume_fuel();

    let mut tmp = TypeIds::new();
    let heres_clone = heres.clone();
    for here in heres_clone.order {
      if let Some(inter) = self.intersection_of_tables(here, there) {
        tmp.insert_type_id(inter);
      }
    }
    heres.retain(&tmp);
    for ty in tmp.order {
      heres.insert_type_id(ty);
    }
  }
}

impl Normalizer {
  pub fn intersect_tyvars_with_ty(
    &mut self,
    here: &mut NormalizedTyvars,
    there: TypeId,
    seen_table_prop_pairs: &mut SeenTablePropPairs,
    seen_set_types: &mut DenseHashSet<TypeId>,
  ) -> NormalizationResult {
    self.consume_fuel();

    let mut it = here.iter_mut();
    while let Some((_, inter_box)) = it.next() {
      let inter = inter_box.as_mut();
      let res = self.intersect_normal_with_ty(inter, there, seen_table_prop_pairs, seen_set_types);
      if res != NormalizationResult::True {
        return res;
      }
      if is_shallow_inhabited(inter) {
        // keep this entry
      } else {
        // remove this entry
        it = here.iter_mut();
        continue;
      }
    }
    NormalizationResult::True
  }
}

impl Normalizer {
  pub fn intersection_of_bools(&mut self, here: TypeId, there: TypeId) -> TypeId {
    self.consume_fuel();

    if get_type::get::<NeverType>(here).is_some() {
      return here;
    }
    if get_type::get::<NeverType>(there).is_some() {
      return there;
    }

    if let Some(hbool) = get_type::get::<SingletonType>(here)
      .as_ref()
      .and_then(|s| s.variant.get_if::<BooleanSingleton>())
    {
      if let Some(tbool) = get_type::get::<SingletonType>(there)
        .as_ref()
        .and_then(|s| s.variant.get_if::<BooleanSingleton>())
      {
        if hbool.value == tbool.value {
          return here;
        } else {
          return self.builtin_types.get().never_type;
        }
      }

      return here;
    }

    there
  }
}

impl Normalizer {
  pub fn intersection_of_functions(&mut self, here: TypeId, there: TypeId) -> Option<TypeId> {
    self.consume_fuel();

    let hftv = get_type::get::<FunctionType>(here)?;
    let tftv = get_type::get::<FunctionType>(there)?;

    if hftv.generics != tftv.generics {
      return None;
    }
    if hftv.generic_packs != tftv.generic_packs {
      return None;
    }

    let (arg_types, ret_types) = if hftv.ret_types == tftv.ret_types {
      let arg_types = self.union_of_type_packs(hftv.arg_types, tftv.arg_types)?;
      (arg_types, hftv.ret_types)
    } else if hftv.arg_types == tftv.arg_types {
      let ret_types = self.intersection_of_type_packs_internal(hftv.arg_types, tftv.arg_types)?;
      (hftv.arg_types, ret_types)
    } else {
      return None;
    };

    if arg_types == hftv.arg_types && ret_types == hftv.ret_types {
      return Some(here);
    }
    if arg_types == tftv.arg_types && ret_types == tftv.ret_types {
      return Some(there);
    }

    let mut result = FunctionType::function_type_new(arg_types, ret_types, None, false);
    result.generics = hftv.generics.clone();
    result.generic_packs = hftv.generic_packs.clone();

    // 契约：归一化期 arena 已接线（wired_arena_mut 断言），单线程驱动无并存别名。
    Some(self.wired_arena_mut().add_type(result))
  }
}

impl Normalizer {
  pub fn intersection_of_tops(&mut self, here: TypeId, there: TypeId) -> TypeId {
    self.consume_fuel();

    let here_is_never = get_type::get::<NeverType>(here).is_some();
    let there_is_never = get_type::get::<NeverType>(there).is_some();
    let here_is_any = get_type::get::<AnyType>(here).is_some();
    let there_is_any = get_type::get::<AnyType>(there).is_some();

    if here_is_never || there_is_never {
      return self.builtin_types.get().never_type;
    }

    if here_is_any || there_is_any {
      return self.builtin_types.get().any_type;
    }

    self.builtin_types.get().unknown_type
  }
}

impl Normalizer {
  pub fn intersection_of_type_packs(
    &mut self,
    here: TypePackId,
    there: TypePackId,
  ) -> Option<TypePackId> {
    // 对齐 cpp `FuelInitializer fi{NotNull{this}}`：构造即 initialize_fuel，
    // 命中资源上限 resume_unwind 时由 Drop 清理 fuel。
    let mut fi = FuelInitializer {
      normalizer: self as *mut Normalizer,
      initialized_fuel: false,
    };
    unsafe { fi.fuel_initializer_not_null_normalizer(self as *mut Normalizer) };

    // 对齐 cpp try/catch：仅捕获 NormalizerHitLimits 并返回 None，其余 panic 继续传播。
    match catch_unwind(AssertUnwindSafe(|| {
      self.intersection_of_type_packs_internal(here, there)
    })) {
      Ok(result) => result,
      Err(payload) if payload.downcast_ref::<NormalizerHitLimits>().is_some() => None,
      Err(payload) => resume_unwind(payload),
    }
  }
}

// `Normalizer::intersectionOfTypePacks_INTERNAL`（Normalize.cpp:3319 之后）
// ——同形骨架见 [`crate::methods::normalizer_combine_type_packs`]。

impl Normalizer {
  pub fn intersection_of_type_packs_internal(
    &mut self,
    here: TypePackId,
    there: TypePackId,
  ) -> Option<TypePackId> {
    self.combine_type_packs(PackOp::Meet, here, there)
  }
}

impl Normalizer {
  pub fn negate(&mut self, mut there: TypeId) -> TypeId {
    self.consume_fuel();

    there = follow_type::follow(there);

    if get_type::get::<AnyType>(there).is_some() {
      there
    } else if get_type::get::<UnknownType>(there).is_some() {
      // Safety: self.builtin_types.as_ptr() 是构造期接线的非空 BuiltinTypes 会话单例
      // （C++ NotNull），只读拷贝 never_type 句柄；there 为存活归一句柄。
      self.builtin_types.get().never_type
    } else if get_type::get::<NeverType>(there).is_some() {
      // Safety: 同上——非空单例只读拷贝 unknown_type 句柄。
      self.builtin_types.get().unknown_type
    } else if let Some(ntv) = get_type::get::<NegationType>(there) {
      ntv.ty
    } else if let Some(utv) = get_type::get::<UnionType>(there) {
      let mut parts = Vec::new();
      // C++ `for (TypeId option : utv)`——UnionTypeIterator 展平嵌套 union
      // 并 follow,裸遍历 options 会漏掉嵌套成员。
      for option in begin_union_type(utv) {
        parts.push(self.negate(option));
      }
      // 契约：self.arena 构造/接线期已注入且活过本次 negate；utv 的共享借用经
      // begin_union_type 消费完后不再使用，递归内对 arena 的可变借用窗口均已
      // 随各自返回结束——wired_arena_mut 的 &mut 再借用为串行窗口，无并存别名；
      // TypedAllocator 只追加，节点地址稳定，已有句柄不失效。
      self.wired_arena_mut().add_type(IntersectionType { parts })
    } else if let Some(itv) = get_type::get::<IntersectionType>(there) {
      let mut options = Vec::new();
      // C++ `for (TypeId part : itv)`——IntersectionTypeIterator 同理。
      for part in begin_intersection_type(itv) {
        options.push(self.negate(part));
      }
      // Safety: 与 union 分支对称——arena 经 wired_arena_mut 断言已接线，itv
      // 借用与递归的可变窗口串行无并存，add_type 仅追加地址稳定节点。
      self.wired_arena_mut().add_type(UnionType { options })
    } else {
      there
    }
  }
}

impl Normalizer {
  pub fn normalize_intersections(
    &mut self,
    intersections: &Vec<TypeId>,
    out_type: &mut NormalizedType,
    seen_table_prop_pairs: &mut SeenTablePropPairs,
    seen_set: &mut DenseHashSet<TypeId>,
  ) -> NormalizationResult {
    if self.arena.is_none() {
      // C++: sharedState->iceHandler->ice("Normalizing types outside a module")
      // 契约：本分支仅在 arena 哨兵为 None（模块外归一化）时触发；`self.shared_state`
      // 为构造/接线期注入的 Option<Handle<UnifierSharedState>>（C++ `sharedState`
      // 成员直译，存活整次归一化，shared_state_ref 断言接线），其 `ice_handler`
      // 字段由 UnifierSharedState 构造期写入 Frontend 持有的 InternalErrorReporter，
      // 同样非空存活；`ice_string` 只读上报，无别名冲突。
      let ice = self.shared_state_ref().ice_handler;
      unsafe {
        (*ice).ice_string("Normalizing types outside a module");
      }
    }

    self.consume_fuel();

    // NormalizedType norm{builtinTypes}; norm.tops = builtinTypes->unknownType;
    let mut norm = fresh_normalized_type(self.builtin_types);
    // Safety: `self.builtin_types.as_ptr()` 是构造期接线的非空 BuiltinTypes 会话单例（C++ NotNull），
    // 此处只读拷贝 unknown_type 句柄。
    norm.tops = self.builtin_types.get().unknown_type;

    for &ty in intersections {
      let res = self.intersect_normal_with_ty(&mut norm, ty, seen_table_prop_pairs, seen_set);
      if res != NormalizationResult::True {
        return res;
      }
    }

    let res = self.union_normals(out_type, &norm, -1);
    if res != NormalizationResult::True {
      return res;
    }

    NormalizationResult::True
  }
}

impl Normalizer {
  pub fn subtract_primitive(&mut self, here: &mut NormalizedType, ty: TypeId) {
    self.consume_fuel();

    let ty_followed = follow_type::follow(ty);
    // 唯一调用方 `intersect_normal_with_ty` 先以 `get_type::get::<PrimitiveType>(follow(t)).is_some()`
    // 甄别分派才进入本函数（cpp 按类型 tag 分派后直接 get 的同前提），下转必命中。
    let ptv = get_type::get::<PrimitiveType>(ty_followed)
      .expect("调用方已按 PrimitiveType 甄别后分派，下转必命中");

    let builtin_types = here.builtin_types;
    // Safety: here.builtin_types 是 Normalizer/NormalizedType 构造期接线的非空
    // BuiltinTypes 会话单例（C++ NotNull<BuiltinTypes>），比本次归一化长寿且此
    // 后只读不变；借用提到函数头一次取得，替代分支内 6 处重复 `&*` 解引用。
    let builtin = builtin_types.get();
    match ptv.r#type {
      Type::NilType => {
        here.nils = builtin.never_type;
      }
      Type::Boolean => {
        here.booleans = builtin.never_type;
      }
      Type::Number => {
        here.numbers = builtin.never_type;
      }
      Type::Integer => {
        here.integers = builtin.never_type;
      }
      Type::String => {
        here.strings.reset_to_never();
      }
      Type::Thread => {
        here.threads = builtin.never_type;
      }
      Type::Buffer => {
        here.buffers = builtin.never_type;
      }
      Type::Function => {
        here.functions.reset_to_never();
      }
      Type::Table => {
        here.tables.clear();
      }
    }
  }
}

impl Normalizer {
  pub fn union_functions(
    &mut self,
    heres: &mut NormalizedFunctionType,
    theres: &NormalizedFunctionType,
  ) {
    self.consume_fuel();

    if heres.is_top {
      return;
    }
    if theres.is_top {
      heres.reset_to_top();
    }

    if theres.is_never() {
      return;
    }

    let mut tmps = TypeIds::new();

    if heres.is_never() {
      tmps = theres.parts.clone();
      heres.parts = tmps;
      return;
    }

    let heres_parts = heres.parts.clone();
    let theres_parts = theres.parts.clone();

    for here in heres_parts.order {
      for there in theres_parts.order.iter() {
        let there = *there;
        if let Some(fun) = self.union_of_functions(here, there) {
          tmps.insert_type_id(fun);
        } else {
          let builtin_types = self.builtin_types.get();
          tmps.insert_type_id(builtin_types.error_recovery_type(there));
        }
      }
    }

    heres.parts = tmps;
  }
}

impl Normalizer {
  pub fn union_functions_with_function(
    &mut self,
    heres: &mut NormalizedFunctionType,
    there: TypeId,
  ) {
    self.consume_fuel();

    if heres.is_never() {
      let mut tmps = TypeIds::new();
      tmps.insert_type_id(there);
      heres.parts = tmps;
      return;
    }

    let mut tmps = TypeIds::new();
    let parts = heres.parts.clone();
    for here in parts.order {
      if let Some(fun) = self.union_of_functions(here, there) {
        tmps.insert_type_id(fun);
      } else {
        let builtin_types = self.builtin_types.get();
        tmps.insert_type_id(builtin_types.error_recovery_type(there));
      }
    }
    heres.parts = tmps;
  }
}

impl Normalizer {
  pub fn union_of_bools(&mut self, here: TypeId, there: TypeId) -> TypeId {
    self.consume_fuel();

    if get_type::get::<NeverType>(here).is_some() {
      return there;
    }
    if get_type::get::<NeverType>(there).is_some() {
      return here;
    }

    if let Some(hbool) = get_type::get::<SingletonType>(here)
      .as_ref()
      .and_then(|s| s.variant.get_if::<BooleanSingleton>())
      && let Some(tbool) = get_type::get::<SingletonType>(there)
        .as_ref()
        .and_then(|s| s.variant.get_if::<BooleanSingleton>())
      && hbool.value == tbool.value
    {
      return here;
    }

    self.builtin_types.get().boolean_type
  }
}

impl Normalizer {
  pub fn union_of_functions(&mut self, here: TypeId, there: TypeId) -> Option<TypeId> {
    self.consume_fuel();

    if get_type::get::<ErrorType>(here).is_some() {
      return Some(here);
    }

    if get_type::get::<ErrorType>(there).is_some() {
      return Some(there);
    }

    // cpp 侧 unionOfFunctions 入口已由变体判定保证双方为 FunctionType（LUAU_ASSERT 同义）。
    let hftv = get_type::get::<FunctionType>(here).expect("调用方已证 here 为 FunctionType 变体");
    let tftv = get_type::get::<FunctionType>(there).expect("调用方已证 there 为 FunctionType 变体");

    let h_generics = hftv.generics.clone();
    let t_generics = tftv.generics.clone();
    if h_generics != t_generics {
      return None;
    }

    let h_generic_packs = hftv.generic_packs.clone();
    let t_generic_packs = tftv.generic_packs.clone();
    if h_generic_packs != t_generic_packs {
      return None;
    }

    let arg_types = self.intersection_of_type_packs_internal(hftv.arg_types, tftv.arg_types);
    arg_types?;

    let ret_types = self.union_of_type_packs(hftv.ret_types, tftv.ret_types);
    ret_types?;

    let arg_types_val = arg_types.expect("上方 `arg_types?` 早退已排除 None");
    let ret_types_val = ret_types.expect("上方 `ret_types?` 早退已排除 None");

    if arg_types_val == hftv.arg_types && ret_types_val == hftv.ret_types {
      return Some(here);
    }

    if arg_types_val == tftv.arg_types && ret_types_val == tftv.ret_types {
      return Some(there);
    }

    let mut result = FunctionType::function_type_new(arg_types_val, ret_types_val, None, false);
    result.generics = h_generics;
    result.generic_packs = h_generic_packs;

    // 契约：归一化期 arena 已接线（wired_arena_mut 断言），单线程驱动无并存别名。
    Some(self.wired_arena_mut().add_type(result))
  }
}

impl Normalizer {
  pub fn union_of_tops(&mut self, here: TypeId, there: TypeId) -> TypeId {
    self.consume_fuel();

    if get_type::get::<NeverType>(here).is_some() || get_type::get::<AnyType>(there).is_some() {
      return there;
    }

    here
  }
}

// `Normalizer::unionOfTypePacks`（Normalize.cpp:1796）——同形骨架见
// [`crate::methods::normalizer_combine_type_packs`]。

impl Normalizer {
  pub fn union_of_type_packs(&mut self, here: TypePackId, there: TypePackId) -> Option<TypePackId> {
    self.combine_type_packs(PackOp::Join, here, there)
  }
}

impl Normalizer {
  pub fn union_saturated_functions(&mut self, here: TypeId, there: TypeId) -> Option<TypeId> {
    self.consume_fuel();

    let hftv = get_type::get::<FunctionType>(here)?;

    let tftv = get_type::get::<FunctionType>(there)?;

    if hftv.generics != tftv.generics {
      return None;
    }

    if hftv.generic_packs != tftv.generic_packs {
      return None;
    }

    let arg_types = self.union_of_type_packs(hftv.arg_types, tftv.arg_types)?;
    let ret_types = self.union_of_type_packs(hftv.ret_types, tftv.ret_types)?;

    let mut result = FunctionType::function_type_new(arg_types, ret_types, None, false);
    result.generics = hftv.generics.clone();
    result.generic_packs = hftv.generic_packs.clone();

    // 契约：归一化期 arena 已接线（wired_arena_mut 断言），单线程驱动无并存别名。
    Some(self.wired_arena_mut().add_type(result))
  }
}

impl Normalizer {
  pub fn union_strings(&mut self, here: &mut NormalizedStringType, there: &NormalizedStringType) {
    self.consume_fuel();

    if there.is_string() {
      normalized_string_type_reset_to_string(here);
    } else if here.is_union() && there.is_union() {
      for (name, ty) in &there.singletons {
        here.singletons.insert(name.clone(), *ty);
      }
    } else if here.is_union() && there.is_intersection() {
      here.is_cofinite = true;
      for (name, ty) in &there.singletons {
        if let Some(it) = here.singletons.remove(name) {
          let _ = it;
        } else {
          here.singletons.insert(name.clone(), *ty);
        }
      }
    } else if here.is_intersection() && there.is_union() {
      for name in there.singletons.keys() {
        here.singletons.remove(name);
      }
    } else if here.is_intersection() && there.is_intersection() {
      let mut keys_to_remove = Vec::new();
      for name in here.singletons.keys() {
        if !there.singletons.contains_key(name) {
          keys_to_remove.push(name.clone());
        }
      }
      for name in keys_to_remove {
        here.singletons.remove(&name);
      }
    } else {
      LUAU_ASSERT!(false);
    }
  }
}

impl Normalizer {
  pub fn union_tables(&mut self, heres: &mut TypeIds, theres: &TypeIds) {
    self.consume_fuel();

    for there in theres.order.iter() {
      let there = *there;
      let builtin_types = self.builtin_types.get();
      if there == builtin_types.table_type {
        heres.clear();
        heres.insert_type_id(there);
        return;
      } else {
        self.union_tables_with_table(heres, there);
      }
    }
  }
}

impl Normalizer {
  pub fn union_tables_with_table(&mut self, heres: &mut TypeIds, there: TypeId) {
    // we can always skip `never`
    let never_ptr = get_type::get::<NeverType>(there);
    if never_ptr.is_some() {
      return;
    }

    heres.insert_type_id(there);
  }
}

impl Normalizer {
  pub fn use_new_luau_solver(&self) -> bool {
    self.solver_mode == SolverMode::New
  }
}

impl Normalizer {
  pub fn within_resource_limits(&mut self) -> bool {
    // If cache is too large, clear it
    if fint::LuauNormalizeCacheLimit.get() > 0 {
      let cache_usage = self.cached_normals.len()
        + self.cached_intersections.len()
        + self.cached_unions.len()
        + self.cached_type_ids.len()
        + self.cached_is_inhabited.size()
        + self.cached_is_inhabited_intersection.size();
      if cache_usage > fint::LuauNormalizeCacheLimit.get() as usize {
        self.clear_caches();
        return false;
      }
    }

    // 检查递归计数
    // 契约：shared_state 是构造/接线期注入的 Option<Handle<UnifierSharedState>>
    //（对应 C++ `UnifierSharedState&` 引用成员，两段接线完成后恒非空），
    // shared_state_ref 断言接线；此处仅重建一次短程共享借用读取两个计数器
    // 字段，与原式逐字等价；读写 counters 的借用只发生在同线程其他调用点，
    // 本借用不跨越任何函数调用，不与它们重叠。
    let shared_state = self.shared_state_ref();
    !(shared_state.counters.recursion_limit > 0
      && shared_state.counters.recursion_limit < shared_state.counters.recursion_count)
  }
}
