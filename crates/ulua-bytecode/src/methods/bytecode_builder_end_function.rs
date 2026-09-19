use core::mem;
use std::vec::Vec;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::bytecode_builder::BytecodeBuilder;

impl BytecodeBuilder {
  /// `cost`：cpp `endFunction(..., uint64_t cost)` 的内联开销模型，LPF_INLINABLE
  /// 时由 `write_function` 序列化进字节码（Compiler.cpp:621 传入）。
  pub fn end_function(&mut self, maxstacksize: u8, numupvalues: u8, flags: u8, cost: u64) {
    LUAU_ASSERT!(self.current_function != u32::MAX);

    let current_function = self.current_function;
    let dump = if let Some(dump_fn) = self.dump_function_ptr {
      let mut dumpinstoffs = mem::take(&mut self.functions[current_function as usize].dumpinstoffs);
      let dump = dump_fn(self, &mut dumpinstoffs);
      self.functions[current_function as usize].dumpinstoffs = dumpinstoffs;
      Some(dump)
    } else {
      None
    };
    {
      let func = &mut self.functions[current_function as usize];
      func.maxstacksize = maxstacksize;
      func.numupvalues = numupvalues;

      if let Some(dump) = dump {
        func.dump = dump;
      }

      func.data.reserve(32 + self.insns.len() * 7);
    }

    // cpp `#ifdef LUAU_ASSERTENABLED validate(); #endif`：紧随 maxstacksize /
    // numupvalues 落位之后、`encoder.encode` 置换操作码之前（`validate` 内部按
    // `LUAU_ASSERTENABLED` 常量短路，release 零开销）。缺了这一步，非法图会被
    // 静默写进 data，错误一路延后到反序列化/VM 才暴露。
    self.validate();

    // 字段级不相交借用: self.encoder 与 self.insns 可同时可变, 无需 unsafe
    if let Some(encoder) = self.encoder.as_mut() {
      encoder.encode(&mut self.insns);
    }

    let mut data = Vec::new();
    self.write_function(&mut data, current_function, flags, cost);
    self.functions[current_function as usize].data = data;

    self.current_function = u32::MAX;
    self.total_instruction_count += self.insns.len();

    self.insns.clear();
    self.lines.clear();
    self.constants.clear();
    self.protos.clear();
    self.jumps.clear();
    self.fb_slots.clear();
    self.table_shapes.clear();

    self.debug_locals.clear();
    self.debug_upvals.clear();

    self.typed_locals.clear();
    self.typed_upvals.clear();

    self.constant_map.clear();
    self.table_shape_map.clear();
    self.proto_map.clear();

    self.debug_remarks.clear();
    self.debug_remark_buffer.clear();
  }
}
