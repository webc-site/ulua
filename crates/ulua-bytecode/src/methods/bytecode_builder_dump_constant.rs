use alloc::{string::String, vec};
use core::{cmp::min, str::from_utf8_unchecked};

use ulua_common::{
  functions::{format_append::formatAppend, format_g::format_g},
  macros::luau_assert::LUAU_ASSERT,
};

use crate::{
  enums::r#type::Type,
  functions::{ceillog_2::ceillog2, printable_string_constant::printable_string_constant},
  methods::bytecode_builder_get_string_hash::bytecode_builder_get_string_hash,
  records::{bytecode_builder::BytecodeBuilder, string_ref::StringRef},
};

impl BytecodeBuilder {
  /// 取 import 段的字符串常量（union 读取 + 前置断言，三段复用）。
  /// SAFETY 前提：`idx` 指向的常量在构建时已注册为 String 且索引非 0。
  fn import_segment_str(&self, idx: i32) -> &StringRef {
    // SAFETY：union 字段 value_string 仅在 r#type == String 的常量上读取，
    // 由下方 LUAU_ASSERT 保证（构建期写入时已定型）。
    let str_idx = unsafe { self.constants[idx as usize].value.value_string };
    LUAU_ASSERT!(str_idx as usize <= self.debug_strings.len());
    &self.debug_strings[str_idx as usize - 1]
  }

  pub fn dump_constant(&self, result: &mut String, k: i32, detailed: bool) {
    LUAU_ASSERT!((k as u32) < self.constants.len() as u32);
    let data = &self.constants[k as usize];

    match data.r#type {
      Type::Nil => formatAppend(result, format_args!("nil")),
      Type::Boolean => formatAppend(
        result,
        format_args!(
          "{}",
          if unsafe { data.value.value_boolean } {
            "true"
          } else {
            "false"
          }
        ),
      ),
      Type::Number => formatAppend(
        result,
        format_args!("{}", format_g(unsafe { data.value.value_number }, 17)),
      ),
      Type::Integer => formatAppend(
        result,
        format_args!("{}", { unsafe { data.value.value_integer64 } }),
      ),
      Type::Vector => {
        let v = unsafe { data.value.value_vector };
        if v[3] == 0.0 {
          formatAppend(
            result,
            format_args!(
              "{}, {}, {}",
              format_g(v[0] as f64, 9),
              format_g(v[1] as f64, 9),
              format_g(v[2] as f64, 9)
            ),
          );
        } else {
          formatAppend(
            result,
            format_args!(
              "{}, {}, {}, {}",
              format_g(v[0] as f64, 9),
              format_g(v[1] as f64, 9),
              format_g(v[2] as f64, 9),
              format_g(v[3] as f64, 9)
            ),
          );
        }
      }
      Type::String => {
        let str_idx = unsafe { data.value.value_string };
        let str = &self.debug_strings[str_idx as usize - 1];
        let bytes = str.as_bytes();
        if printable_string_constant(bytes) {
          if str.length < 32 {
            formatAppend(
              result,
              format_args!("'{:.*}'", str.length, unsafe { from_utf8_unchecked(bytes) }),
            );
          } else {
            formatAppend(
              result,
              format_args!("'{:.*}'...", 32, unsafe { from_utf8_unchecked(bytes) }),
            );
          }
        } else {
          formatAppend(result, format_args!("'"));
          for &b in &bytes[..min(str.length, 32)] {
            if b < b' ' {
              formatAppend(result, format_args!("\\x{:02X}", b));
            } else {
              formatAppend(result, format_args!("{}", b as char));
            }
          }
          if str.length >= 32 {
            formatAppend(result, format_args!("'..."));
          } else {
            formatAppend(result, format_args!("'"));
          }
        }
      }
      Type::Import => {
        let mut id0: i32 = -1;
        let mut id1: i32 = -1;
        let mut id2: i32 = -1;
        let count = BytecodeBuilder::decompose_import_id(
          unsafe { data.value.value_import },
          &mut id0,
          &mut id1,
          &mut id2,
        );
        if count > 0 {
          // SAFETY：from_utf8_unchecked 的前提由 LUAU_ASSERT(printable) 类似约束保障，
          // 调试字符串表中的条目均为合法 UTF-8（写入时来自 Luau 源码的标识符）。
          let seg0 = self.import_segment_str(id0);
          formatAppend(
            result,
            format_args!("{}", unsafe { from_utf8_unchecked(seg0.as_bytes()) }),
          );

          if count > 1 {
            let seg1 = self.import_segment_str(id1);
            formatAppend(
              result,
              format_args!(".{}", unsafe { from_utf8_unchecked(seg1.as_bytes()) }),
            );
          }

          if count > 2 {
            let seg2 = self.import_segment_str(id2);
            formatAppend(
              result,
              format_args!(".{}", unsafe { from_utf8_unchecked(seg2.as_bytes()) }),
            );
          }
        }
      }
      Type::Table => {
        if detailed {
          let shape = &self.table_shapes[unsafe { data.value.value_table } as usize];
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
            // SAFETY：union 字段 value_string 仅在 String 常量上读取（assert 保证）。
            LUAU_ASSERT!(
              key_const.r#type == Type::String && unsafe { key_const.value.value_string } != 0
            );
            let str = &self.debug_strings[unsafe { key_const.value.value_string } as usize - 1];
            let hash = bytecode_builder_get_string_hash(*str);
            slots[i] = hash & mask;

            if slot_owner[slots[i] as usize] == !0u32 {
              slot_owner[slots[i] as usize] = i as u32;
            }
          }

          formatAppend(result, format_args!("{{"));

          for (i, &key_idx) in shape.keys.iter().enumerate().take(shape.length as usize) {
            if i > 0 {
              formatAppend(result, format_args!(", "));
            }

            formatAppend(result, format_args!("["));
            self.dump_constant(result, key_idx, false);
            formatAppend(result, format_args!("]"));

            if shape.has_constants && shape.constants[i] != -1 {
              formatAppend(result, format_args!(" = "));
              self.dump_constant(result, shape.constants[i], false);
            }

            formatAppend(result, format_args!(" #{}", slots[i]));

            if slot_owner[slots[i] as usize] != i as u32 {
              formatAppend(result, format_args!(" (conflict)"));
            }
          }

          formatAppend(result, format_args!("}} sizenode={}", sizenode));
        } else {
          formatAppend(result, format_args!("{{...}}"));
        }
      }
      Type::Closure => {
        let func = &self.functions[unsafe { data.value.value_closure } as usize];
        if !func.dumpname.is_empty() {
          formatAppend(result, format_args!("'{}'", func.dumpname));
        }
      }
      Type::ClassShape => {
        let cs = &self.class_shapes[unsafe { data.value.value_class_shape } as usize];
        let class_name_const = &self.constants[cs.class_name as usize];
        LUAU_ASSERT!(
          class_name_const.r#type == Type::String
            && unsafe { class_name_const.value.value_string } as usize <= self.debug_strings.len()
        );
        let str = &self.debug_strings[unsafe { class_name_const.value.value_string } as usize - 1];
        LUAU_ASSERT!(printable_string_constant(str.as_bytes()));
        formatAppend(
          result,
          format_args!(
            "class {} (props: {}, methods: {})",
            unsafe { from_utf8_unchecked(str.as_bytes()) },
            cs.property_names.len(),
            cs.method_names.len()
          ),
        );
      }
    }
  }
}
