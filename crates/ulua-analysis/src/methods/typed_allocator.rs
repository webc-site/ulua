//! `typed_allocator` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;
use core::{
  ptr::{drop_in_place, write},
  slice,
};

use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::{
    paged_allocate::paged_allocate, paged_deallocate::paged_deallocate, paged_freeze::paged_freeze,
    paged_unfreeze::paged_unfreeze,
  },
  records::typed_allocator::TypedAllocator,
};

impl<T> TypedAllocator<T> {
  pub fn allocate(&mut self, value: T) -> *mut T {
    LUAU_ASSERT!(!self.frozen);

    if self.current_block_size >= Self::K_BLOCK_SIZE {
      LUAU_ASSERT!(self.current_block_size == Self::K_BLOCK_SIZE);
      self.append_block();
    }

    // 不变式：Default 置 current_block_size=K_BLOCK_SIZE，首次 allocate 必走
    // append_block 播种首块，此后 stuff 只增不减，last() 恒命中 Some。
    let block = *self
      .stuff
      .last()
      .expect("构造即强制 append_block 播种，stuff 恒非空");
    // Safety: block 是 append_block 经 paged_allocate 申请的非空 K_BLOCK_SIZE_BYTES 堆区
    // （恰容纳 K_BLOCK_SIZE 个 T 槽）；上方逻辑保证进入此处时 current_block_size <
    // K_BLOCK_SIZE（达到上限即 append_block 归零并换新区），故 block.add 的偏移落在该对象
    // 界内，结果对齐且指向分配器独占拥有的存活内存。
    let res = unsafe { block.add(self.current_block_size) };
    // Safety: res 是 bump arena 首次发放的槽位，此前未被写入过任何 T（旧内容不存在），
    // 故用 ptr::write 直接落值、无需先 drop；slot 对齐有效且分配器独占（frozen 已断言为否）。
    unsafe {
      write(res, value);
    }
    self.current_block_size += 1;
    res
  }
}

impl<T> TypedAllocator<T> {
  pub(crate) fn append_block(&mut self) {
    // Commit to an allocation strategy on the first block and keep it for the
    // allocator's whole lifetime, so `free` deallocates the way it allocated
    // even if the (ScopedFastFlag) DebugLuauFreezeArena flag is toggled in
    // between. Reading the flag per-call mismatched VirtualFree/operator-delete
    // and corrupted the heap on Windows.
    if self.stuff.is_empty() {
      self.paged = fflag::DebugLuauFreezeArena.get();
    }
    // `None` 即分配失败（原 cpp `operator new(nothrow)` 返回 nullptr），按
    // bad_alloc 语义 panic；成功块折回裸指针入 arena（`stuff` 布局契约不变）。
    let Some(block) = paged_allocate(Self::K_BLOCK_SIZE_BYTES, self.paged) else {
      panic!("std::bad_alloc");
    };

    self.stuff.push(block.as_ptr().cast::<T>());
    self.current_block_size = 0;
  }
}

impl<T> TypedAllocator<T> {
  pub fn clear(&mut self) {
    if self.frozen {
      self.unfreeze();
    }
    self.free();

    self.current_block_size = Self::K_BLOCK_SIZE;
  }
}

impl<T> TypedAllocator<T> {
  pub fn contains(&self, ptr: *const T) -> bool {
    for &block in &self.stuff {
      let block_ptr = block as *const T;
      let block_end = unsafe { block_ptr.add(Self::K_BLOCK_SIZE) };

      if ptr >= block_ptr && ptr < block_end {
        return true;
      }
    }

    false
  }
}

impl<T> TypedAllocator<T> {
  pub fn empty(&self) -> bool {
    self.stuff.is_empty()
  }
}

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

impl<T> TypedAllocator<T> {
  pub fn freeze(&mut self) {
    for &block in &self.stuff {
      paged_freeze(block as *mut u8, Self::K_BLOCK_SIZE_BYTES);
    }
    self.frozen = true;
  }
}

impl<T> TypedAllocator<T> {
  #[inline]
  pub fn is_frozen(&self) -> bool {
    self.frozen
  }
}

impl<T> TypedAllocator<T> {
  pub fn size(&self) -> usize {
    if self.stuff.is_empty() {
      0
    } else {
      Self::K_BLOCK_SIZE * (self.stuff.len() - 1) + self.current_block_size
    }
  }
}

impl<T> TypedAllocator<T> {
  pub fn new() -> Self {
    Self {
      frozen: false,
      stuff: Vec::new(),
      current_block_size: Self::K_BLOCK_SIZE,
      paged: false,
    }
  }
}
impl<T> Drop for TypedAllocator<T> {
  fn drop(&mut self) {
    if self.frozen {
      self.unfreeze();
    }
    self.free();
  }
}

impl<T> TypedAllocator<T> {
  pub fn unfreeze(&mut self) {
    for &block in &self.stuff {
      paged_unfreeze(block as *mut u8, Self::K_BLOCK_SIZE_BYTES);
    }
    self.frozen = false;
  }
}
