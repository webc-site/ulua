use core::ffi::c_char;

use ulua_common::{
  enums::luau_bytecode_type::LBC_TYPE_TAGGED_USERDATA_BASE, macros::luau_assert::LUAU_ASSERT,
};

use crate::functions::{c_slice_mut, read_var_int::read_var_int};

/// 单字节类型重映射：带符号差转 u32 索引，命中重映射表才写回
#[inline]
unsafe fn remap_byte(t: &mut u8, userdata_remapping: *const u8, count: u32) {
  let index = (*t as i32 - LBC_TYPE_TAGGED_USERDATA_BASE.0 as i32) as u32;

  if index < count {
    unsafe { *t = *userdata_remapping.add(index as usize) };
  }
}

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

      // Skip two bytes of function type introduction
      for i in 2..type_size {
        remap_byte(&mut *types.add(i as usize), userdata_remapping, count);
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
