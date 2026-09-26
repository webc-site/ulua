use core::{ptr::drop_in_place, slice};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::paged_deallocate::paged_deallocate, records::typed_allocator::TypedAllocator,
};
impl<T> TypedAllocator<T> {
  pub(crate) fn free(&mut self) {
    LUAU_ASSERT!(!self.frozen);

    let last_block = self.stuff.last().copied();

    for &block in &self.stuff {
      let block_size = if Some(block) == last_block {
        self.current_block_size
      } else {
        Self::K_BLOCK_SIZE
      };

      // 单次建切片迭代析构，免逐索引 add 与越界检查
      // Safety: `block` 是 `stuff` 中由 `paged_allocate` 取得、非 null 且对齐到 `T` 的堆块
      // 首地址，整块容量恒为 `K_BLOCK_SIZE` 个 `T`；非末块经 `allocate` 恰好写满 K_BLOCK_SIZE
      // 个、末块只写 `current_block_size` 个，二者都由 `write(res, value)` 逐一初始化过。故
      // `block_size ≤ 已初始化元素数 ≤ 块容量`，切片覆盖的对象全部存活且落在单一分配之内。
      for elem in unsafe { slice::from_raw_parts_mut(block, block_size) } {
        // Safety: 切片每个槽位都是上方论证过的、经 `write` 构造的存活 `T`；逐元素
        // `drop_in_place` 各析构一次（每元素仅被访问一次），块随后整体 `paged_deallocate`，
        // 无未初始化读、也无二次释放。
        unsafe { drop_in_place(elem as *mut T) };
      }

      paged_deallocate(block as *mut u8, Self::K_BLOCK_SIZE_BYTES, self.paged);
    }

    self.stuff.clear();
    self.current_block_size = 0;
  }
}
