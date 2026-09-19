use alloc::{string::String, vec};
use core::cmp::min;

use ulua_common::{
  fflag,
  functions::{format_append::format_append, format_g::format_g},
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
      Type::Nil => format_append(result, format_args!("nil")),
      Type::Boolean => format_append(
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
      Type::Number => format_append(
        result,
        format_args!("{}", format_g(unsafe { data.value.value_number }, 17)),
      ),
      Type::Integer => format_append(
        result,
        format_args!("{}", { unsafe { data.value.value_integer64 } }),
      ),
      Type::Vector => {
        let v = unsafe { data.value.value_vector };
        if v[3] == 0.0 {
          format_append(
            result,
            format_args!(
              "{}, {}, {}",
              format_g(v[0] as f64, 9),
              format_g(v[1] as f64, 9),
              format_g(v[2] as f64, 9)
            ),
          );
        } else {
          format_append(
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
      // cpp `BytecodeBuilder.cpp:2353-2377`：flag 开时按 %.17g 打双精度，
      // 关时先转 float 再按 %.9g 打（与 writeFunction 的降级路径一致）。
      // 三分量截断只看 `valueVectord[3] == 0`，与 flag 无关。
      Type::Vectord => {
        let v = unsafe { data.value.value_vector_d };
        let wide = fflag::LuauCompileEmitVectorDouble.get();
        let g = |x: f64| {
          format_g(
            if wide { x } else { f64::from(x as f32) },
            if wide { 17 } else { 9 },
          )
        };
        if v[3] == 0.0 {
          format_append(
            result,
            format_args!("{}, {}, {}", g(v[0]), g(v[1]), g(v[2])),
          );
        } else {
          format_append(
            result,
            format_args!("{}, {}, {}, {}", g(v[0]), g(v[1]), g(v[2]), g(v[3])),
          );
        }
      }
      Type::String => {
        let str_idx = unsafe { data.value.value_string };
        let str = &self.debug_strings[str_idx as usize - 1];
        let bytes = str.as_bytes();
        if printable_string_constant(bytes) {
          // 显示路径用 lossy 容错；printable 分支均为合法 UTF-8，输出无损
          let s = str.to_string_lossy();
          if str.length < 32 {
            format_append(result, format_args!("'{:.*}'", str.length, s));
          } else {
            format_append(result, format_args!("'{:.*}'...", 32, s));
          }
        } else {
          format_append(result, format_args!("'"));
          for &b in &bytes[..min(str.length, 32)] {
            if b < b' ' {
              format_append(result, format_args!("\\x{:02X}", b));
            } else {
              format_append(result, format_args!("{}", b as char));
            }
          }
          if str.length >= 32 {
            format_append(result, format_args!("'..."));
          } else {
            format_append(result, format_args!("'"));
          }
        }
      }
      Type::Import => {
        let (count, id0, id1, id2) =
          BytecodeBuilder::decompose_import_id(unsafe { data.value.value_import });
        if count > 0 {
          let seg0 = self.import_segment_str(id0);
          format_append(result, format_args!("{}", seg0.to_string_lossy()));

          if count > 1 {
            let seg1 = self.import_segment_str(id1);
            format_append(result, format_args!(".{}", seg1.to_string_lossy()));
          }

          if count > 2 {
            let seg2 = self.import_segment_str(id2);
            format_append(result, format_args!(".{}", seg2.to_string_lossy()));
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

          format_append(result, format_args!("{{"));

          for (i, &key_idx) in shape.keys.iter().enumerate().take(shape.length as usize) {
            if i > 0 {
              format_append(result, format_args!(", "));
            }

            format_append(result, format_args!("["));
            self.dump_constant(result, key_idx, false);
            format_append(result, format_args!("]"));

            if shape.has_constants && shape.constants[i] != -1 {
              format_append(result, format_args!(" = "));
              self.dump_constant(result, shape.constants[i], false);
            }

            format_append(result, format_args!(" #{}", slots[i]));

            if slot_owner[slots[i] as usize] != i as u32 {
              format_append(result, format_args!(" (conflict)"));
            }
          }

          format_append(result, format_args!("}} sizenode={}", sizenode));
        } else {
          format_append(result, format_args!("{{...}}"));
        }
      }
      Type::Closure => {
        let func = &self.functions[unsafe { data.value.value_closure } as usize];
        // detailed 模式下输出 "function"/"function <名>"，非 detailed 输出 "'<名>'"
        if detailed {
          if !func.dumpname.is_empty() {
            format_append(result, format_args!("function {}", func.dumpname));
          } else {
            format_append(result, format_args!("function"));
          }
        } else if !func.dumpname.is_empty() {
          format_append(result, format_args!("'{}'", func.dumpname));
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
        format_append(
          result,
          format_args!(
            "class {} (props: {}, methods: {})",
            str.to_string_lossy(),
            cs.property_names.len(),
            cs.method_names.len()
          ),
        );

        // detailed 模式追加 props/methods 的常量索引清单
        if detailed {
          if !cs.property_names.is_empty() {
            format_append(result, format_args!("\n  props:"));
            for &k in &cs.property_names {
              format_append(result, format_args!("\n    K{} [", k));
              self.dump_constant(result, k, false);
              result.push(']');
            }
          }

          if !cs.method_names.is_empty() {
            format_append(result, format_args!("\n  methods:"));
            for &k in &cs.method_names {
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
