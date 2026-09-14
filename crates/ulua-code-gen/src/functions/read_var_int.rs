use crate::functions::read::read;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn read_var_int(data: *const u8, offset: &mut usize) -> u32 {
  unsafe {
    let mut result: u32 = 0;
    let mut shift: u32 = 0;
    let mut byte: u8;

    loop {
      byte = read::<u8>(data, offset);
      result |= ((byte & 127) as u32) << shift;
      shift += 7;
      if (byte & 128) == 0 {
        break;
      }
    }

    result
  }
}
