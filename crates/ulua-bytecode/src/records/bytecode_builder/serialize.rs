//! `BytecodeBuilder` 之 字节码序列化：write_function / write_line_info / finalize 与版本选择。

use core::{mem::take, slice};
use std::{vec, vec::Vec};

use ulua_common::{
  enums::{
    luau_bytecode_tag::LuauBytecodeTag, luau_feedback_type::LuauFeedbackType,
    luau_proto_flag::LuauProtoFlag,
  },
  fflag,
  macros::luau_assert::LUAU_ASSERT,
};

use super::BytecodeBuilder;
use crate::{
  functions::{
    bytecode_write::{write_byte, write_bytes, write_var_int},
    log_2::log2,
  },
  records::{class_shape::ClassShape, constant::Constant, string_ref::StringRef},
};

// ── abs-r139：并自 `methods/bytecode_builder_finalize.rs` ──
// Source: `Bytecode/src/BytecodeBuilder.cpp:676`
//
// Faithful port of `BytecodeBuilder::finalize`: assemble the final bytecode
// blob — version byte, type-encoding version, string table, userdata type-name
// mapping, then every function's pre-serialized `data` blob, then the main
// function index. `bytecode` is taken out of `self` while writing so the
// `&self` helpers (`write_string_table`) and `&mut self` field reads don't
// alias the Buffer being filled.

impl<'a> BytecodeBuilder<'a> {
  pub fn finalize(&mut self) {
    LUAU_ASSERT!(self.bytecode.is_empty());

    // 已使用的 userdata 类型名先注册进字符串表。`UserdataType::name` 是 `StringRef<'a>`
    // （借用上游名表的视图，clone 不拷贝字节），因此这里把 `self` 拆成互不相交的
    // 字段借用即可完成，无需索引循环或裸指针。
    {
      let BytecodeBuilder {
        userdata_types,
        string_table,
        debug_strings,
        dump_flags,
        ..
      } = self;
      for ty in userdata_types.iter_mut() {
        if ty.used {
          ty.name_ref = super::interning::intern_string(
            string_table,
            debug_strings,
            *dump_flags,
            ty.name.clone(),
          );
        }
      }
    }

    // preallocate space for bytecode blob
    let mut capacity: usize = 16;

    for (string_ref, _index) in self.string_table.iter() {
      capacity += string_ref.len() + 2;
    }

    for func in &self.functions {
      capacity += func.data.len();
    }

    // assemble final bytecode blob — taken out of `self` so the Buffer is
    // not aliased by the `&self`/field reads below.
    let mut bytecode = take(&mut self.bytecode);
    bytecode.reserve(capacity);

    let version = self.get_version();
    // cpp 769: `(version >= LBC_VERSION_MIN && version <= LBC_VERSION_MAX) || version == LBC_VERSION_CLASSES`
    LUAU_ASSERT!(
      (version >= LuauBytecodeTag::LBC_VERSION_MIN.0 as u8
        && version <= LuauBytecodeTag::LBC_VERSION_MAX.0 as u8)
        || version == LuauBytecodeTag::LBC_VERSION_CLASSES.0 as u8
    );

    // 版本字节直接写入字节缓冲（Vec<u8>，无 UTF-8 不变量约束）。
    bytecode.push(version);

    let typesversion = self.get_type_encoding_version();
    LUAU_ASSERT!(
      typesversion >= LuauBytecodeTag::LBC_TYPE_VERSION_MIN.0 as u8
        && typesversion <= LuauBytecodeTag::LBC_TYPE_VERSION_MAX.0 as u8
    );
    write_byte(&mut bytecode, typesversion);

    self.write_string_table(&mut bytecode);

    // Write the mapping between used type name indices and their name
    for (i, userdata_type) in self.userdata_types.iter().enumerate() {
      if userdata_type.used {
        write_byte(&mut bytecode, (i + 1) as u8);
        write_var_int(&mut bytecode, userdata_type.name_ref as u64);
      }
    }

    // 0 marks the end of the mapping
    write_byte(&mut bytecode, 0);

    write_var_int(&mut bytecode, self.functions.len() as u64);

    // cpp 797-802：四个 flag 任一开启时，每个函数体前写 VarInt 长度前缀，
    // 供读取方在应用该 flag 的增量（cost model / double 常量 / fastcall /
    // class shape）前定位函数边界。谓词与 `write_function` 的 feedback/cost
    // 尾段共用单一源。
    let size_prefix = cost_section_enabled();

    for func in &self.functions {
      if size_prefix {
        write_var_int(&mut bytecode, func.data.len() as u64);
      }
      // func.data 的字节流原样写入字节缓冲。
      bytecode.extend_from_slice(&func.data);
    }

    LUAU_ASSERT!((self.main_function as usize) < self.functions.len());
    write_var_int(&mut bytecode, self.main_function as u64);

    self.bytecode = bytecode;
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_get_bytecode.rs` ──
impl<'a> BytecodeBuilder<'a> {
  /// finalize 后的字节码 blob 原始字节。
  pub fn get_bytecode(&self) -> &[u8] {
    LUAU_ASSERT!(!self.bytecode.is_empty()); // did you forget to call finalize?
    &self.bytecode
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_get_error.rs` ──
/// 错误 blob 首字节标记：合法 blob 首字节为字节码版本（≥ 1），0 不可能是版本号，
/// 故 0 即「这不是字节码，是错误消息」（cpp BytecodeBuilder.cpp `getError` 同值）。
const ERROR_BLOB_MARKER: u8 = 0;

impl<'a> BytecodeBuilder<'a> {
  /// 错误字节码 blob：首字节为错误标记，其后为可读消息的原始字节。
  pub fn get_error(message: &str) -> Vec<u8> {
    let mut result = Vec::with_capacity(message.len() + 1);
    result.push(ERROR_BLOB_MARKER);
    result.extend_from_slice(message.as_bytes());

    result
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_get_type_encoding_version.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub(crate) fn get_type_encoding_version(&self) -> u8 {
    // C++: `return LBC_TYPE_VERSION_TARGET;` (3). Was stubbed to 1, which made
    // the Rust compiler emit a v1 typeinfo Header the VM loader can't parse.
    LuauBytecodeTag::LBC_TYPE_VERSION_TARGET.0 as u8
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_get_version.rs` ──
// Source: `Bytecode/src/BytecodeBuilder.cpp:1487-1503` (hand-ported).
//
// ```cpp
// uint8_t BytecodeBuilder::getVersion()
// {
//     if (FFlag::DebugLuauUserDefinedClasses)
//         return LBC_VERSION_CLASSES;
//     if (FFlag::LuauCompileFastpcall)
//         return 14;
//     if (FFlag::LuauCompileEmitVectorDouble)
//         return 13;
//     if (FFlag::LuauBytecodeCostModel)
//         return 12;
//     if (FFlag::LuauEmitCallFeedback)
//         return 11;
//     return LBC_VERSION_TARGET;
// }
// ```
//
// `LuauCompileFastpcall` / `LuauCompileEmitVectorDouble` / `LuauBytecodeCostModel`
// 三个分支保留（flag 定义见 `ulua-common/src/fflag.rs`，默认 false）——旧版
// 移植的 `UdataDirect→9 / IntegerType2→8 / DuptableConstantPack2→7` 逐 flag
// 链已被上游吸收进 `LBC_VERSION_TARGET = 9`，逐 flag bump 属过期语义。

impl<'a> BytecodeBuilder<'a> {
  pub(crate) fn get_version(&self) -> u8 {
    if fflag::DebugLuauUserDefinedClasses.get() {
      return LuauBytecodeTag::LBC_VERSION_CLASSES.0 as u8;
    }

    // 上游 cpp `getVersion` 直接回字面量；此处命名收口，版本↔格式增量的
    // 对应关系只在本块声明一次（改 bump 只动一处）。
    const VERSION_FASTPCALL: u8 = 14;
    const VERSION_VECTOR_DOUBLE: u8 = 13;
    const VERSION_COST_MODEL: u8 = 12;
    const VERSION_CALL_FEEDBACK: u8 = 11;

    if fflag::LuauCompileFastpcall.get() {
      return VERSION_FASTPCALL;
    }
    if fflag::LuauCompileEmitVectorDouble.get() {
      return VERSION_VECTOR_DOUBLE;
    }
    if fflag::LuauBytecodeCostModel.get() {
      return VERSION_COST_MODEL;
    }

    if fflag::LuauEmitCallFeedback.get() {
      return VERSION_CALL_FEEDBACK;
    }

    LuauBytecodeTag::LBC_VERSION_TARGET.0 as u8
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_write_class_shape.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub(crate) fn write_class_shape(&self, ss: &mut Vec<u8>, cs: &ClassShape) {
    write_var_int(ss, cs.class_name as u64);
    write_var_int(ss, cs.property_names.len() as u64);
    write_var_int(ss, cs.method_names.len() as u64);

    for &prop_name in &cs.property_names {
      write_var_int(ss, prop_name as u64);
    }

    for &method_name in &cs.method_names {
      write_var_int(ss, method_name as u64);
    }
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_write_function.rs` ──
impl<'a> BytecodeBuilder<'a> {
  /// `cost`：cpp `writeFunction(ss, id, flags, cost)` 的内联开销模型，
  /// LPF_INLINABLE 时以 VarInt 写入（BytecodeBuilder.cpp:1041-1046）。
  pub(crate) fn write_function(&mut self, ss: &mut Vec<u8>, id: u32, flags: u8, cost: u64) {
    LUAU_ASSERT!(id < self.functions.len() as u32);
    let func = &self.functions[id as usize];

    // Header
    write_byte(ss, func.maxstacksize);
    write_byte(ss, func.numparams);
    write_byte(ss, func.numupvalues);
    write_byte(ss, if func.isvararg { 1 } else { 0 });

    write_byte(ss, flags);

    if !func.typeinfo.is_empty() || !self.typed_upvals.is_empty() || !self.typed_locals.is_empty() {
      // collect type info into a temporary string to know the overall size of type data
      self.temp_type_info.clear();
      write_var_int(&mut self.temp_type_info, func.typeinfo.len() as u64);
      write_var_int(&mut self.temp_type_info, self.typed_upvals.len() as u64);
      write_var_int(&mut self.temp_type_info, self.typed_locals.len() as u64);

      self.temp_type_info.extend_from_slice(&func.typeinfo);

      for l in &self.typed_upvals {
        write_byte(&mut self.temp_type_info, l.r#type.0 as u8);
      }

      for l in &self.typed_locals {
        write_byte(&mut self.temp_type_info, l.r#type.0 as u8);
        write_byte(&mut self.temp_type_info, l.reg);
        write_var_int(&mut self.temp_type_info, l.startpc as u64);
        LUAU_ASSERT!(l.endpc >= l.startpc);
        write_var_int(&mut self.temp_type_info, (l.endpc - l.startpc) as u64);
      }

      write_var_int(ss, self.temp_type_info.len() as u64);
      ss.extend_from_slice(&self.temp_type_info);
    } else {
      write_var_int(ss, 0);
    }

    // instructions
    write_var_int(ss, self.insns.len() as u64);

    for &insn in &self.insns {
      write_bytes(ss, &(insn as i32).to_ne_bytes());
    }

    // constants
    write_var_int(ss, self.constants.len() as u64);

    for c in &self.constants {
      match c {
        Constant::Nil => {
          write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_NIL.0 as u8);
        }
        Constant::Boolean(value) => {
          write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_BOOLEAN.0 as u8);
          write_byte(ss, *value as u8);
        }
        Constant::Number(value) => {
          write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_NUMBER.0 as u8);
          write_bytes(ss, &value.to_ne_bytes());
        }
        Constant::Integer(value) => {
          write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_INTEGER.0 as u8);
          if *value < 0 {
            write_byte(ss, 1);
            // C++ `~(uint64_t)value + 1` 即负数的绝对值
            write_var_int(ss, value.unsigned_abs());
          } else {
            write_byte(ss, 0);
            write_var_int(ss, *value as u64);
          }
        }
        Constant::Vector(vec) => {
          write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_VECTOR.0 as u8);
          for &component in vec {
            write_bytes(ss, &component.to_ne_bytes());
          }
        }
        // cpp `BytecodeBuilder.cpp:899-916`：flag 关闭时降级为 4×float 的
        // LBC_CONSTANT_VECTOR，与旧版本字节码兼容。
        Constant::Vectord(vec) => {
          if fflag::LuauCompileEmitVectorDouble.get() {
            write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_VECTORD.0 as u8);
            for &component in vec {
              write_bytes(ss, &component.to_ne_bytes());
            }
          } else {
            write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_VECTOR.0 as u8);
            for &component in vec {
              write_bytes(ss, &(component as f32).to_ne_bytes());
            }
          }
        }
        Constant::String(index) => {
          write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_STRING.0 as u8);
          write_var_int(ss, *index as u64);
        }
        Constant::Import(iid) => {
          write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_IMPORT.0 as u8);
          write_bytes(ss, &(*iid as i32).to_ne_bytes());
        }
        Constant::Table(shape_idx) => {
          let shape = &self.table_shapes[*shape_idx as usize];
          // cpp BytecodeBuilder.cpp:967-980 仅看 has_constants（无 flag 门控）：
          // 两臂只差 tag 与「是否交错写 value」，坍缩为单写路径。
          let has_constants = shape.has_constants;
          write_byte(
            ss,
            if has_constants {
              LuauBytecodeTag::LBC_CONSTANT_TABLE_WITH_CONSTANTS.0 as u8
            } else {
              LuauBytecodeTag::LBC_CONSTANT_TABLE.0 as u8
            },
          );
          write_var_int(ss, shape.length as u64);
          // 定长数组按 length 切片后迭代（§3：免逐下标边界重查）；越界 `length`
          // 的 panic 边界与原 `[i]` 索引完全一致（切片 end 越界即 panic）。
          let n = shape.length as usize;
          for (&k, &c) in shape.keys[..n].iter().zip(&shape.constants[..n]) {
            write_var_int(ss, k as u64);
            if has_constants {
              write_bytes(ss, &c.to_ne_bytes());
            }
          }
        }
        Constant::Closure(fid) => {
          write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_CLOSURE.0 as u8);
          write_var_int(ss, *fid as u64);
        }
        Constant::ClassShape(shape_idx) => {
          write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_CLASS_SHAPE.0 as u8);
          let cs = &self.class_shapes[*shape_idx as usize];
          self.write_class_shape(ss, cs);
        }
      }
    }

    // child protos
    write_var_int(ss, self.protos.len() as u64);

    for &child in &self.protos {
      write_var_int(ss, child as u64);
    }

    // debug info
    write_var_int(ss, func.debuglinedefined as u64);
    write_var_int(ss, func.debugname as u64);

    let has_lines = self.lines.iter().all(|&line| line != 0);

    if has_lines {
      write_byte(ss, 1);

      self.write_line_info(ss);
    } else {
      write_byte(ss, 0);
    }

    let has_debug = !self.debug_locals.is_empty() || !self.debug_upvals.is_empty();

    if has_debug {
      write_byte(ss, 1);

      write_var_int(ss, self.debug_locals.len() as u64);

      for l in &self.debug_locals {
        write_var_int(ss, l.name as u64);
        write_var_int(ss, l.startpc as u64);
        write_var_int(ss, l.endpc as u64);
        write_byte(ss, l.reg);
      }

      write_var_int(ss, self.debug_upvals.len() as u64);

      for l in &self.debug_upvals {
        write_var_int(ss, l.name as u64);
      }
    } else {
      write_byte(ss, 0);
    }

    if fflag::LuauEmitCallFeedback.get() {
      // Feedback Slots
      write_var_int(ss, self.fb_slots.len() as u64);
      for &pc in &self.fb_slots {
        write_byte(ss, LuauFeedbackType::LFT_CALLTARGET as u8);
        write_var_int(ss, pc as u64);
      }
    } else if cost_section_enabled() {
      // cpp: `cpp/Bytecode/src/BytecodeBuilder.cpp:1036-1039` — Empty feedback vector
      write_var_int(ss, 0);
    }

    // cpp BytecodeBuilder.cpp:1041-1046 — LPF_INLINABLE 时写内联开销模型
    if cost_section_enabled() && (flags & LuauProtoFlag::LPF_INLINABLE as u8) != 0 {
      write_var_int(ss, cost);
    }
  }
}

/// feedback/cost 尾段随带的格式开关组合：cpp 同条件在 `writeFunction` 的
/// 1036 与 1041 两处、以及 `finalize` 的 797 处（函数体长度前缀）出现，
/// 此处收敛为单一谓词（任一开启即写入空 feedback 向量与 LPF_INLINABLE 的
/// cost VarInt，finalize 侧逐函数写长度前缀）。
pub(crate) fn cost_section_enabled() -> bool {
  fflag::LuauBytecodeCostModel.get()
    || fflag::LuauCompileEmitVectorDouble.get()
    || fflag::LuauCompileFastpcall.get()
    || fflag::DebugLuauUserDefinedClasses.get()
}

// ── abs-r139：并自 `methods/bytecode_builder_write_line_info.rs` ──
/// line-info 分组跨度初值（覆盖全表的足够大跨度）。
const INITIAL_SPAN: usize = 1 << 24;
/// 行号与组内基线之差的最大值（单字节增量编码）。
const MAX_LINE_DELTA: i32 = 255;

impl<'a> BytecodeBuilder<'a> {
  pub(crate) fn write_line_info(&self, ss: &mut Vec<u8>) {
    LUAU_ASSERT!(!self.lines.is_empty());

    let mut span = INITIAL_SPAN;

    // offset 是行号表的扫描位置（窗口推进量 = span，属数据语义），外层保留下标游走。
    // 内层定窗扫描改为切片顺序迭代：`advance` 为首个使 max-min 超出单字节增量的
    // 元素下标（与 cpp 在违例元素处 break、游标停在其位置一致），无违例则为窗口长。
    // min/max 只服务于此终止判定，编码期基线另由 chunks().min() 重算。
    let mut offset = 0;
    while offset < self.lines.len() {
      let window = &self.lines[offset..offset + span.min(self.lines.len() - offset)];
      let (mut min, mut max) = (window[0], window[0]);
      let mut advance = window.len();
      for (k, &line) in window.iter().enumerate() {
        min = min.min(line);
        max = max.max(line);
        if max - min > MAX_LINE_DELTA {
          advance = k;
          break;
        }
      }
      let next = offset + advance;

      if next < self.lines.len() && next - offset < span {
        span = 1 << log2((next - offset) as i32);
      }
      // C++ `for (...; offset += span)`：收缩后也按新 span 推进，
      // 与 calcLinesSpan 的扫描节奏一致，保证最终 span 与 C++ 输出逐字节一致
      offset += span;
    }

    let mut baseline_one = 0;
    let mut baseline_scratch = Vec::new();
    let baseline_size = (self.lines.len() - 1) / span + 1;

    if baseline_size > 1 {
      baseline_scratch.resize(baseline_size, 0);
    }

    let baseline = if baseline_size > 1 {
      &mut baseline_scratch
    } else {
      slice::from_mut(&mut baseline_one)
    };

    // 按 span 窗口分块求每窗口最小行号；chunks 保证每块非空
    for (window, chunk) in self.lines.chunks(span).enumerate() {
      baseline[window] = chunk.iter().copied().min().unwrap();
    }

    let logspan = log2(span as i32);
    write_byte(ss, logspan as u8);

    let mut last_offset = 0u8;
    for (i, &line) in self.lines.iter().enumerate() {
      let delta = line - baseline[i >> logspan];
      LUAU_ASSERT!((0..=MAX_LINE_DELTA).contains(&delta));

      write_byte(ss, (delta as u8).wrapping_sub(last_offset));
      last_offset = delta as u8;
    }

    let mut last_line = 0;
    for &line in &baseline[..baseline_size] {
      write_bytes(ss, &(line - last_line).to_ne_bytes());
      last_line = line;
    }
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_write_string_table.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub(crate) fn write_string_table(&self, ss: &mut Vec<u8>) {
    let count = self.string_table.size();
    // 槽位存共享引用而非 clone：序列化只读不改，省去每个条目一次堆拷贝；
    // 未落位槽按空串写出（与原 clone(default) 占位行为一致）。
    let empty = StringRef::default();
    let mut strings: Vec<Option<&StringRef<'a>>> = vec![None; count];

    for (string_ref, &index) in self.string_table.iter() {
      LUAU_ASSERT!(index > 0 && (index as usize) <= strings.len());
      strings[index as usize - 1] = Some(string_ref);
    }

    write_var_int(ss, strings.len() as u64);

    for slot in strings {
      let s = slot.unwrap_or(&empty);
      write_var_int(ss, s.len() as u64);
      // 字符串表条目为原始字节，Vec<u8> 直接追加，无 UTF-8 约束。
      ss.extend_from_slice(s.as_bytes());
    }
  }
}
