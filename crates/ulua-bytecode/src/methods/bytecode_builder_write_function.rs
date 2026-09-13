use alloc::string::String;

use ulua_common::{
  FFlag,
  enums::{luau_bytecode_tag::LuauBytecodeTag, luau_feedback_type::LuauFeedbackType},
  macros::luau_assert::LUAU_ASSERT,
};

use crate::{
  enums::r#type::Type,
  functions::{
    write_byte::write_byte, write_double::write_double, write_float::write_float,
    write_int::write_int, write_var_int::write_var_int,
  },
  records::bytecode_builder::BytecodeBuilder,
};

impl BytecodeBuilder {
  pub fn write_function(&mut self, ss: &mut String, id: u32, flags: u8) {
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

      self.temp_type_info.push_str(&func.typeinfo);

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
      ss.push_str(&self.temp_type_info);
    } else {
      write_var_int(ss, 0);
    }

    // instructions
    write_var_int(ss, self.insns.len() as u64);

    for &insn in &self.insns {
      write_int(ss, insn as i32);
    }

    // constants
    write_var_int(ss, self.constants.len() as u64);

    for c in &self.constants {
      match c.r#type {
        Type::Nil => {
          write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_NIL.0 as u8);
        }
        Type::Boolean => {
          write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_BOOLEAN.0 as u8);
          write_byte(ss, unsafe { c.value.value_boolean } as u8);
        }
        Type::Number => {
          write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_NUMBER.0 as u8);
          write_double(ss, unsafe { c.value.value_number });
        }
        Type::Integer => {
          write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_INTEGER.0 as u8);
          let value = unsafe { c.value.value_integer64 };
          if value < 0 {
            write_byte(ss, 1);
            // C++ `~(uint64_t)value + 1` 即负数的绝对值
            write_var_int(ss, value.unsigned_abs());
          } else {
            write_byte(ss, 0);
            write_var_int(ss, value as u64);
          }
        }
        Type::Vector => {
          write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_VECTOR.0 as u8);
          let vec = unsafe { c.value.value_vector };
          write_float(ss, vec[0]);
          write_float(ss, vec[1]);
          write_float(ss, vec[2]);
          write_float(ss, vec[3]);
        }
        Type::String => {
          write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_STRING.0 as u8);
          write_var_int(ss, unsafe { c.value.value_string } as u64);
        }
        Type::Import => {
          write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_IMPORT.0 as u8);
          write_int(ss, unsafe { c.value.value_import } as i32);
        }
        Type::Table => {
          let shape = &self.table_shapes[unsafe { c.value.value_table } as usize];
          if FFlag::LuauCompileDuptableConstantPack2.get() && shape.has_constants {
            write_byte(
              ss,
              LuauBytecodeTag::LBC_CONSTANT_TABLE_WITH_CONSTANTS.0 as u8,
            );
            write_var_int(ss, shape.length as u64);
            for (&key, &value) in shape
              .keys
              .iter()
              .zip(&shape.constants)
              .take(shape.length as usize)
            {
              write_var_int(ss, key as u64);
              write_int(ss, value);
            }
          } else {
            write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_TABLE.0 as u8);
            write_var_int(ss, shape.length as u64);
            for &key in &shape.keys[..shape.length as usize] {
              write_var_int(ss, key as u64);
            }
          }
        }
        Type::Closure => {
          write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_CLOSURE.0 as u8);
          write_var_int(ss, unsafe { c.value.value_closure } as u64);
        }
        Type::ClassShape => {
          write_byte(ss, LuauBytecodeTag::LBC_CONSTANT_CLASS_SHAPE.0 as u8);
          let cs = &self.class_shapes[unsafe { c.value.value_class_shape } as usize];
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

    if FFlag::LuauEmitCallFeedback.get() {
      // Feedback Slots
      write_var_int(ss, self.fb_slots.len() as u64);
      for &pc in &self.fb_slots {
        write_byte(ss, LuauFeedbackType::LFT_CALLTARGET as u8);
        write_var_int(ss, pc as u64);
      }
    }
  }
}
