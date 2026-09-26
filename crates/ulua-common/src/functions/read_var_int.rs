//! Source: `Common/src/BytecodeWire.cpp`

use crate::functions::read_var_int_64::read_var_int_64;

pub fn read_var_int(data: &[u8], offset: &mut usize) -> u32 {
  read_var_int_64(data, offset) as u32
}
