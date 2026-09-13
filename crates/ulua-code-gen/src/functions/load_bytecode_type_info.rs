use core::slice::from_raw_parts;

use ulua_common::enums::luau_bytecode_type::{LBC_TYPE_ANY, LBC_TYPE_FUNCTION};

use crate::records::{
  bytecode_reg_type_info::BytecodeRegTypeInfo, bytecode_type_info::BytecodeTypeInfo,
  ir_function::IrFunction,
};

fn read_u8(data: &[u8], offset: &mut usize) -> u8 {
  let result = data[*offset];
  *offset += 1;
  result
}

fn read_var_int(data: &[u8], offset: &mut usize) -> u32 {
  let mut result = 0u32;
  let mut shift = 0u32;

  loop {
    let byte = read_u8(data, offset);
    result |= ((byte & 127) as u32) << shift;
    shift += 7;

    if byte & 128 == 0 {
      return result;
    }
  }
}

pub fn load_bytecode_type_info(function: &mut IrFunction) {
  let proto = function.proto;
  if proto.is_null() {
    return;
  }

  let proto = unsafe { &*proto };
  let type_info: &mut BytecodeTypeInfo = &mut function.bc_type_info;
  type_info.argument_types.clear();
  type_info.upvalue_types.clear();
  type_info.reg_types.clear();
  type_info.reg_type_offsets.clear();

  if proto.typeinfo.is_null() {
    type_info
      .argument_types
      .resize(proto.numparams as usize, LBC_TYPE_ANY.0 as u8);
    type_info
      .upvalue_types
      .resize(proto.nups as usize, LBC_TYPE_ANY.0 as u8);
    return;
  }

  let data = unsafe { from_raw_parts(proto.typeinfo as *const u8, proto.sizetypeinfo as usize) };
  let mut offset = 0usize;

  let type_size = read_var_int(data, &mut offset) as usize;
  let upval_count = read_var_int(data, &mut offset) as usize;
  let local_count = read_var_int(data, &mut offset) as usize;

  if type_size != 0 {
    let types = &data[offset..offset + type_size];

    assert_eq!(type_size, 2 + proto.numparams as usize);
    assert_eq!(types[0], LBC_TYPE_FUNCTION.0 as u8);
    assert_eq!(types[1], proto.numparams);

    type_info
      .argument_types
      .extend_from_slice(&types[2..2 + proto.numparams as usize]);
    offset += type_size;
  }

  if upval_count != 0 {
    assert_eq!(upval_count, proto.nups as usize);

    type_info
      .upvalue_types
      .extend_from_slice(&data[offset..offset + upval_count]);
    offset += upval_count;
  }

  if local_count != 0 {
    type_info.reg_types.reserve(local_count);

    for _ in 0..local_count {
      let r#type = read_u8(data, &mut offset);
      let reg = read_u8(data, &mut offset);
      let startpc = read_var_int(data, &mut offset) as i32;
      let endpc = startpc + read_var_int(data, &mut offset) as i32;

      type_info.reg_types.push(BytecodeRegTypeInfo {
        r#type,
        reg,
        startpc,
        endpc,
      });
    }
  }

  function.bc_original_type_info = function.bc_type_info.clone();

  assert_eq!(offset, proto.sizetypeinfo as usize);
}

/// C++ equivalent:
/// - void loadBytecodeTypeInfo(IrFunction& function)
/// - void* pointer deref/read_var_int/read
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn load_bytecode_type_info_from_bytes(
  function: &mut IrFunction,
  data: *const u8,
  type_size: usize,
) {
  if data.is_null() {
    return;
  }

  let data = unsafe { from_raw_parts(data, type_size) };
  let mut offset = 0usize;

  let type_size = read_var_int(data, &mut offset) as usize;
  let upval_count = read_var_int(data, &mut offset) as usize;
  let local_count = read_var_int(data, &mut offset) as usize;

  function.bc_type_info.argument_types.clear();
  function.bc_type_info.upvalue_types.clear();
  function.bc_type_info.reg_types.clear();
  function.bc_type_info.reg_type_offsets.clear();

  if type_size != 0 {
    let types = &data[offset..offset + type_size];
    assert_eq!(types[0], LBC_TYPE_FUNCTION.0 as u8);
    function
      .bc_type_info
      .argument_types
      .extend_from_slice(&types[2..]);
    offset += type_size;
  }

  if upval_count != 0 {
    function
      .bc_type_info
      .upvalue_types
      .extend_from_slice(&data[offset..offset + upval_count]);
    offset += upval_count;
  }

  for _ in 0..local_count {
    let r#type = read_u8(data, &mut offset);
    let reg = read_u8(data, &mut offset);
    let startpc = read_var_int(data, &mut offset) as i32;
    let endpc = startpc + read_var_int(data, &mut offset) as i32;

    function.bc_type_info.reg_types.push(BytecodeRegTypeInfo {
      r#type,
      reg,
      startpc,
      endpc,
    });
  }

  function.bc_original_type_info = function.bc_type_info.clone();
  assert_eq!(offset, data.len());
}
