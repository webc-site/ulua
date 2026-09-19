use core::ffi::c_char;

use ulua_common::enums::luau_bytecode_type::LBC_TYPE_TAGGED_USERDATA_BASE;

use crate::functions::{c_slice_mut, read_var_int::read_var_int};

/// `[offset, offset + len)` 是否完整落在缓冲内。
fn fits(offset: usize, len: usize, size: usize) -> bool {
  offset.checked_add(len).is_some_and(|end| end <= size)
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
/// 单字节类型重映射：带符号差转 u32 索引，命中重映射表才写回
#[inline]
unsafe fn remap_byte(t: &mut u8, userdata_remapping: *const u8, count: u32) {
  let index = (*t as i32 - LBC_TYPE_TAGGED_USERDATA_BASE.0 as i32) as u32;

  if index < count {
    unsafe { *t = *userdata_remapping.add(index as usize) };
  }
}

/// cpp `remapUserdataTypes`（lvmload.cpp:220-280）：typeinfo 三段布局的
/// userdata 类型重映射。
///
/// 与 cpp 的差异：cpp 的段长度全部来自不可信 varint，只靠末尾
/// `LUAU_ASSERT(offset == size)` 兜底（release 编译掉就是越界读写），Rust 侧
/// 每段先硬校验再取，任一环节不自洽即返回 `false`，由 `loadsafe` 转成
/// 「损坏字节码」错误。
///
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn remap_userdata_types(
  data: *mut c_char,
  size: usize,
  userdata_remapping: *mut u8,
  count: u32,
) -> bool {
  unsafe {
    let mut offset: usize = 0;

    let (Some(type_size), Some(upval_count), Some(local_count)) = (
      read_var_int(data, size, &mut offset),
      read_var_int(data, size, &mut offset),
      read_var_int(data, size, &mut offset),
    ) else {
      return false;
    };

    if type_size != 0 {
      // types[2..type_size] 的读写都必须在缓冲内
      if !fits(offset, type_size as usize, size) {
        return false;
      }

      // Skip two bytes of function type introduction
      if type_size > 2 {
        let types = data.add(offset) as *mut u8;
        let body = c_slice_mut(types.add(2), type_size as usize - 2);

        for t in body {
          remap_byte(t, userdata_remapping, count);
        }
      }

      offset += type_size as usize;
    }

    if upval_count != 0 {
      if !fits(offset, upval_count as usize, size) {
        return false;
      }

      let types = data.add(offset) as *mut u8;

      for t in c_slice_mut(types, upval_count as usize) {
        remap_byte(t, userdata_remapping, count);
      }

      offset += upval_count as usize;
    }

    for _ in 0..local_count {
      // locals 项：1 字节类型 + 2 字节对齐 + 两个 varint，逐项都要落在缓冲内
      if !fits(offset, 1, size) {
        return false;
      }

      remap_byte(
        &mut *(data.add(offset) as *mut u8),
        userdata_remapping,
        count,
      );

      offset += 2;

      if read_var_int(data, size, &mut offset).is_none()
        || read_var_int(data, size, &mut offset).is_none()
      {
        return false;
      }
    }

    // 三段布局必须刚好吃完 typeinfo：cpp 用 LUAU_ASSERT 表达同一约束
    offset == size
  }
}
