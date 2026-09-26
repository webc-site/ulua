use crate::{
  records::{skip_cache_for_type::SkipCacheForType, unifier::Unifier},
  type_aliases::{collections::HashSet, type_id::TypeId},
};
impl Unifier {
  pub fn unifier_can_cache_result(&mut self, sub_ty: TypeId, super_ty: TypeId) -> bool {
    // shared_state 为 `Handle` 单例句柄（契约见 records/arena_handle.rs）：
    // 指向 TypeCheckResult 持有的共享状态、比本 Unifier 长寿；此处取共享
    // 借用只读 `skip_cache_for_type` 缓存表，单线程无并发写。
    let shared_state = self.shared_state.get();

    if let Some(super_ty_info) = shared_state.skip_cache_for_type.find(&super_ty)
      && *super_ty_info
    {
      return false;
    }

    if let Some(sub_ty_info) = shared_state.skip_cache_for_type.find(&sub_ty)
      && *sub_ty_info
    {
      return false;
    }

    let skip_cache_for = |ty: TypeId| -> bool {
      let mut visitor = SkipCacheForType::skip_cache_for_type_skip_cache_for_type(
        &shared_state.skip_cache_for_type,
        self.types.get().arena_id,
      );
      // C++ `visitor.traverse(ty)` — dispatch to the per-variant visit
      // overrides and recurse into composite types, so any nested mutable
      // element (unsealed/free table, free/bound/generic/blocked pack,
      // etc.) flips `result` and makes the unification uncacheable.
      let mut seen_types = HashSet::new();
      let mut seen_packs = HashSet::new();
      // 变体读取已收口于 `type_variant_of`，traverse 为 safe fn；`ty` 及其可达
      // 子节点均出自构造 visitor 时存入的 `self.types` arena（bump 分块、地址
      // 稳定，canCacheResult 全程不改写类型图）。
      visitor.traverse_skip_cache(ty, &mut seen_types, &mut seen_packs);

      // 重建可变视图——上方共享借用此刻只余 `find` 只读用途（读-改-写序列在
      // 单线程序列化，两个借用窗口不交叠），此借用仅把 visitor 结果记入
      // `skip_cache_for_type[ty]`，随即随闭包返回而丢弃。
      let mut_shared_state = self.shared_state.get_mut();
      mut_shared_state
        .skip_cache_for_type
        .try_insert(ty, visitor.result);
      visitor.result
    };

    if shared_state.skip_cache_for_type.find(&super_ty).is_none() && skip_cache_for(super_ty) {
      return false;
    }

    if shared_state.skip_cache_for_type.find(&sub_ty).is_none() && skip_cache_for(sub_ty) {
      return false;
    }

    true
  }
}
