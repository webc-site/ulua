//! `BytecodeBuilder` 之 反汇编与 remark 展示通道（dump_*）。

use core::{cmp::min, fmt::Arguments};
use std::{string::String, vec, vec::Vec};

use ulua_common::{
  enums::{
    luau_bytecode_type::LuauBytecodeType, luau_capture_type::LuauCaptureType,
    luau_opcode::LuauOpcode,
  },
  fflag,
  functions::{
    format_append::format_append,
    format_g::{format_g, format_g_append_vector},
    get_jump_target::get_jump_target,
  },
  macros::luau_assert::LUAU_ASSERT,
  records::instruction::Instruction,
};

use super::{BytecodeBuilder, K_INVALID_REG, insn};
use crate::{
  enums::{dump_flags::DumpFlags, r#type::Type},
  functions::{
    get_base_type_string::get_base_type_string, log_2::ceillog2,
    printable_string_constant::printable_string_constant,
  },
  methods::bytecode_builder_get_string_hash::bytecode_builder_get_string_hash_slice,
  records::{constant::Constant, string_ref::StringRef},
};

// ── abs-r139：并自 `methods/bytecode_builder_add_debug_remark.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn add_debug_remark(&mut self, args: Arguments<'_>) {
    if !DumpFlags::Remarks.is_set(self.dump_flags) {
      return;
    }

    let offset = self.debug_remark_buffer.len();

    // C++ `addDebugRemark(const char* format, ...)` printf-formats the whole remark.
    // Rust has no printf, so callers pass the fully-rendered message as a single
    // `format_args!(...)` (the C-style `%d`/`%.2f` become `{}`/`{:.2}`).
    format_append(&mut self.debug_remark_buffer, args);

    // 定界即区间：本条 remark 恰为 `[offset..now]`，直接截出文本，免二次 NUL 扫描
    let remark_str = self.debug_remark_buffer[offset..].to_string();

    // we null-terminate all remarks to avoid storing remark length
    self.debug_remark_buffer.push('\0');

    self
      .debug_remarks
      .push((self.insns.len() as u32, offset as u32));

    self.dump_remarks.push((self.debug_line, remark_str));
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_annotate_instruction.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn annotate_instruction(&self, result: &mut String, fid: u32, instpos: u32) {
    if !DumpFlags::Code.is_set(self.dump_flags) {
      return;
    }

    LUAU_ASSERT!(fid < self.functions.len() as u32);

    let function = &self.functions[fid as usize];
    let (dump, dumpinstoffs) = (&function.dump, &function.dumpinstoffs);

    let next = instpos + 1;

    LUAU_ASSERT!(next < dumpinstoffs.len() as u32);

    // Skip locations of multi-dword instructions（定位下一个非 -1 偏移）
    let next = dumpinstoffs[instpos as usize + 1..]
      .iter()
      .position(|&off| off != -1)
      .map_or(dumpinstoffs.len() as u32, |k| instpos + 1 + k as u32);

    let start_offset = dumpinstoffs[instpos as usize] as usize;
    let end_offset = dumpinstoffs[next as usize] as usize;

    // cpp `formatAppend(result, "%.*s", len, dump.data() + start)`：定长字节追加，
    // Rust 直接 push_str，免过格式化机
    result.push_str(&dump[start_offset..end_offset]);
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_dump_constant.rs` ──
/// 常量串 dump 的截断长度（超出部分以 `...` 省略）。
const K_MAX_DUMPED_STRING_LEN: usize = 32;

impl<'a> BytecodeBuilder<'a> {
  /// 取 import 段的字符串常量。误用防护：`idx` 指向的常量在构建时已注册为
  /// String 且索引非 0（入口断言兜底）。
  fn import_segment_str(&self, idx: i32) -> &StringRef<'a> {
    let str_idx = self.constants[idx as usize].as_string();
    LUAU_ASSERT!(str_idx as usize <= self.debug_strings.len());
    &self.debug_strings[str_idx as usize - 1]
  }

  pub(crate) fn dump_constant(&self, result: &mut String, k: i32, detailed: bool) {
    LUAU_ASSERT!((k as u32) < self.constants.len() as u32);
    let data = &self.constants[k as usize];

    match data {
      Constant::Nil => result.push_str("nil"),
      Constant::Boolean(value) => result.push_str(if *value { "true" } else { "false" }),
      Constant::Number(value) => result.push_str(&format_g(*value, 17)),
      Constant::Integer(value) => format_append(result, format_args!("{value}")),
      // cpp `BytecodeBuilder.cpp` 两分支同构：`%.9g` 打 float 分量，截断判定
      // `v[3] == 0`（f32→f64 提升精确，与原 f32 判定等价）
      Constant::Vector(v) => {
        format_g_append_vector(result, &v.map(f64::from), |x| format_g(x, 9));
      }
      // cpp `BytecodeBuilder.cpp:2353-2377`：flag 开时按 %.17g 打双精度，
      // 关时先转 float 再按 %.9g 打（与 writeFunction 的降级路径一致）。
      // 三分量截断只看原始 `v[3] == 0`（helper 内先判后打，变换仅作用于
      // 打印值），与 flag 无关。
      Constant::Vectord(v) => {
        let wide = fflag::LuauCompileEmitVectorDouble.get();
        format_g_append_vector(result, v, |x| {
          format_g(
            if wide { x } else { f64::from(x as f32) },
            if wide { 17 } else { 9 },
          )
        });
      }
      Constant::String(str_idx) => {
        let str = &self.debug_strings[*str_idx as usize - 1];
        let bytes = str.as_bytes();
        if printable_string_constant(bytes) {
          // 显示路径用 lossy 容错；printable 分支均为合法 UTF-8，输出无损
          let s = str.to_string_lossy();
          let limit = min(s.len(), K_MAX_DUMPED_STRING_LEN);
          format_append(result, format_args!("'{:.*}'", limit, s));
          if str.len() >= K_MAX_DUMPED_STRING_LEN {
            result.push_str("...");
          }
        } else {
          result.push('\'');
          for &b in &bytes[..min(str.len(), K_MAX_DUMPED_STRING_LEN)] {
            if b < b' ' {
              format_append(result, format_args!("\\x{:02X}", b));
            } else {
              result.push(b as char);
            }
          }
          if str.len() >= K_MAX_DUMPED_STRING_LEN {
            result.push_str("'...");
          } else {
            result.push('\'');
          }
        }
      }
      Constant::Import(iid) => {
        let (count, id0, id1, id2) = BytecodeBuilder::decompose_import_id(*iid);
        for (i, &id) in [id0, id1, id2][..count as usize].iter().enumerate() {
          if i > 0 {
            result.push('.');
          }
          result.push_str(&self.import_segment_str(id).to_string_lossy());
        }
      }
      Constant::Table(shape_idx) => {
        if detailed {
          let shape = &self.table_shapes[*shape_idx as usize];
          let sizenode = if shape.length > 0 {
            1u32 << (ceillog2(shape.length as i32) as u32)
          } else {
            0u32
          };
          let mask = if sizenode > 0 { sizenode - 1 } else { 0u32 };

          let mut slots = vec![0u32; shape.length as usize];
          let mut slot_owner = vec![!0u32; sizenode as usize];

          for (i, &key_idx) in shape.keys.iter().enumerate().take(shape.length as usize) {
            let key_const = &self.constants[key_idx as usize];
            LUAU_ASSERT!(key_const.kind() == Type::String && key_const.as_string() != 0);
            let str = &self.debug_strings[key_const.as_string() as usize - 1];
            let hash = bytecode_builder_get_string_hash_slice(str.as_bytes());
            slots[i] = hash & mask;

            if slot_owner[slots[i] as usize] == !0u32 {
              slot_owner[slots[i] as usize] = i as u32;
            }
          }

          result.push('{');

          for (i, &key_idx) in shape.keys.iter().enumerate().take(shape.length as usize) {
            if i > 0 {
              result.push_str(", ");
            }

            result.push('[');
            self.dump_constant(result, key_idx, false);
            result.push(']');

            if shape.has_constants && shape.constants[i] != -1 {
              result.push_str(" = ");
              self.dump_constant(result, shape.constants[i], false);
            }

            format_append(result, format_args!(" #{}", slots[i]));

            if slot_owner[slots[i] as usize] != i as u32 {
              result.push_str(" (conflict)");
            }
          }

          format_append(result, format_args!("}} sizenode={sizenode}"));
        } else {
          result.push_str("{...}");
        }
      }
      Constant::Closure(fid) => {
        let func = &self.functions[*fid as usize];
        // detailed 模式下输出 "function"/"function <名>"，非 detailed 输出 "'<名>'"
        if detailed {
          result.push_str("function");
          if !func.dumpname.is_empty() {
            result.push(' ');
            result.push_str(&func.dumpname);
          }
        } else if !func.dumpname.is_empty() {
          format_append(result, format_args!("'{}'", func.dumpname));
        }
      }
      Constant::ClassShape(shape_idx) => {
        let cs = &self.class_shapes[*shape_idx as usize];
        let class_name_const = &self.constants[cs.class_name as usize];
        LUAU_ASSERT!(
          class_name_const.kind() == Type::String
            && class_name_const.as_string() as usize <= self.debug_strings.len()
        );
        let str = &self.debug_strings[class_name_const.as_string() as usize - 1];
        LUAU_ASSERT!(printable_string_constant(str.as_bytes()));
        format_append(
          result,
          format_args!(
            "class {} (props: {}, methods: {})",
            str.to_string_lossy(),
            cs.property_names.len(),
            cs.method_names.len()
          ),
        );

        // detailed 模式追加 props/methods 的常量索引清单（两族同构，单源迭代）
        if detailed {
          for (label, keys) in [("props", &cs.property_names), ("methods", &cs.method_names)] {
            if keys.is_empty() {
              continue;
            }
            format_append(result, format_args!("\n  {label}:"));
            for &k in keys {
              format_append(result, format_args!("\n    K{} [", k));
              self.dump_constant(result, k, false);
              result.push(']');
            }
          }
        }
      }
    }
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_dump_current_function.rs` ──
impl<'a> BytecodeBuilder<'a> {
  /// 类型名 + 可选 '?' 后缀（userdata 名优先，回落到基础类型名）。
  /// C++ `name = userdata ? userdata : getBaseTypeString(et)`。
  fn type_name_and_optional(&self, ty: LuauBytecodeType) -> (&str, &'static str) {
    let name = self
      .try_get_userdata_type_name(ty)
      .unwrap_or_else(|| get_base_type_string(ty.0 as u8));
    let optional = if (ty.0 & LuauBytecodeType::LBC_TYPE_OPTIONAL_BIT.0) != 0 {
      "?"
    } else {
      ""
    };
    (name, optional)
  }

  /// 反汇编当前函数：返回 `(dump 文本, 每条指令的文本起始偏移表)`。
  ///
  /// 偏移表长 `insns.len() + 1`（末项为文本总长），非 Code 段填充 `-1`；
  /// 由 `end_function` 写回所属 `Function`，供 `annotate_instruction` 定位指令文本区间。
  pub(crate) fn dump_current_function(&self) -> (String, Vec<i32>) {
    // 只读 dump 开关下的空产出：偏移表在 Code 分支内按需重建，此处给空表。
    if (self.dump_flags & (DumpFlags::Code | DumpFlags::Constants)) == 0 {
      return (String::new(), Vec::new());
    }

    let mut dumpinstoffs: Vec<i32> = Vec::new();

    let mut last_line = -1;
    let mut result = String::new();

    if DumpFlags::Locals.is_set(self.dump_flags) {
      for (i, l) in self.debug_locals.iter().enumerate() {
        if l.startpc == l.endpc {
          LUAU_ASSERT!(l.startpc < self.lines.len() as u32);

          format_append(
            &mut result,
            format_args!(
              "local {}: reg {}, start pc {} line {}, no live range\n",
              i, l.reg, l.startpc, self.lines[l.startpc as usize]
            ),
          );
        } else {
          LUAU_ASSERT!(l.startpc < l.endpc);
          LUAU_ASSERT!(l.startpc < self.lines.len() as u32);
          LUAU_ASSERT!(l.endpc <= self.lines.len() as u32);

          format_append(
            &mut result,
            format_args!(
              "local {}: reg {}, start pc {} line {}, end pc {} line {}\n",
              i,
              l.reg,
              l.startpc,
              self.lines[l.startpc as usize],
              l.endpc - 1,
              self.lines[(l.endpc - 1) as usize]
            ),
          );
        }
      }
    }

    if DumpFlags::Types.is_set(self.dump_flags) {
      // cpp `functions.back().typeinfo`：空表时 release 下是 UB。dump 属调试通道，
      // 按断言 + 非 panic 读法收敛：无函数时视作空类型信息，不再解包 panic。
      // 空兜底用 `const &[u8]`（编译期进 rodata）：`Vec` 含 Drop 不能作为
      // const 初始化 static，函数内 `static Vec` 每次访问都过线程安全惰性
      // 门闩，此处改切片即零门闩。
      LUAU_ASSERT!(!self.functions.is_empty());
      const NO_TYPEINFO: &[u8] = &[];
      let typeinfo_bytes = self
        .functions
        .last()
        .map_or(NO_TYPEINFO, |f| f.typeinfo.as_slice());

      if typeinfo_bytes.len() >= 2 {
        for (i, &et) in typeinfo_bytes[2..].iter().enumerate() {
          let (name, optional) = self.type_name_and_optional(LuauBytecodeType(et as u16));
          format_append(
            &mut result,
            format_args!("R{}: {}{} [argument]\n", i, name, optional),
          );
        }
      }

      for (i, l) in self.typed_upvals.iter().enumerate() {
        let (name, optional) = self.type_name_and_optional(l.r#type);
        format_append(&mut result, format_args!("U{}: {}{}\n", i, name, optional));
      }

      for l in &self.typed_locals {
        let (name, optional) = self.type_name_and_optional(l.r#type);
        format_append(
          &mut result,
          format_args!(
            "R{}: {}{} from {} to {}\n",
            l.reg, name, optional, l.startpc, l.endpc
          ),
        );
      }
    }

    if DumpFlags::Constants.is_set(self.dump_flags) {
      for (i, _) in self.constants.iter().enumerate() {
        format_append(&mut result, format_args!("K{}: ", i));
        self.dump_constant(&mut result, i as i32, true);
        result.push('\n');
      }
    }

    if DumpFlags::Code.is_set(self.dump_flags) {
      let insns_slice = self.instructions();
      let mut labels = vec![-1; insns_slice.len()];

      // 第一遍：标记跳转目标槽位（变步长遍历，见 `scan::stepped`）
      for (i, _end, insn) in super::scan::stepped(insns_slice, 0) {
        let target = get_jump_target(insn.raw(), i as u32);

        if target >= 0 {
          LUAU_ASSERT!((target as usize) < insns_slice.len());
          labels[target as usize] = 0;
        }
      }

      let mut next_label = 0;

      for label in &mut labels {
        if *label == 0 {
          *label = next_label;
          next_label += 1;
        }
      }

      dumpinstoffs.resize(insns_slice.len() + 1, -1);

      let mut remarks = self.debug_remarks.iter().copied().peekable();

      for (i, _end, insn) in super::scan::stepped(insns_slice, 0) {
        dumpinstoffs[i] = result.len() as i32;

        // C++: `if (op == LOP_PREPVARARGS) { i++; continue; }` — the vararg
        // prologue is a call-dispatch Header with no "interesting" info and
        // is never disassembled. (The prior `op == 32` was a mistranslated
        // literal; LOP_PREPVARARGS is 65, so the skip never fired and the
        // Header reached `dump_instruction`'s unsupported-opcode assert.)
        if insn.opcode() == Some(LuauOpcode::LOP_PREPVARARGS) {
          continue;
        }

        if DumpFlags::Remarks.is_set(self.dump_flags) {
          // 归并输出起始于当前指令位置的 remark
          while let Some((_, remark_start)) = remarks.next_if(|(pos, _)| *pos == i as u32) {
            // C++ reads `debugRemarkBuffer.c_str() + offset` — a C-string that stops
            // at the null terminator. remark_end points at the *next* remark (past this
            // remark's '\0'), so slice only up to the terminator.
            let remark_end = remarks
              .peek()
              .map_or(self.debug_remark_buffer.len(), |&(_, end)| end as usize);
            let remark_str = &self.debug_remark_buffer[remark_start as usize..remark_end];
            let remark_str = remark_str.split('\0').next().unwrap_or("");
            format_append(&mut result, format_args!("REMARK {}\n", remark_str));
          }
        }

        if DumpFlags::Source.is_set(self.dump_flags) {
          let line = self.lines[i];

          if line > 0 && line != last_line {
            LUAU_ASSERT!(((line - 1) as usize) < self.dump_source.len());
            format_append(
              &mut result,
              format_args!("{:5}: {}\n", line, self.dump_source[(line - 1) as usize]),
            );
            last_line = line;
          }
        }

        if DumpFlags::Lines.is_set(self.dump_flags) {
          format_append(&mut result, format_args!("{}: ", self.lines[i]));
        }

        if labels[i] != -1 {
          format_append(&mut result, format_args!("L{}: ", labels[i]));
        }

        let target = get_jump_target(insn.raw(), i as u32);
        let target_label = if target >= 0 {
          labels[target as usize]
        } else {
          -1
        };

        // Pass the full remaining instruction stream (not just one word):
        // multi-word ops (LOADKX, GETIMPORT, FASTCALL2K, NEWCLASSMEMBER,
        // CMPPROTO, …) read their aux word via `code[1]`, which would be
        // out of bounds on a length-1 slice. C++ passes a bare pointer.
        self.dump_instruction(&insns_slice[i..], &mut result, target_label);
      }

      dumpinstoffs[insns_slice.len()] = result.len() as i32;
    }

    (result, dumpinstoffs)
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_dump_everything.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn dump_everything(&self) -> String {
    let mut result = String::new();

    for (i, function) in self.functions.iter().enumerate() {
      let debugname = if function.dumpname.is_empty() {
        "??"
      } else {
        &function.dumpname
      };

      format_append(
        &mut result,
        format_args!("Function {} ({}):\n", i as i32, debugname),
      );

      result.push_str(&function.dump);
      result.push('\n');
    }

    result
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_dump_function.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn dump_function(&self, id: u32) -> String {
    LUAU_ASSERT!(id < self.functions.len() as u32);
    self.functions[id as usize].dump.clone()
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_dump_instruction.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub(crate) fn dump_instruction(
    &self,
    code: &[Instruction],
    result: &mut String,
    target_label: i32,
  ) -> usize {
    let insn = code[0];
    let op_enum = insn.luau_opcode();

    // 寄存器与操作数预取：const fn 位截取，无副作用，绑定后供下方各指令臂复用
    let a = insn.a() as u32;
    let b = insn.b() as u32;
    let c = insn.c() as u32;
    let d = insn.d() as i32;

    // K 常量内联样板：前缀格式串以 " [" 结尾，随后 dump_constant 输出常量体、"]\n" 收尾，块值为指令宽度。
    macro_rules! k_inline {
      ($fmt:literal, $($arg:expr),+ $(,)? ; $k:expr, $width:expr $(,)?) => {{
        format_append(result, format_args!($fmt, $($arg),+));
        self.dump_constant(result, $k, false);
        result.push_str("]\n");
        $width
      }};
    }

    // 单寄存器样板：<OP> R{a}
    macro_rules! r1 {
      ($op:literal) => {{
        format_append(result, format_args!(concat!($op, " R{}\n"), a));
        1
      }};
    }

    // 双寄存器样板：<OP> R{a} R{b}
    macro_rules! r2 {
      ($op:literal) => {{
        format_append(result, format_args!(concat!($op, " R{} R{}\n"), a, b));
        1
      }};
    }

    // 三寄存器运算样板：<OP> R{a} R{b} R{c}
    macro_rules! r3 {
      ($op:literal) => {{
        format_append(
          result,
          format_args!(concat!($op, " R{} R{} R{}\n"), a, b, c),
        );
        1
      }};
    }

    // 二元 K 运算样板：<OP> R{a} R{b} K{c} [常量体]（ADDK/SUBK 等共用）
    macro_rules! rk_binary {
      ($op:literal) => {{
        format_append(
          result,
          format_args!(concat!($op, " R{} R{} K{} ["), a, b, c),
        );
        self.dump_constant(result, c as i32, false);
        result.push_str("]\n");
        1
      }};
    }

    // 表属性 KS 样板：<OP> R{a} R{b} K{aux} [常量体]（GETTABLEKS/SETTABLEKS/NAMECALL 共用）
    macro_rules! table_ks {
      ($op:literal) => {{
        format_append(
          result,
          format_args!(concat!($op, " R{} R{} K{} ["), a, b, code[1]),
        );
        self.dump_constant(result, code[1].raw() as i32, false);
        result.push_str("]\n");
        2
      }};
    }

    // 表下标 TABLEN 样板：<OP> R{a} R{b} {c+1}
    macro_rules! table_n {
      ($op:literal) => {{
        format_append(
          result,
          format_args!(concat!($op, " R{} R{} {}\n"), a, b, c + 1),
        );
        1
      }};
    }

    // 上值操作样板：<OP> R{a} {b}
    macro_rules! r_upval {
      ($op:literal) => {{
        format_append(result, format_args!(concat!($op, " R{} {}\n"), a, b));
        1
      }};
    }

    // 反向 K 运算样板：<OP> R{a} K{b} [常量体] R{c}（SUBRK/DIVRK 共用）
    macro_rules! rk_reverse {
      ($op:literal) => {{
        format_append(result, format_args!(concat!($op, " R{} K{} ["), a, b));
        self.dump_constant(result, b as i32, false);
        format_append(result, format_args!("] R{}\n", c));
        1
      }};
    }

    // 比较跳转样板：<OP> R{a} R{b} L{target}，宽度 2
    macro_rules! jump_cmp {
      ($op:literal) => {{
        format_append(
          result,
          format_args!(concat!($op, " R{} R{} L{}\n"), a, code[1], target_label),
        );
        2
      }};
    }

    // 单寄存器条件跳转样板：<OP> R{a} L{target}，宽度 1
    macro_rules! jump_cond {
      ($op:literal) => {{
        format_append(
          result,
          format_args!(concat!($op, " R{} L{}\n"), a, target_label),
        );
        1
      }};
    }

    // 纯 Label 跳转样板：<OP> L{target}，宽度 1
    macro_rules! jump_label {
      ($op:literal) => {{
        format_append(result, format_args!(concat!($op, " L{}\n"), target_label));
        1
      }};
    }

    // 条件跳转常量样板：<OP> R{a} K{aux_mask} L{target}{invert} [常量体]，宽度 2
    macro_rules! jump_xeq_k {
      ($op:literal) => {{
        let k = code[1].aux_kv();
        format_append(
          result,
          format_args!(
            concat!($op, " R{} K{} L{}{} ["),
            a,
            k,
            target_label,
            insn::invert_label(code[1].raw()),
          ),
        );
        self.dump_constant(result, k as i32, false);
        result.push_str("]\n");
        2
      }};
    }

    // UDATA KS 族样板：<OP> R{a} R{b} K{aux16} [常量体]，宽度 2
    macro_rules! udata_ks {
      ($op:literal) => {{
        let k = code[1].aux_kv16() as u32;
        format_append(
          result,
          format_args!(concat!($op, " R{} R{} K{} ["), a, b, k),
        );
        self.dump_constant(result, k as i32, false);
        result.push_str("]\n");
        2
      }};
    }

    match op_enum {
      LuauOpcode::LOP_LOADNIL => r1!("LOADNIL"),
      LuauOpcode::LOP_LOADB => {
        if c != 0 {
          format_append(result, format_args!("LOADB R{a} {b} +{c}\n"));
        } else {
          format_append(result, format_args!("LOADB R{a} {b}\n"));
        }
        1
      }
      LuauOpcode::LOP_LOADN => {
        format_append(result, format_args!("LOADN R{a} {d}\n"));
        1
      }
      LuauOpcode::LOP_LOADK => k_inline!("LOADK R{} K{} [", a, d; d, 1),
      LuauOpcode::LOP_MOVE => r2!("MOVE"),
      LuauOpcode::LOP_GETGLOBAL => {
        k_inline!("GETGLOBAL R{} K{} [", a, code[1]; code[1].raw() as i32, 2)
      }
      LuauOpcode::LOP_SETGLOBAL => {
        k_inline!("SETGLOBAL R{} K{} [", a, code[1]; code[1].raw() as i32, 2)
      }
      LuauOpcode::LOP_GETUPVAL => r_upval!("GETUPVAL"),
      LuauOpcode::LOP_SETUPVAL => r_upval!("SETUPVAL"),
      LuauOpcode::LOP_CLOSEUPVALS => r1!("CLOSEUPVALS"),
      LuauOpcode::LOP_GETIMPORT => k_inline!("GETIMPORT R{} {} [", a, d; d, 2),
      LuauOpcode::LOP_GETTABLE => r3!("GETTABLE"),
      LuauOpcode::LOP_SETTABLE => r3!("SETTABLE"),
      LuauOpcode::LOP_GETTABLEKS => table_ks!("GETTABLEKS"),
      LuauOpcode::LOP_SETTABLEKS => table_ks!("SETTABLEKS"),
      LuauOpcode::LOP_GETTABLEN => table_n!("GETTABLEN"),
      LuauOpcode::LOP_SETTABLEN => table_n!("SETTABLEN"),
      LuauOpcode::LOP_NEWCLOSURE => {
        format_append(result, format_args!("NEWCLOSURE R{a} P{d}\n"));
        1
      }
      LuauOpcode::LOP_NAMECALL => table_ks!("NAMECALL"),
      LuauOpcode::LOP_CALL => {
        format_append(
          result,
          format_args!("CALL R{a} {} {}\n", b as i32 - 1, c as i32 - 1),
        );
        1
      }
      LuauOpcode::LOP_CALLFB => {
        format_append(
          result,
          format_args!(
            "CALLFB R{a} {} {} [{}]\n",
            b as i32 - 1,
            c as i32 - 1,
            code[1].raw() as i32
          ),
        );
        2
      }
      LuauOpcode::LOP_RETURN => {
        format_append(result, format_args!("RETURN R{a} {}\n", b as i32 - 1));
        1
      }
      LuauOpcode::LOP_JUMP => jump_label!("JUMP"),
      LuauOpcode::LOP_JUMPIF => jump_cond!("JUMPIF"),
      LuauOpcode::LOP_JUMPIFNOT => jump_cond!("JUMPIFNOT"),
      LuauOpcode::LOP_JUMPIFEQ => jump_cmp!("JUMPIFEQ"),
      LuauOpcode::LOP_JUMPIFLE => jump_cmp!("JUMPIFLE"),
      LuauOpcode::LOP_JUMPIFLT => jump_cmp!("JUMPIFLT"),
      LuauOpcode::LOP_JUMPIFNOTEQ => jump_cmp!("JUMPIFNOTEQ"),
      LuauOpcode::LOP_JUMPIFNOTLE => jump_cmp!("JUMPIFNOTLE"),
      LuauOpcode::LOP_JUMPIFNOTLT => jump_cmp!("JUMPIFNOTLT"),
      LuauOpcode::LOP_ADD => r3!("ADD"),
      LuauOpcode::LOP_SUB => r3!("SUB"),
      LuauOpcode::LOP_MUL => r3!("MUL"),
      LuauOpcode::LOP_DIV => r3!("DIV"),
      LuauOpcode::LOP_IDIV => r3!("IDIV"),
      LuauOpcode::LOP_MOD => r3!("MOD"),
      LuauOpcode::LOP_POW => r3!("POW"),
      LuauOpcode::LOP_ADDK => rk_binary!("ADDK"),
      LuauOpcode::LOP_SUBK => rk_binary!("SUBK"),
      LuauOpcode::LOP_MULK => rk_binary!("MULK"),
      LuauOpcode::LOP_DIVK => rk_binary!("DIVK"),
      LuauOpcode::LOP_IDIVK => rk_binary!("IDIVK"),
      LuauOpcode::LOP_MODK => rk_binary!("MODK"),
      LuauOpcode::LOP_POWK => rk_binary!("POWK"),
      LuauOpcode::LOP_SUBRK => rk_reverse!("SUBRK"),
      LuauOpcode::LOP_DIVRK => rk_reverse!("DIVRK"),
      LuauOpcode::LOP_AND => r3!("AND"),
      LuauOpcode::LOP_OR => r3!("OR"),
      LuauOpcode::LOP_ANDK => rk_binary!("ANDK"),
      LuauOpcode::LOP_ORK => rk_binary!("ORK"),
      LuauOpcode::LOP_CONCAT => r3!("CONCAT"),
      LuauOpcode::LOP_NOT => r2!("NOT"),
      LuauOpcode::LOP_MINUS => r2!("MINUS"),
      LuauOpcode::LOP_LENGTH => r2!("LENGTH"),
      LuauOpcode::LOP_NEWTABLE => {
        format_append(
          result,
          format_args!(
            "NEWTABLE R{a} {} {}\n",
            if b == 0 { 0 } else { 1 << (b as i32 - 1) },
            code[1]
          ),
        );
        2
      }
      LuauOpcode::LOP_DUPTABLE => {
        format_append(result, format_args!("DUPTABLE R{a} {d}\n"));
        1
      }
      LuauOpcode::LOP_SETLIST => {
        format_append(
          result,
          format_args!("SETLIST R{a} R{b} {} [{}]\n", c as i32 - 1, code[1]),
        );
        2
      }
      LuauOpcode::LOP_FORNPREP => jump_cond!("FORNPREP"),
      LuauOpcode::LOP_FORNLOOP => jump_cond!("FORNLOOP"),
      LuauOpcode::LOP_FORGPREP => jump_cond!("FORGPREP"),
      LuauOpcode::LOP_FORGLOOP => {
        format_append(
          result,
          format_args!(
            "FORGLOOP R{} L{} {}{}\n",
            a,
            target_label,
            code[1].aux_a(),
            if (code[1].raw() as i32) < 0 {
              " [inext]"
            } else {
              ""
            }
          ),
        );
        2
      }
      LuauOpcode::LOP_FORGPREP_INEXT => jump_cond!("FORGPREP_INEXT"),
      LuauOpcode::LOP_FORGPREP_NEXT => jump_cond!("FORGPREP_NEXT"),
      LuauOpcode::LOP_GETVARARGS => {
        format_append(result, format_args!("GETVARARGS R{a} {}\n", b as i32 - 1));
        1
      }
      LuauOpcode::LOP_DUPCLOSURE => k_inline!("DUPCLOSURE R{} K{} [", a, d; d, 1),
      LuauOpcode::LOP_BREAK => {
        result.push_str("BREAK\n");
        1
      }
      LuauOpcode::LOP_JUMPBACK => jump_label!("JUMPBACK"),
      LuauOpcode::LOP_LOADKX => k_inline!("LOADKX R{} K{} [", a, code[1]; code[1].raw() as i32, 2),
      LuauOpcode::LOP_JUMPX => jump_label!("JUMPX"),
      LuauOpcode::LOP_FASTCALL => {
        format_append(result, format_args!("FASTCALL {a} L{target_label}\n"));
        1
      }
      LuauOpcode::LOP_FASTCALL1 => {
        format_append(result, format_args!("FASTCALL1 {a} R{b} L{target_label}\n"));
        1
      }
      LuauOpcode::LOP_FASTCALL2 => {
        format_append(
          result,
          format_args!("FASTCALL2 {a} R{b} R{} L{target_label}\n", code[1].aux_a()),
        );
        2
      }
      LuauOpcode::LOP_FASTCALL2K => {
        k_inline!(
          "FASTCALL2K {} R{} K{} L{} [",
          a,
          b,
          code[1],
          target_label;
          code[1].raw() as i32,
          2
        )
      }
      LuauOpcode::LOP_FASTCALL3 => {
        format_append(
          result,
          format_args!(
            "FASTCALL3 {} R{} R{} R{} L{}\n",
            a,
            b,
            code[1].aux_a(),
            code[1].aux_b(),
            target_label
          ),
        );
        2
      }
      LuauOpcode::LOP_COVERAGE => {
        result.push_str("COVERAGE\n");
        1
      }
      LuauOpcode::LOP_CAPTURE => {
        let cap_type = u8::try_from(a).ok().and_then(LuauCaptureType::from_repr);
        let capture_name: &'static str = cap_type.map_or("", |c| c.into());
        format_append(
          result,
          format_args!(
            "CAPTURE {} {}{b}\n",
            capture_name,
            if cap_type == Some(LuauCaptureType::LctUpval) {
              'U'
            } else {
              'R'
            },
          ),
        );
        1
      }
      LuauOpcode::LOP_JUMPXEQKNIL => {
        format_append(
          result,
          format_args!(
            "JUMPXEQKNIL R{a} L{}{}\n",
            target_label,
            insn::invert_label(code[1].raw())
          ),
        );
        2
      }
      LuauOpcode::LOP_JUMPXEQKB => {
        format_append(
          result,
          format_args!(
            "JUMPXEQKB R{a} {} L{}{}\n",
            code[1].aux_kb(),
            target_label,
            insn::invert_label(code[1].raw())
          ),
        );
        2
      }
      LuauOpcode::LOP_JUMPXEQKN => jump_xeq_k!("JUMPXEQKN"),
      LuauOpcode::LOP_JUMPXEQKS => jump_xeq_k!("JUMPXEQKS"),
      LuauOpcode::LOP_GETUDATAKS => udata_ks!("GETUDATAKS"),
      LuauOpcode::LOP_SETUDATAKS => udata_ks!("SETUDATAKS"),
      LuauOpcode::LOP_NAMECALLUDATA => udata_ks!("NAMECALLUDATA"),
      LuauOpcode::LOP_NEWCLASSMEMBER => {
        k_inline!("NEWCLASSMEMBER R{} R{} [", a, c; code[1].raw() as i32, 2)
      }
      LuauOpcode::LOP_CMPPROTO => {
        format_append(
          result,
          format_args!("CMPPROTO R{a} #{} L{target_label}\n", code[1]),
        );
        2
      }
      LuauOpcode::LOP_FASTPCALL => {
        format_append(
          result,
          format_args!(
            "FASTPCALL {} L{target_label}\n",
            if a == 0 { "pcall" } else { "xpcall" },
          ),
        );
        1
      }
      LuauOpcode::LOP_NEWCLASS => {
        if b == K_INVALID_REG {
          k_inline!("NEWCLASS R{} no_base K{} {} [", a, code[1], c; code[1].raw() as i32, 2)
        } else {
          k_inline!("NEWCLASS R{} R{} K{} {} [", a, b, code[1], c; code[1].raw() as i32, 2)
        }
      }
      _ => {
        LUAU_ASSERT!(false);
        1
      }
    }
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_dump_source_remarks.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn dump_source_remarks(&self) -> String {
    let mut result = String::new();

    let mut remarks: Vec<(i32, String)> = self.dump_remarks.clone();
    // C++ `std::sort(remarks)` orders by the WHOLE (line, message) pair, so within a
    // line remarks are ordered lexicographically by text ("builtin ..." before
    // "inlining ...") and the consecutive-duplicate skip can collapse repeated inline
    // remarks. Sorting by line only left them in insertion order.
    remarks.sort();
    let mut remarks = remarks.into_iter().peekable();

    for (i, line) in self.dump_source.iter().enumerate() {
      let line_no = (i + 1) as i32;

      let indent: usize = line
        .bytes()
        .take_while(|&b| b == b' ' || b == b'\t')
        .count();

      // 归并输出归属当前行的 remark
      while let Some((_, msg)) = remarks.next_if(|(no, _)| *no == line_no) {
        format_append(
          &mut result,
          format_args!("{:.*}-- remark: {}\n", indent, line, msg),
        );

        // 跳过重复 remark（内联/展开导致）：cpp `remarks[next] == remarks[next-1]`
        // 是 (line, message) 整对相等，行不同不得吞并
        while remarks
          .next_if(|(no, m)| *no == line_no && *m == msg)
          .is_some()
        {}
      }

      result.push_str(line);
      if i + 1 < self.dump_source.len() {
        result.push('\n');
      }
    }

    result
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_dump_type_info.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn dump_type_info(&self) -> String {
    let mut result = String::new();

    for (i, function) in self.functions.iter().enumerate() {
      let typeinfo = &function.typeinfo;
      if typeinfo.is_empty() {
        continue;
      }

      let encoded_type = typeinfo[0];

      LUAU_ASSERT!(encoded_type == LuauBytecodeType::LBC_TYPE_FUNCTION.0 as u8);

      format_append(&mut result, format_args!("{}: function(", i));

      LUAU_ASSERT!(typeinfo.len() >= 2);

      let numparams = typeinfo[1];

      LUAU_ASSERT!((1 + numparams as usize - 1) < typeinfo.len());

      for (j, &et) in typeinfo[2..2 + numparams as usize].iter().enumerate() {
        let optional = if (et & LuauBytecodeType::LBC_TYPE_OPTIONAL_BIT.0 as u8) != 0 {
          "?"
        } else {
          ""
        };

        let base_type_str = get_base_type_string(et);
        format_append(&mut result, format_args!("{}{}", base_type_str, optional));

        if j + 1 != numparams as usize {
          format_append(&mut result, format_args!(", "));
        }
      }

      format_append(&mut result, format_args!(")\n"));
    }

    result
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_needs_debug_remarks.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn needs_debug_remarks(&self) -> bool {
    DumpFlags::Remarks.is_set(self.dump_flags)
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_set_dump_flags.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn set_dump_flags(&mut self, flags: u32) {
    self.dump_flags = flags;
    self.dump_function_ptr = Some(BytecodeBuilder::dump_current_function);
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_set_dump_source.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn set_dump_source(&mut self, source: &str) {
    // cpp `while (pos != npos)` 手工切行即 `split('\n')`：按下一个 '\n' 切块、
    // 无 '\n' 时产出余量，且以 '\n' 结尾时额外产出一个尾部空行——
    // dumpSourceRemarks 依赖该空行输出收尾换行，不可丢。CRLF 的行尾 '\r' 逐行剥除。
    self.dump_source = source
      .split('\n')
      .map(|line| line.strip_suffix('\r').unwrap_or(line).to_owned())
      .collect();
  }
}
