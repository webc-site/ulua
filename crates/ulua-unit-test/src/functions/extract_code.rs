use alloc::vec::Vec;
use core::mem::size_of;

/// 从函数级字节码 blob 中提取指令段（原始字节），供 roundtrip 断言比较。
/// 头部布局：4 字节函数头 + 1 字节类型段标记（偏移 5 起为 VarInt 类型段长度）。
pub fn extract_code(bytecode: &[u8]) -> Vec<u8> {
  let data = bytecode;
  let mut offset = 5usize;

  // readVarInt（LEB128）局部实现：unit-test crate 不引 ulua_common 依赖。
  fn read_var_int(data: &[u8], offset: &mut usize) -> u32 {
    let mut result = 0u32;
    let mut shift = 0u32;
    loop {
      let byte = data[*offset];
      *offset += 1;
      result |= ((byte & 0x7F) as u32) << shift;
      if (byte & 0x80) == 0 {
        break;
      }
      shift += 7;
    }
    result
  }

  let type_info_size = read_var_int(data, &mut offset) as usize;
  offset += type_info_size;

  let code_size = read_var_int(data, &mut offset) as usize;

  // Instruction is defined as uint32_t in Bytecode.h
  let instruction_size = size_of::<u32>();
  let start = offset;
  let end = offset + code_size * instruction_size;

  if end <= data.len() {
    data[start..end].to_vec()
  } else {
    Vec::new()
  }
}
