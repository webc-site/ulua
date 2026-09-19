use core::ffi::c_char;

use ulua_common::{
  enums::luau_bytecode_type::LBC_TYPE_TAGGED_USERDATA_BASE, macros::luau_assert::LUAU_ASSERT,
};

use crate::functions::{c_slice_mut, read_var_int::read_var_int};

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

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn remap_userdata_types(
  data: *mut c_char,
  size: usize,
  userdata_remapping: *mut u8,
  count: u32,
) {
  unsafe {
    let mut offset: usize = 0;

    let type_size = read_var_int(data, size, &mut offset);
    let upval_count = read_var_int(data, size, &mut offset);
    let local_count = read_var_int(data, size, &mut offset);

    if type_size != 0 {
      let types = data.add(offset) as *mut u8;

      // Skip two bytes of function type introduction（type_size < 2 时空区间）
      let body = c_slice_mut(types.add(2), type_size.saturating_sub(2) as usize);
      for t in body {
        remap_byte(t, userdata_remapping, count);
      }

      offset += type_size as usize;
    }

    if upval_count != 0 {
      let types = data.add(offset) as *mut u8;

      for t in c_slice_mut(types, upval_count as usize) {
        remap_byte(t, userdata_remapping, count);
      }

      offset += upval_count as usize;
    }

    if local_count != 0 {
      for _ in 0..local_count {
        remap_byte(
          &mut *(data.add(offset) as *mut u8),
          userdata_remapping,
          count,
        );

        offset += 2;
        read_var_int(data, size, &mut offset);
        read_var_int(data, size, &mut offset);
      }
    }

    LUAU_ASSERT!(offset == size);
  }
}
