use core::mem::size_of;
use std::{string::String, vec, vec::Vec};

use ulua_common::{
  enums::{
    luau_bytecode_tag::LuauBytecodeTag, luau_bytecode_type::LuauBytecodeType,
    luau_feedback_type::LuauFeedbackType, luau_proto_flag::LuauProtoFlag,
  },
  fflag,
  macros::luau_assert::LUAU_ASSERT,
};

use crate::{
  enums::bc_vm_const_kind::BcVmConstKind,
  fflag::LUAU_BYTECODE_COST_MODEL,
  functions::{bytecode_cursor::Cursor, read_string::read_string},
  methods::bc_function_remap_local_pcs::bc_function_remap_local_pcs,
  records::{
    bc_function::BcFunction, bc_vm_const::BcVmConst, bytecode_graph_parser::BytecodeGraphParser,
    class_shape::ClassShape, debug_local_bytecode_graph::DebugLocal, table_shape::TableShape,
    typed_local_bytecode_graph::TypedLocal,
  },
  type_aliases::instruction::Instruction,
};

const LBC_CONSTANT_NIL: u8 = LuauBytecodeTag::LBC_CONSTANT_NIL.0 as u8;
const LBC_CONSTANT_BOOLEAN: u8 = LuauBytecodeTag::LBC_CONSTANT_BOOLEAN.0 as u8;
const LBC_CONSTANT_NUMBER: u8 = LuauBytecodeTag::LBC_CONSTANT_NUMBER.0 as u8;
const LBC_CONSTANT_STRING: u8 = LuauBytecodeTag::LBC_CONSTANT_STRING.0 as u8;
const LBC_CONSTANT_IMPORT: u8 = LuauBytecodeTag::LBC_CONSTANT_IMPORT.0 as u8;
const LBC_CONSTANT_TABLE: u8 = LuauBytecodeTag::LBC_CONSTANT_TABLE.0 as u8;
const LBC_CONSTANT_CLOSURE: u8 = LuauBytecodeTag::LBC_CONSTANT_CLOSURE.0 as u8;
const LBC_CONSTANT_VECTOR: u8 = LuauBytecodeTag::LBC_CONSTANT_VECTOR.0 as u8;
const LBC_CONSTANT_VECTORD: u8 = LuauBytecodeTag::LBC_CONSTANT_VECTORD.0 as u8;
const LBC_CONSTANT_TABLE_WITH_CONSTANTS: u8 =
  LuauBytecodeTag::LBC_CONSTANT_TABLE_WITH_CONSTANTS.0 as u8;
const LBC_CONSTANT_INTEGER: u8 = LuauBytecodeTag::LBC_CONSTANT_INTEGER.0 as u8;
const LBC_CONSTANT_CLASS_SHAPE: u8 = LuauBytecodeTag::LBC_CONSTANT_CLASS_SHAPE.0 as u8;

/// 读取 `n` 个条目；调用方必须先用 [`Cursor::has_room_for`] 限定 `n`，
/// 否则损坏的长度字段会先触发一次天文数字分配再触发越界。
fn read_vec<T, F>(n: usize, c: &mut Cursor<'_>, mut f: F) -> Option<Vec<T>>
where
  F: FnMut(&mut Cursor<'_>) -> Option<T>,
{
  let mut out = Vec::with_capacity(n);
  for _ in 0..n {
    out.push(f(c)?);
  }
  Some(out)
}

/// cpp `fromFunctionBytecode`（`Bytecode/src/BytecodeGraph.cpp:28`）。
///
/// 输入是不可信字节码，任何越界或非法计数都以 `None`（"字节码损坏"）收口，
/// 而不是像 cpp 的 `LUAU_ASSERT` 那样在 release 下继续越界。
pub fn from_function_bytecode(bytecode: &[u8], strings: &[&[u8]]) -> Option<BcFunction> {
  let mut c = Cursor::new(bytecode);

  let maxstacksize = c.read::<u8>()?;
  let numparams = c.read::<u8>()?;
  let nups = c.read::<u8>()?;
  let is_vararg = c.read::<u8>()? != 0;
  let flags = c.read::<u8>()?;

  let mut fn_ = BcFunction {
    maxstacksize,
    numparams,
    nups,
    is_vararg,
    flags,
    ..Default::default()
  };

  let types_size = c.var_int()? as usize;
  if types_size > 0 {
    let type_info_size = c.var_int()? as usize;
    let typed_upval_size = c.var_int()? as usize;
    let typed_local_size = c.var_int()? as usize;

    fn_.type_info = c.bytes(type_info_size)?.to_vec();

    if !c.has_room_for(typed_upval_size, 1) {
      return None;
    }
    fn_.upvalue_types = read_vec(typed_upval_size, &mut c, |c| {
      Some(LuauBytecodeType(c.read::<u8>()? as u16))
    })?;

    if !c.has_room_for(typed_local_size, 4) {
      return None;
    }
    fn_.local_types = read_vec(typed_local_size, &mut c, |c| {
      let ty = c.read::<u8>()?;
      let reg = c.read::<u8>()?;
      let startpc = c.var_int()?;
      let endpc = startpc.wrapping_add(c.var_int()?);
      Some(TypedLocal {
        r#type: LuauBytecodeType(ty as u16),
        reg,
        startpc,
        endpc,
      })
    })?;
  }

  let codesize = c.var_int()?;
  let n_code = codesize as usize;
  if !c.has_room_for(n_code, size_of::<Instruction>()) {
    return None;
  }
  let code: Vec<Instruction> = read_vec(n_code, &mut c, |c| c.read::<Instruction>())?;

  let sizek = c.var_int()? as usize;
  if !c.has_room_for(sizek, 1) {
    return None;
  }
  let mut table_shapes: Vec<TableShape> = Vec::new();
  let mut class_shapes: Vec<ClassShape> = Vec::new();
  fn_.constants = read_vec(sizek, &mut c, |c| {
    read_constant(c, strings, sizek, &mut table_shapes, &mut class_shapes)
  })?;
  fn_.table_shapes = table_shapes;
  fn_.class_shapes = class_shapes;

  let psize = c.var_int()? as usize;
  if !c.has_room_for(psize, 1) {
    return None;
  }
  fn_.protos = read_vec(psize, &mut c, |c| c.var_int())?;

  fn_.linedefined = c.var_int()?;
  fn_.debugname = String::from_utf8_lossy(read_string(strings, &mut c)?).into_owned();

  let lineinfo = c.read::<u8>()?;
  let mut lines: Vec<u32> = Vec::new();

  if lineinfo != 0 {
    // linegaplog2 ≥ 32 在 cpp 里就是有符号移位 UB，这里按损坏字节码处理
    let linegaplog2 = u32::from(c.read::<u8>()?);
    if linegaplog2 >= 32 {
      return None;
    }

    // 对齐 cpp 的有符号移位语义：codesize == 0 时 intervals 为 0
    let intervals = (((n_code as i64) - 1) >> linegaplog2).saturating_add(1) as usize;
    let absoffset = (n_code + 3) & !3;

    let mut lineinfo_bytes = vec![0u8; absoffset];
    let mut abslineinfo = vec![0i32; intervals];

    let mut lastoffset = 0u8;
    for b in &mut lineinfo_bytes[..n_code] {
      lastoffset = lastoffset.wrapping_add(c.read::<u8>()?);
      *b = lastoffset;
    }

    let mut lastline = 0i32;
    for abs in &mut abslineinfo {
      lastline = lastline.wrapping_add(c.read::<i32>()?);
      *abs = lastline;
    }

    // i < codesize ⇒ i >> linegaplog2 < intervals
    lines = (0..n_code)
      .map(|i| (abslineinfo[i >> linegaplog2] + lineinfo_bytes[i] as i32) as u32)
      .collect();
  }

  let debuginfo = c.read::<u8>()?;

  if debuginfo != 0 {
    let sizelocvars = c.var_int()? as usize;
    if !c.has_room_for(sizelocvars, 4) {
      return None;
    }
    fn_.locals = read_vec(sizelocvars, &mut c, |c| {
      let varname = read_string(strings, c)?;
      let startpc = c.var_int()?;
      let endpc = c.var_int()?;
      let reg = c.read::<u8>()?;
      Some(DebugLocal {
        varname: String::from_utf8_lossy(varname).into_owned(),
        reg,
        startpc,
        endpc,
      })
    })?;

    let sizeupvalues = c.var_int()? as usize;
    if !c.has_room_for(sizeupvalues, 1) {
      return None;
    }
    fn_.upvalue_names = read_vec(sizeupvalues, &mut c, |c| {
      Some(String::from_utf8_lossy(read_string(strings, c)?).into_owned())
    })?;
  }

  // 调用反馈槽（对齐 cpp `FFlag::LuauCallFeedback` 段，slot PC 当前忽略）
  if fflag::LuauCallFeedback.get() {
    let feedback_vec_size = c.var_int()? as usize;
    if !c.has_room_for(feedback_vec_size, 2) {
      return None;
    }
    for _ in 0..feedback_vec_size {
      let slot_type = c.read::<u8>()?;
      LUAU_ASSERT!(slot_type == LuauFeedbackType::LFT_CALLTARGET as u8);
      // read slot PC. ignore it for now.
      let _ = c.var_int()?;
    }
  }

  // 成本模型（对齐 cpp `FFlag::LuauCostModel` 段，仅内联候选函数携带）
  if LUAU_BYTECODE_COST_MODEL.get() && (fn_.flags & (LuauProtoFlag::LPF_INLINABLE as u8)) != 0 {
    let _ = c.var_int_64()?;
  }

  let mut insns_pc: Vec<u32> = Vec::new();
  let mut graph_parser = BytecodeGraphParser::new(&mut fn_);
  if !graph_parser.rebuild_graph(&code, &lines, &mut insns_pc) {
    return None;
  }

  bc_function_remap_local_pcs(&mut fn_, &insns_pc, codesize);

  Some(fn_)
}

fn read_constant(
  c: &mut Cursor<'_>,
  strings: &[&[u8]],
  sizek: usize,
  table_shapes: &mut Vec<TableShape>,
  class_shapes: &mut Vec<ClassShape>,
) -> Option<BcVmConst> {
  let const_type = c.read::<u8>()?;
  let mut constant = BcVmConst::new();

  match const_type {
    LBC_CONSTANT_NIL => {
      constant.kind = BcVmConstKind::Nil;
    }
    LBC_CONSTANT_BOOLEAN => {
      constant.kind = BcVmConstKind::Boolean;
      constant.value.value_boolean = c.read::<u8>()? != 0;
    }
    LBC_CONSTANT_NUMBER => {
      constant.kind = BcVmConstKind::Number;
      constant.value.value_number = c.read::<f64>()?;
    }
    LBC_CONSTANT_VECTOR => {
      constant.kind = BcVmConstKind::Vector;
      constant.value.value_vector = [
        c.read::<f32>()?,
        c.read::<f32>()?,
        c.read::<f32>()?,
        c.read::<f32>()?,
      ];
    }
    // cpp `BytecodeGraph.cpp:103-111`：LBC_CONSTANT_VECTORD 为 4×double
    LBC_CONSTANT_VECTORD => {
      constant.kind = BcVmConstKind::Vectord;
      constant.value.value_vectord = [
        c.read::<f64>()?,
        c.read::<f64>()?,
        c.read::<f64>()?,
        c.read::<f64>()?,
      ];
    }
    LBC_CONSTANT_STRING => {
      constant.kind = BcVmConstKind::String;
      let s = read_string(strings, c)?;
      // 与 cpp `readString` 一致：常量按**原始字节**保留，绝不做 UTF-8 往返
      // （非法序列经 lossy 会变成 U+FFFD，回写常量表即永久污染、去重键失真）。
      // Box::leak 从所有权副本产出真正的 'static 切片，消除对调用方
      // 字符串表的借用（旧实现用 transmute 谎报生命期，属悬垂 UB）。
      // 常量表随编译产物存续，副本泄漏量以字符串常量数为上界。
      // 文本化（dump/错误消息）才在输出处 `from_utf8_lossy`。
      constant.value.value_string = Box::leak(s.to_vec().into_boxed_slice());
    }
    LBC_CONSTANT_IMPORT => {
      constant.kind = BcVmConstKind::Import;
      constant.value.value_import = c.read::<u32>()?;
    }
    LBC_CONSTANT_TABLE | LBC_CONSTANT_TABLE_WITH_CONSTANTS => {
      constant.kind = BcVmConstKind::Table;
      constant.value.value_table = table_shapes.len() as u32;

      let length = c.var_int()?;
      // cpp 直接写 `shape.keys[i]`（越界即栈溢出），这里以 TableShape 容量为硬上界
      if length > TableShape::K_MAX_LENGTH {
        return None;
      }
      let has_constants = const_type == LBC_CONSTANT_TABLE_WITH_CONSTANTS;

      let mut shape = TableShape {
        length,
        has_constants,
        ..Default::default()
      };
      let len = usize::try_from(length).ok()?;
      if !c.has_room_for(len, if has_constants { 5 } else { 1 }) {
        return None;
      }

      // keys 与 constants 为不相交字段，可同时可变借用，zip 单遍填充
      for (key, kconst) in shape
        .keys
        .iter_mut()
        .zip(shape.constants.iter_mut())
        .take(len)
      {
        // cpp 只断言 `key < sizek`；越界索引会在内联/优化阶段被当合法常量号使用
        let k = i32::try_from(c.var_int()?).ok()?;
        if k as usize >= sizek {
          return None;
        }
        *key = k;

        if has_constants {
          let v = c.read::<i32>()?;
          // cpp 的 `value < int32_t(sizek)` 是有符号比较，-1（未初始化）照样放行
          if v >= 0 && usize::try_from(v).ok()? >= sizek {
            return None;
          }
          *kconst = v;
        }
      }
      table_shapes.push(shape);
    }
    LBC_CONSTANT_CLOSURE => {
      constant.kind = BcVmConstKind::Closure;
      constant.value.value_closure = c.var_int()?;
    }
    LBC_CONSTANT_INTEGER => {
      constant.kind = BcVmConstKind::Integer;
      let is_negative = c.read::<u8>()? != 0;
      let magnitude = c.var_int_64()?;
      // cpp: `isNegative ? (int64_t)(~magnitude + 1) : (int64_t)magnitude`
      constant.value.value_integer = if is_negative {
        magnitude.wrapping_neg() as i64
      } else {
        magnitude as i64
      };
    }
    // cpp: `cpp/Bytecode/src/BytecodeGraph.cpp:169-194`
    LBC_CONSTANT_CLASS_SHAPE => {
      constant.kind = BcVmConstKind::ClassShape;
      constant.value.value_class_shape = class_shapes.len() as u32;

      let class_name = i32::try_from(c.var_int()?).ok()?;
      let num_props = c.var_int()? as usize;
      let num_methods = c.var_int()? as usize;
      if !c.has_room_for(num_props.saturating_add(num_methods), 1) {
        return None;
      }

      class_shapes.push(ClassShape {
        class_name,
        property_names: read_vec(num_props, c, |c| i32::try_from(c.var_int()?).ok())?,
        method_names: read_vec(num_methods, c, |c| i32::try_from(c.var_int()?).ok())?,
      });
    }
    _ => {
      LUAU_ASSERT!(false, "Unknown constant type!");
      return None;
    }
  }

  Some(constant)
}
