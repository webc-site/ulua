//! `apply_mapped_generics` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    clone_clone::{pack_is_persistent, type_is_persistent},
    follow_type, get_type,
  },
  macros::{substitution_entry::substitution_entry, substitution_vtable},
  methods::subtyping_bind_generic::dense_hash_map_find_no_default,
  records::{
    apply_mapped_generics::ApplyMappedGenerics, arena_handle::Handle, builtin_types::BuiltinTypes,
    extern_type::ExternType, function_type::FunctionType, generic_type::GenericType,
    internal_error_reporter::InternalErrorReporter, intersection_builder::IntersectionBuilder,
    subtyping_environment::SubtypingEnvironment, type_arena::TypeArena,
    union_builder::UnionBuilder,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl ApplyMappedGenerics {
  pub fn apply_mapped_generics(
    &mut self,
    builtin_types: Handle<BuiltinTypes>,
    arena: Handle<TypeArena>,
    env: &mut SubtypingEnvironment,
    ice_reporter: *mut InternalErrorReporter,
  ) {
    self.builtin_types = builtin_types;
    self.arena = arena;
    self.env = env as *mut _;
    self.ice_reporter = ice_reporter;
  }
}
// C++ `ApplyMappedGenerics` overrides `isDirty` / `clean` / `ignoreChildren`
// (it does NOT override `ignoreChildrenVisit`, so that defaults to
// `ignoreChildren`). The inherited `substitute` traversal dispatches into these
// at runtime. `found_dirty` is `Substitution`'s own (non-overridden) method, so
// its thunk forwards to `self.base.found_dirty_*`. TypePackId 侧三覆写槽全为真实
// 转发（见 substitution_vtable 模块文档的统一安全论证）。
substitution_vtable!(real, ApplyMappedGenerics, ic = ignore_children_type_id);
impl ApplyMappedGenerics {
  substitution_entry!(id, pack);
}

impl ApplyMappedGenerics {
  /// # Safety
  /// `ty` 为类型 arena bump 节点句柄；本函数经 `self.env`/`self.builtin_types`/`self.arena`
  /// 三个裸指针（由 apply_mapped_generics 每轮从调用方独占借用的 `&mut` 接线，比本遍历长寿）
  /// 解引用读取。调用方须保证这些指针非空、对齐且在此遍历期内不被其他可变借用。单线程独占。对应
  pub fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
    // Safety: builtin_types 由 apply_mapped_generics 在每轮映射前从调用方存活的
    // &mut BuiltinTypes（C++ NotNull 语义）接线为非空裸指针，比本遍历长寿；取共享
    // 引用仅读常量 TypeId，与 env 指向的对象不重叠。
    let bt = self.builtin_types.get();
    // Safety: self.env 同样每轮接线自调用方独占借用的 &mut SubtypingEnvironment，
    // 指向存活对象；ty 为类型 arena bump 节点指针，get_mapped_type_bounds 按其
    // 契约缺界时以 ice_reporter（NotNull 接线的存活报告器）报告，返回引用借用 env
    // 内部表，本块之后 env 无其他访问。
    let bounds = unsafe { (*self.env).get_mapped_type_bounds(ty, self.ice_reporter) };
    let lower_bound = &bounds.lower_bound;
    let upper_bound = &bounds.upper_bound;

    if upper_bound.empty() && lower_bound.empty() {
      // No bounds for the generic we're mapping.
      // In this case, unknown vs never is an arbitrary choice:
      // ie, does it matter if we map add<A, A> to add<unknown, unknown> or add<never, never> in the context of subtyping?
      // We choose unknown here, since it's closest to the original behavior.
      bt.unknown_type
    } else if !upper_bound.empty() {
      let mut ib = IntersectionBuilder::new(self.arena, self.builtin_types);
      for &ub in &upper_bound.order {
        // NOTE: The original implementation skips over generic
        // types, but that seems incorrect to me.
        if get_type::get::<GenericType>(ub).is_none() {
          ib.add(ub);
        }
      }
      ib.build()
    } else if !lower_bound.empty() {
      let mut ub_builder = UnionBuilder::new(self.arena, self.builtin_types);
      for &lb in &lower_bound.order {
        // NOTE: The original implementation skips over generic
        // types, but that seems incorrect to me.
        if get_type::get::<GenericType>(lb).is_none() {
          ub_builder.add(lb);
        }
      }
      ub_builder.build()
    } else {
      LUAU_ASSERT!(false);
      bt.unknown_type
    }
  }

  pub fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    // Safety: self.env 为本轮映射接线自调用方 &mut SubtypingEnvironment 的非空裸
    // 指针（见 apply_mapped_generics 构造），存活至本函数末，仅只读查询 pack 映射。
    let env = unsafe { &*self.env };
    let result = env.lookup_generic_pack(tp);
    if let Some(mapped_gen) = result.get_if::<TypePackId>() {
      return *mapped_gen;
    }
    LUAU_ASSERT!(false);
    // Safety: builtin_types 每轮接线自存活的 &mut BuiltinTypes（NotNull 语义），
    // 此处只读常量 pack id。
    self.builtin_types.get().any_type_pack
  }
}

impl ApplyMappedGenerics {
  /// 泛型实参带 mapped-generics 界（或 extern/持久）的类型不向下遍历子节点。
  /// 对应 cpp `ApplyMappedGenerics`（`Analysis/src/Subtyping.cpp:395` 子类，
  /// `ignoreChildrenVisit` 语义即 `Tarjan::ignoreChildrenVisit`
  /// `Substitution.cpp:561` 的定制覆写）。
  ///
  /// 降 safe 说明：`ty` 为 arena `TypeId` 句柄（同 `get_type_id` 门面纪律），
  /// 对 `self.env`（构造期 `NotNull<SubtypingEnvironment>` 接线、比本对象长寿）
  /// 与 `(*ty).persistent` 的解引用收进窄 `unsafe` 块。
  pub fn ignore_children_type_id(&mut self, ty: TypeId) -> bool {
    // Safety: self.env 构造期以 NotNull{&env} 接线，非空且比本次替换长寿；
    // 只读查询 mapped_generics 帧，无并存可变借用。
    let env = unsafe { &*self.env };
    if get_type::get::<ExternType>(ty).is_some() {
      return true;
    }
    if let Some(f) = get_type::get::<FunctionType>(ty) {
      for &g in &f.generics {
        let g = follow_type::follow(g);
        if let Some(bounds) = dense_hash_map_find_no_default(&env.mapped_generics, &g)
          && !bounds.is_empty()
        {
          return true;
        }
      }
    }
    // type_is_persistent 探针（clone_clone）内部窄块已证成 arena 句柄前提。
    type_is_persistent(ty)
  }

  /// 类型包孪生版；降 safe 理由同上。cpp 锚点 `Substitution.cpp:566`。
  pub fn ignore_children_type_pack_id(&mut self, ty: TypePackId) -> bool {
    // 同上：pack_is_persistent 探针收口。
    pack_is_persistent(ty)
  }
}

impl ApplyMappedGenerics {
  pub fn is_dirty_type_id(&mut self, ty: TypeId) -> bool {
    unsafe { (*self.env).contains_mapped_type(ty) }
  }

  pub fn is_dirty_type_pack_id(&mut self, tp: TypePackId) -> bool {
    unsafe { (*self.env).contains_mapped_pack(tp) }
  }
}
