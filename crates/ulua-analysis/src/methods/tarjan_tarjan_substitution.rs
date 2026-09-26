use ulua_common::fint;

use crate::records::tarjan::Tarjan;

impl Tarjan {
  pub fn tarjan(&mut self) {
    // 对齐 cpp Substitution.cpp:10,161-171：预分配容量读取可调的
    // FInt::LuauTarjanPreallocationSize（默认 256），恢复其运行时可调语义；
    // cpp 断言桶数须为 2 的幂，此处向上取整到最近的 2 的幂以兼容任意取值。
    let raw = fint::LuauTarjanPreallocationSize.get().max(0) as usize;
    let preallocation_size = if raw == 0 { 0 } else { raw.next_power_of_two() };

    self.nodes.reserve(preallocation_size);
    self.stack.reserve(preallocation_size);
    self.edges_ty.reserve(preallocation_size);
    self.edges_tp.reserve(preallocation_size);
    self.worklist.reserve(preallocation_size);

    // cpp 在 Tarjan 构造时以同一 FInt 预分配 typeToIndex/packToIndex 两个
    // DenseHashMap 的桶；Rust 侧 map 先于本构造体创建，这里对两个空索引
    // map 做同容量的桶预留。
    self.type_to_index.reserve_buckets(preallocation_size);
    self.pack_to_index.reserve_buckets(preallocation_size);
  }
}
