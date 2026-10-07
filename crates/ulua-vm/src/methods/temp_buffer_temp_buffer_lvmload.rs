use core::{mem::size_of, ptr::null_mut};

use crate::{functions::lua_m_free::lua_m_free, records::temp_buffer::TempBuffer};

impl<T> Default for TempBuffer<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T> TempBuffer<T> {
  /// 空缓冲构造，逐字节复刻 cpp `TempBuffer::TempBuffer() : L(NULL), data(NULL), count(0)`
  /// （lvmload.cpp:45-49）。§2 判定：
  /// - `l`：null = 「尚未 `allocate`」的缺席语义（`allocate` 入口 `LUAU_ASSERT!(self.l.is_null())`
  ///   正是 cpp `this->L == nullptr` 的不变式判空，规则 3）。理想形态 `Option<NonNull<LuaState>>`；
  ///   但写点在 `temp_buffer_allocate.rs`（`self.l = l`、`.is_null()`），就地改型破坏其编译。
  /// - `data`：既是「空缓冲」缺席（`Drop` 的 `if !self.data.is_null()` 复刻 cpp `if (data)` 释放守卫，
  ///   规则 3），又是 `operator_index.rs` 里 `self.data.add(index)` 的指针算式基址（规则 2）。理想
  ///   缺席侧可落 `Option<NonNull<T>>`，但算式基址与 `allocate`/`operator_index` 的裸指针读写均在
  ///   并行会话文件，就地改型破坏其编译。
  ///
  /// 综上本轮保留 `*mut LuaState` / `*mut T`，把 null 判定收拢到本 `Drop` 与 `allocate` 单点，
  /// 并记录待协调的 `Option<NonNull<_>>` 迁移点，非机械照抄 cpp。
  pub fn new() -> Self {
    Self {
      l: null_mut(),
      data: null_mut(),
      count: 0,
    }
  }
}

impl<T> Drop for TempBuffer<T> {
  fn drop(&mut self) {
    if !self.data.is_null() {
      // SAFETY: data 只会来自 allocate（经 l 的分配器按 count*size_of::<T>() 申请且未被别处释放），
      // 以同一 l、同一字节数归还给 lua_m_free
      unsafe { lua_m_free(self.l, self.data as *mut u8, self.count * size_of::<T>(), 0) };
      self.data = null_mut();
      self.count = 0;
    }
  }
}
