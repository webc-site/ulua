use core::ptr::write;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::typed_allocator::TypedAllocator;
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
