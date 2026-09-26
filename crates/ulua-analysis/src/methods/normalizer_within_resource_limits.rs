use ulua_common::fint;

use crate::records::normalizer::Normalizer;

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
