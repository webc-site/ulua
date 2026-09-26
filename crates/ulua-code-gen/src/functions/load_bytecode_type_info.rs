use ulua_common::{
  enums::luau_bytecode_type::LuauBytecodeType,
  functions::{read::read, read_var_int::read_var_int},
};

use crate::{
  functions::proto_views::typeinfo,
  records::{
    bytecode_reg_type_info::BytecodeRegTypeInfo, bytecode_type_info::BytecodeTypeInfo,
    ir_function::IrFunction,
  },
};

/// 载入字节码类型信息（cpp `loadBytecodeTypeInfo`）。
///
/// 契约：`function.proto` 为空或指向存活 Proto（IrFunction 构造接线），`typeinfo`/
/// `sizetypeinfo` 由字节码加载成界记录；原型子数组一律经 [`proto_views`] 的安全视图读取。
pub fn load_bytecode_type_info(function: &mut IrFunction) {
  let Some(proto) = function.proto_view() else {
    return;
  };
  let type_info: &mut BytecodeTypeInfo = &mut function.bc_type_info;
  type_info.argument_types.clear();
  type_info.upvalue_types.clear();
  type_info.reg_types.clear();
  type_info.reg_type_offsets.clear();

  // cpp 判据是 `!proto->typeinfo`；空基址与零长度在 VM 侧同义（无类型信息缓冲），故用视图
  // 的 `is_empty()` 收口，取安全方向（视图会把两者都折成空切片）。
  let data = typeinfo(proto);
  if data.is_empty() {
    type_info.argument_types.resize(
      proto.numparams as usize,
      LuauBytecodeType::LBC_TYPE_ANY.0 as u8,
    );
    type_info
      .upvalue_types
      .resize(proto.nups as usize, LuauBytecodeType::LBC_TYPE_ANY.0 as u8);
    return;
  }

  let mut offset = 0usize;

  let type_size = read_var_int(data, &mut offset) as usize;
  let upval_count = read_var_int(data, &mut offset) as usize;
  let local_count = read_var_int(data, &mut offset) as usize;

  if type_size != 0 {
    let types = &data[offset..offset + type_size];

    assert_eq!(type_size, 2 + proto.numparams as usize);
    assert_eq!(types[0], LuauBytecodeType::LBC_TYPE_FUNCTION.0 as u8);
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
      let r#type = read::<u8>(data, &mut offset);
      let reg = read::<u8>(data, &mut offset);
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
