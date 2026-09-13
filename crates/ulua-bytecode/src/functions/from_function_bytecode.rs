use alloc::{string::String, vec::Vec};
use core::{mem, str};

use ulua_common::{
  enums::{luau_bytecode_tag::LuauBytecodeTag, luau_bytecode_type::LuauBytecodeType},
  functions::{read::read, read_var_int::read_var_int, read_var_int_64::read_var_int_64},
  macros::luau_assert::LUAU_ASSERT,
};

use crate::{
  enums::bc_vm_const_kind::BcVmConstKind,
  functions::read_string::read_string,
  methods::bc_function_remap_local_pcs::bc_function_remap_local_pcs,
  records::{
    bc_function::BcFunction, bc_vm_const::BcVmConst, bytecode_graph_parser::BytecodeGraphParser,
    debug_local_bytecode_graph::DebugLocal, table_shape::TableShape,
    typed_local_bytecode_graph::TypedLocal,
  },
  type_aliases::{comp_time_bc_function::CompTimeBcFunction, instruction::Instruction},
};

const LBC_CONSTANT_NIL: u8 = LuauBytecodeTag::LBC_CONSTANT_NIL.0 as u8;
const LBC_CONSTANT_BOOLEAN: u8 = LuauBytecodeTag::LBC_CONSTANT_BOOLEAN.0 as u8;
const LBC_CONSTANT_NUMBER: u8 = LuauBytecodeTag::LBC_CONSTANT_NUMBER.0 as u8;
const LBC_CONSTANT_STRING: u8 = LuauBytecodeTag::LBC_CONSTANT_STRING.0 as u8;
const LBC_CONSTANT_IMPORT: u8 = LuauBytecodeTag::LBC_CONSTANT_IMPORT.0 as u8;
const LBC_CONSTANT_TABLE: u8 = LuauBytecodeTag::LBC_CONSTANT_TABLE.0 as u8;
const LBC_CONSTANT_CLOSURE: u8 = LuauBytecodeTag::LBC_CONSTANT_CLOSURE.0 as u8;
const LBC_CONSTANT_VECTOR: u8 = LuauBytecodeTag::LBC_CONSTANT_VECTOR.0 as u8;
const LBC_CONSTANT_TABLE_WITH_CONSTANTS: u8 =
  LuauBytecodeTag::LBC_CONSTANT_TABLE_WITH_CONSTANTS.0 as u8;
const LBC_CONSTANT_INTEGER: u8 = LuauBytecodeTag::LBC_CONSTANT_INTEGER.0 as u8;

pub fn from_function_bytecode(
  bytecode: String,
  strings: &mut Vec<&[u8]>,
) -> Option<CompTimeBcFunction> {
  let data = bytecode.as_bytes();
  let mut offset = 0usize;

  let maxstacksize = read::<u8>(data, &mut offset);
  let numparams = read::<u8>(data, &mut offset);
  let nups = read::<u8>(data, &mut offset);
  let is_vararg = read::<u8>(data, &mut offset) != 0;
  let flags = read::<u8>(data, &mut offset);

  let mut fn_ = BcFunction {
    maxstacksize,
    numparams,
    nups,
    is_vararg,
    flags,
    ..Default::default()
  };

  let types_size = read_var_int(data, &mut offset);
  if types_size > 0 {
    let type_info_size = read_var_int(data, &mut offset);
    let typed_upval_size = read_var_int(data, &mut offset);
    let typed_local_size = read_var_int(data, &mut offset);

    fn_.type_info = bytecode[offset..offset + type_info_size as usize].to_string();
    offset += type_info_size as usize;

    fn_.upvalue_types = (0..typed_upval_size as usize)
      .map(|_| LuauBytecodeType(read::<u8>(data, &mut offset) as u16))
      .collect();

    fn_.local_types = (0..typed_local_size as usize)
      .map(|_| {
        let ty = read::<u8>(data, &mut offset);
        let reg = read::<u8>(data, &mut offset);
        let startpc = read_var_int(data, &mut offset);
        let endpc = startpc + read_var_int(data, &mut offset);
        TypedLocal {
          r#type: LuauBytecodeType(ty as u16),
          reg,
          startpc,
          endpc,
        }
      })
      .collect();
  }

  let codesize = read_var_int(data, &mut offset) as usize;
  let code: Vec<Instruction> = (0..codesize)
    .map(|_| read::<Instruction>(data, &mut offset))
    .collect();

  let sizek = read_var_int(data, &mut offset) as usize;
  fn_.constants.resize(sizek, BcVmConst::new());
  for c in fn_.constants.iter_mut() {
    let const_type = read::<u8>(data, &mut offset);
    match const_type {
      LBC_CONSTANT_NIL => {
        c.kind = BcVmConstKind::Nil;
      }
      LBC_CONSTANT_BOOLEAN => {
        c.kind = BcVmConstKind::Boolean;
        c.value.value_boolean = read::<u8>(data, &mut offset) != 0;
      }
      LBC_CONSTANT_NUMBER => {
        c.kind = BcVmConstKind::Number;
        c.value.value_number = read::<f64>(data, &mut offset);
      }
      LBC_CONSTANT_VECTOR => {
        c.kind = BcVmConstKind::Vector;
        c.value.value_vector = [
          read::<f32>(data, &mut offset),
          read::<f32>(data, &mut offset),
          read::<f32>(data, &mut offset),
          read::<f32>(data, &mut offset),
        ];
      }
      LBC_CONSTANT_STRING => {
        c.kind = BcVmConstKind::String;
        let s = read_string(strings, data, &mut offset);
        // SAFETY：字节码常量表中的字符串在源语言层面均为合法 UTF-8；
        // transmute 把生命期提升为 'static —— 字符串切片实际指向调用方
        // 持有的 bytecode 缓冲，其存活期覆盖 BcVmConst 的使用范围。
        unsafe {
          let value = str::from_utf8_unchecked(s);
          c.value.value_string = mem::transmute::<&str, &'static str>(value);
        }
      }
      LBC_CONSTANT_IMPORT => {
        c.kind = BcVmConstKind::Import;
        c.value.value_import = read::<u32>(data, &mut offset);
      }
      LBC_CONSTANT_TABLE | LBC_CONSTANT_TABLE_WITH_CONSTANTS => {
        c.kind = BcVmConstKind::Table;
        c.value.value_table = fn_.table_shapes.len() as u32;

        let length = read_var_int(data, &mut offset);
        let has_constants = const_type == LBC_CONSTANT_TABLE_WITH_CONSTANTS;
        let mut shape = TableShape {
          length,
          has_constants,
          ..Default::default()
        };

        for (j, key) in shape
          .keys
          .iter_mut()
          .enumerate()
          .take(shape.length as usize)
        {
          *key = read_var_int(data, &mut offset) as i32;
          if shape.has_constants {
            shape.constants[j] = read::<i32>(data, &mut offset);
          }
        }
        fn_.table_shapes.push(shape);
      }
      LBC_CONSTANT_CLOSURE => {
        c.kind = BcVmConstKind::Closure;
        c.value.value_closure = read_var_int(data, &mut offset);
      }
      LBC_CONSTANT_INTEGER => {
        c.kind = BcVmConstKind::Integer;
        let is_negative = read::<u8>(data, &mut offset) != 0;
        let magnitude = read_var_int_64(data, &mut offset);
        c.value.value_integer = if is_negative {
          !(magnitude - 1) as i64
        } else {
          magnitude as i64
        };
      }
      _ => {
        LUAU_ASSERT!(false, "Unknown constant type!");
        return None;
      }
    }
  }

  let psize = read_var_int(data, &mut offset) as usize;
  fn_.protos = (0..psize)
    .map(|_| read_var_int(data, &mut offset))
    .collect();

  fn_.linedefined = read_var_int(data, &mut offset);
  fn_.debugname =
    unsafe { str::from_utf8_unchecked(read_string(strings, data, &mut offset)) }.to_string();

  let lineinfo = read::<u8>(data, &mut offset);
  let mut lines: Vec<u32> = Vec::new();

  if lineinfo != 0 {
    let linegaplog2 = read::<u8>(data, &mut offset) as usize;

    let intervals = ((codesize - 1) >> linegaplog2) + 1;
    let absoffset = (codesize + 3) & !3;

    let mut lineinfo_bytes = vec![0u8; absoffset];
    let mut abslineinfo = vec![0i32; intervals];

    let mut lastoffset = 0u8;
    for b in &mut lineinfo_bytes[..codesize] {
      lastoffset = lastoffset.wrapping_add(read::<u8>(data, &mut offset));
      *b = lastoffset;
    }

    let mut lastline = 0i32;
    for abs in &mut abslineinfo {
      lastline += read::<i32>(data, &mut offset);
      *abs = lastline;
    }

    lines = (0..codesize)
      .map(|i| {
        let idx = i >> linegaplog2;
        let abs = abslineinfo[idx];
        let off = lineinfo_bytes[i] as i32;
        (abs + off) as u32
      })
      .collect();
  }

  let debuginfo = read::<u8>(data, &mut offset);

  if debuginfo != 0 {
    let sizelocvars = read_var_int(data, &mut offset) as usize;
    fn_.locals = (0..sizelocvars)
      .map(|_| {
        let varname = read_string(strings, data, &mut offset);
        let startpc = read_var_int(data, &mut offset);
        let endpc = read_var_int(data, &mut offset);
        let reg = read::<u8>(data, &mut offset);
        DebugLocal {
          varname: unsafe {
            mem::transmute::<&str, &'static str>(str::from_utf8_unchecked(varname))
          },
          reg,
          startpc,
          endpc,
        }
      })
      .collect();

    let sizeupvalues = read_var_int(data, &mut offset) as usize;
    fn_.upvalue_names = (0..sizeupvalues)
      .map(|_| {
        let name = read_string(strings, data, &mut offset);
        unsafe { str::from_utf8_unchecked(name) }.to_string()
      })
      .collect();
  }

  let mut insns_pc: Vec<u32> = Vec::new();
  let mut graph_parser = BytecodeGraphParser::new(&mut fn_);
  if !graph_parser.rebuild_graph(&code, &lines, &mut insns_pc) {
    return None;
  }

  bc_function_remap_local_pcs(&mut fn_, &insns_pc, codesize as u32);

  Some(fn_)
}
