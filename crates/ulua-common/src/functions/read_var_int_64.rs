//! Generated skeleton item.
//! Node: `cxx:Function:Luau.Common:Common/src/BytecodeWire.cpp:9:read_var_int_64`
//! Source: `Common/src/BytecodeWire.cpp`
//! Graph edges:
//! - declared_by: source_file Common/src/BytecodeWire.cpp
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/BytecodeWire.h
//!   - includes -> source_file Common/include/Luau/Common.h
//! - incoming:
//!   - declares <- source_file Common/src/BytecodeWire.cpp

use crate::functions::read::read;

pub fn read_var_int_64(data: &[u8], offset: &mut usize) -> u64 {
  let mut result: u64 = 0;
  let mut shift: u32 = 0;

  loop {
    let byte = read::<u8>(data, offset);
    // 损坏的流可能连续出现续字节使 shift ≥ 64（u64 的 varint 最多 10 字节）；
    // 溢出位按丢弃处理，避免 debug 下移位溢出 panic。
    if shift < 64 {
      result |= u64::from(byte & 0x7f) << shift;
    }
    shift += 7;

    if byte & 0x80 == 0 {
      return result;
    }
  }
}
