use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::get_type,
  records::{
    apply_mapped_generics::ApplyMappedGenerics, generic_type::GenericType,
    intersection_builder::IntersectionBuilder, union_builder::UnionBuilder,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl ApplyMappedGenerics {
  /// # Safety
  /// `ty` 为类型 arena bump 节点句柄；本函数经 `self.env`/`self.builtin_types`/`self.arena`
  /// 三个裸指针（由 apply_mapped_generics 每轮从调用方独占借用的 `&mut` 接线，比本遍历长寿）
  /// 解引用读取。调用方须保证这些指针非空、对齐且在此遍历期内不被其他可变借用。单线程独占。对应
  /// C++ `TypeId ApplyMappedGenerics::clean(TypeId ty)` (`cpp/Analysis/src/Subtyping.cpp:427`)。
  pub unsafe fn clean_type_id(&mut self, ty: TypeId) -> TypeId {
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
