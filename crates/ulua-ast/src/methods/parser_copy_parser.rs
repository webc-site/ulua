use core::{
  mem::size_of,
  ptr::{copy_nonoverlapping, null_mut},
};

use crate::records::{allocator::Allocator, ast_array::AstArray, parser::Parser};

impl Parser {
  pub(crate) fn copy_t_usize<T: Clone>(&mut self, data: *const T, size: usize) -> AstArray<T> {
    let mut result = AstArray {
      data: null_mut(),
      size,
    };

    if size == 0 || data.is_null() {
      return result;
    }

    unsafe {
      let storage = Allocator::allocate(&mut *self.allocator, size_of::<T>() * size) as *mut T;

      result.data = storage;

      // Allocator 返回的存储是新鲜分配，与源区间必不重叠：整段 memcpy
      // 一次到位，省掉逐元素 `write(add(i), clone())` 的循环与逐元素
      // Clone 分发。位拷贝语义与逐个 clone 后再 write 等价（此处 T 均为
      // 指针/POD 数组元素，无自引用不变式）。
      copy_nonoverlapping(data, storage, size);
    }

    result
  }
}
