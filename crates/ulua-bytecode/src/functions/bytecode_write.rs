use std::vec::Vec;

/// 把原始字节追加进字节缓冲（cpp `std::string::push_back(char)` 的字节语义）。
#[inline]
pub(crate) fn write_byte(ss: &mut Vec<u8>, value: u8) {
  ss.push(value);
}

/// 把原始字节追加进字节缓冲（对齐 C++ writeInt/writeFloat/writeDouble 的 memcpy）。
#[inline]
pub(crate) fn write_bytes(ss: &mut Vec<u8>, bytes: &[u8]) {
  ss.extend_from_slice(bytes);
}

/// LEB128 载荷掩码：每字节低 7 位为值（读取侧 `try_read_var_int_64` 的 `0x7f` 镜像）。
const VARINT_PAYLOAD_MASK: u64 = 0x7f;
/// LEB128 续位：第 7 位置 1 表示后续还有字节（读取侧 `0x80` 的镜像）。
const VARINT_CONTINUE_BIT: u8 = 0x80;
/// LEB128 每字节的载荷位宽。
const VARINT_SHIFT: u32 = 7;

/// 写入变长无符号整数编码。
pub(crate) fn write_var_int(ss: &mut Vec<u8>, mut value: u64) {
  loop {
    let payload = (value & VARINT_PAYLOAD_MASK) as u8;
    let cont = if value > VARINT_PAYLOAD_MASK {
      VARINT_CONTINUE_BIT
    } else {
      0
    };
    write_byte(ss, payload | cont);
    value >>= VARINT_SHIFT;
    if value == 0 {
      break;
    }
  }
}
