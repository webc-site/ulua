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
    apply_mapped_generics::ApplyMappedGenerics, extern_type::ExternType,
    function_type::FunctionType, generic_type::GenericType,
    intersection_builder::IntersectionBuilder, union_builder::UnionBuilder,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

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
  /// `ty` 为类型 arena bump 节点句柄；`self.env`/`self.builtin_types`/`self.arena`
  /// 均为 `Handle`（非空由类型编码，构造期自调用方独占 `&mut` 接线、比本遍历长寿），
  /// 解引用收口在 arena_handle 单点，本函数无 `unsafe`。
  pub(crate) fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
    let bt = self.builtin_types.get();
    // get_mapped_type_bounds 按其契约缺界时以 ice_reporter（Handle 编码非空的存活
    // 报告器）报告并发散；返回引用借用 env 内部表，其后对 env 无其他访问。
    let bounds = self
      .env
      .get_mut()
      .get_mapped_type_bounds(ty, self.ice_reporter);
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
    let env = self.env.get();
    let result = env.lookup_generic_pack(tp);
    if let Some(mapped_gen) = result.get_if::<TypePackId>() {
      return *mapped_gen;
    }
    LUAU_ASSERT!(false);
    // builtin_types 为构造期接线的存活 Handle（NotNull 语义），此处只读常量 pack id。
    self.builtin_types.get().any_type_pack
  }
}

impl ApplyMappedGenerics {
  /// 泛型实参带 mapped-generics 界（或 extern/持久）的类型不向下遍历子节点。
  /// 对应 cpp `ApplyMappedGenerics`（`Analysis/src/Subtyping.cpp:395` 子类，
  /// `ignoreChildrenVisit` 语义即 `Tarjan::ignoreChildrenVisit`
  /// `Substitution.cpp:561` 的定制覆写）。
  ///
  /// 降 safe 说明：`ty` 为 arena `TypeId` 句柄（同 `get_type_id` 门面纪律）；
  /// `self.env` 为 `Handle`（非空由类型编码，构造期自调用方独占 `&mut` 接线），
  /// 解引用收口在 arena_handle 单点，本函数无 `unsafe`。
  pub fn ignore_children_type_id(&mut self, ty: TypeId) -> bool {
    let env = self.env.get();
    if get_type::get::<ExternType>(ty).is_some() {
      return true;
    }
    if let Some(f) = get_type::get::<FunctionType>(ty) {
      for &g in &f.generics {
        let g = follow_type::follow(g);
        if let Some(bounds) = dense_hash_map_find_no_default(&env.current().mapped_generics, &g)
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
    self.env.get().contains_mapped_type(ty)
  }

  pub fn is_dirty_type_pack_id(&mut self, tp: TypePackId) -> bool {
    self.env.get().contains_mapped_pack(tp)
  }
}
