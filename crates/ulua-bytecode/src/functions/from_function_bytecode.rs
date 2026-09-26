use core::mem::size_of;
use std::{string::String, vec, vec::Vec};

use ulua_common::{
  enums::{
    luau_bytecode_tag::LuauBytecodeTag, luau_bytecode_type::LuauBytecodeType,
    luau_feedback_type::LuauFeedbackType, luau_proto_flag::LuauProtoFlag,
  },
  fflag,
  fflag::LuauCostModel,
  macros::luau_assert::LUAU_ASSERT,
  records::instruction::Instruction,
};

use crate::{
  functions::{bytecode_cursor::Cursor, read_string::read_string},
  records::{
    bc_function::BcFunction, bc_vm_const::BcVmConst, bytecode_graph_parser::BytecodeGraphParser,
    class_shape::ClassShape, debug_local_bytecode_graph::DebugLocal, table_shape::TableShape,
    typed_local::TypedLocal,
  },
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
  (0..n).map(|_| f(c)).collect()
}

/// cpp `fromFunctionBytecode`（`Bytecode/src/BytecodeGraph.cpp:28`）。
///
/// 输入是不可信字节码，任何越界或非法计数都以 `None`（"字节码损坏"）收口，
/// 而不是像 cpp 的 `LUAU_ASSERT` 那样在 release 下继续越界。
/// `strings` 外层 Vec/slice 只在解析期间使用，产出的 `BcFunction` 仅借用表内
/// 的字节切片（cpp `string_view` 语义），故外层与内层生命周期分离——否则
/// 调用方临时构造的指针表会成为返回图的借用来源，图无法在表释放后存活。
pub fn from_function_bytecode<'a>(bytecode: &[u8], strings: &[&'a [u8]]) -> Option<BcFunction<'a>> {
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
  // shape 表由本函数（调用方）统一编号入表：read_constant 只返回 shape 载荷，
  // 消灭旧实现把 `&mut Vec` 当出参反塞的模式。解析顺序不变 ⇒ 下标与 cpp 一致。
  let mut table_shapes: Vec<TableShape> = Vec::new();
  let mut class_shapes: Vec<ClassShape> = Vec::new();
  // 常量段走 read_vec 单源：读满 sizek 条，任一损坏即整段判 `None`，
  // 已入表的 shape 随函数返回一并丢弃（与旧的逐条 push 写法等价）。
  let constants: Vec<BcVmConst<'_>> = read_vec(sizek, &mut c, |c| {
    Some(match read_constant(c, strings, sizek)? {
      ConstRead::Value(v) => v,
      ConstRead::Table(shape) => {
        // Table 载荷是 shape 在 `table_shapes` 中的下标（先取号再入表）。
        let shape_idx = table_shapes.len() as u32;
        table_shapes.push(*shape);
        BcVmConst::Table(shape_idx)
      }
      ConstRead::Class(shape) => {
        // ClassShape 载荷是 shape 在 `class_shapes` 中的下标（先取号再入表）。
        let shape_idx = class_shapes.len() as u32;
        class_shapes.push(shape);
        BcVmConst::ClassShape(shape_idx)
      }
    })
  })?;
  fn_.constants = constants;
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
    // 计数驱动的跳过：条数来自字节码流、无容器可迭代，槽位号也不参与任何计算
    // （只校验类型并推进游标），故此处不是索引游走，保留计数写法。
    for _ in 0..feedback_vec_size {
      let slot_type = c.read::<u8>()?;
      LUAU_ASSERT!(slot_type == LuauFeedbackType::LFT_CALLTARGET as u8);
      // read slot PC. ignore it for now.
      let _ = c.var_int()?;
    }
  }

  // 成本模型（对齐 cpp `FFlag::LuauCostModel` 段，仅内联候选函数携带）
  if LuauCostModel.get() && (fn_.flags & (LuauProtoFlag::LPF_INLINABLE as u8)) != 0 {
    let _ = c.var_int_64()?;
  }

  let mut graph_parser = BytecodeGraphParser::new(&mut fn_);
  let insns_pc = graph_parser.rebuild_graph(&code, &lines)?;

  fn_.remap_local_pcs(&insns_pc, codesize);

  Some(fn_)
}

/// [`read_constant`] 的单条解析结果：普通常量直接给值；Table / ClassShape 额外
/// 携带 shape 载荷，由调用方统一入表编号（见 `from_function_bytecode` 常量段）。
/// `TableShape`（264 B）装箱以平衡变体大小——仅在解析期一次性解箱搬入形状表，
/// 不触及运行热路径。
enum ConstRead<'a> {
  Value(BcVmConst<'a>),
  Table(Box<TableShape>),
  Class(ClassShape),
}

fn read_constant<'a>(
  c: &mut Cursor<'_>,
  strings: &[&'a [u8]],
  sizek: usize,
) -> Option<ConstRead<'a>> {
  let const_type = c.read::<u8>()?;
  let constant = match const_type {
    LBC_CONSTANT_NIL => ConstRead::Value(BcVmConst::Nil),
    LBC_CONSTANT_BOOLEAN => ConstRead::Value(BcVmConst::Boolean(c.read::<u8>()? != 0)),
    LBC_CONSTANT_NUMBER => ConstRead::Value(BcVmConst::Number(c.read::<f64>()?)),
    LBC_CONSTANT_VECTOR => ConstRead::Value(BcVmConst::Vector([
      c.read::<f32>()?,
      c.read::<f32>()?,
      c.read::<f32>()?,
      c.read::<f32>()?,
    ])),
    // cpp `BytecodeGraph.cpp:103-111`：LBC_CONSTANT_VECTORD 为 4×double
    LBC_CONSTANT_VECTORD => ConstRead::Value(BcVmConst::Vectord([
      c.read::<f64>()?,
      c.read::<f64>()?,
      c.read::<f64>()?,
      c.read::<f64>()?,
    ])),
    // cpp `BytecodeGraph.cpp:110`：`valueString = readString(...)` 直接借用
    // `strings` 表中的原始字节（string_view 语义），不拷贝、不泄漏。
    LBC_CONSTANT_STRING => ConstRead::Value(BcVmConst::from_string(read_string(strings, c)?)),
    LBC_CONSTANT_IMPORT => ConstRead::Value(BcVmConst::Import(c.read::<u32>()?)),
    LBC_CONSTANT_TABLE | LBC_CONSTANT_TABLE_WITH_CONSTANTS => {
      // cpp 直接写 `shape.keys[i]`（越界即栈溢出），这里以 TableShape 容量为硬上界
      let length = c.var_int()?;
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

      // shape 载荷直接返回，入表编号由调用方统一完成。
      ConstRead::Table(Box::new(shape))
    }
    LBC_CONSTANT_CLOSURE => ConstRead::Value(BcVmConst::Closure(c.var_int()?)),
    LBC_CONSTANT_INTEGER => {
      let is_negative = c.read::<u8>()? != 0;
      let magnitude = c.var_int_64()?;
      // cpp: `isNegative ? (int64_t)(~magnitude + 1) : (int64_t)magnitude`
      ConstRead::Value(BcVmConst::Integer(if is_negative {
        magnitude.wrapping_neg() as i64
      } else {
        magnitude as i64
      }))
    }
    // cpp: `cpp/Bytecode/src/BytecodeGraph.cpp:169-194`
    LBC_CONSTANT_CLASS_SHAPE => {
      let class_name = i32::try_from(c.var_int()?).ok()?;
      let num_props = c.var_int()? as usize;
      let num_methods = c.var_int()? as usize;
      if !c.has_room_for(num_props.saturating_add(num_methods), 1) {
        return None;
      }

      // shape 载荷直接返回，入表编号由调用方统一完成。
      ConstRead::Class(ClassShape {
        class_name,
        property_names: read_vec(num_props, c, |c| i32::try_from(c.var_int()?).ok())?,
        method_names: read_vec(num_methods, c, |c| i32::try_from(c.var_int()?).ok())?,
      })
    }
    _ => {
      // cpp 原址为 `LUAU_ASSERT(!"Unknown constant type")`：tag 来自不可信输入，
      // debug 断言会经 debugbreak 中断进程，违反「损坏字节码一律 None 收口」的
      // 契约（`tests/from_function_bytecode_invalid.rs` 负向回归守门）。
      return None;
    }
  };

  Some(constant)
}
