//! `BytecodeBuilder` 之 函数体生命周期：begin/end、debug 名与局部/上值类型登记。

use std::{string::String, vec::Vec};

use ulua_common::{enums::luau_bytecode_type::LuauBytecodeType, macros::luau_assert::LUAU_ASSERT};

use super::BytecodeBuilder;
use crate::records::{
  debug_local_bytecode_builder::DebugLocal, debug_upval::DebugUpval, function::Function,
  string_ref::StringRef, typed_local::TypedLocal, typed_upval::TypedUpval,
};

// ── abs-r139：并自 `methods/bytecode_builder_begin_function.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn begin_function(&mut self, numparams: u8, isvararg: bool) -> u32 {
    LUAU_ASSERT!(self.current_function.is_none());

    let id = self.functions.len() as u32;

    let func = Function {
      numparams,
      isvararg,
      ..Default::default()
    };

    self.functions.push(func);

    self.current_function = Some(id);

    self.has_long_jumps = false;
    self.debug_line = 0;

    id
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_end_function.rs` ──
impl<'a> BytecodeBuilder<'a> {
  /// `cost`：cpp `endFunction(..., uint64_t cost)` 的内联开销模型，LPF_INLINABLE
  /// 时由 `write_function` 序列化进字节码（Compiler.cpp:621 传入）。
  pub fn end_function(&mut self, maxstacksize: u8, numupvalues: u8, flags: u8, cost: u64) {
    LUAU_ASSERT!(self.current_function.is_some());
    // 协议违例（未 begin 就 end）在 cpp 里是 `functions[-1]` 越界；断言 release
    // 放行时按 id 0 走安全路径，不再复制 UB。
    let current_function = self.current_function.unwrap_or(0) as usize;
    let dump = self.dump_function_ptr.map(|dump_fn| dump_fn(self));
    {
      let func = &mut self.functions[current_function];
      func.maxstacksize = maxstacksize;
      func.numupvalues = numupvalues;

      if let Some((dump, dumpinstoffs)) = dump {
        func.dump = dump;
        func.dumpinstoffs = dumpinstoffs;
      }
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

    // 序列化容量提示落在真正收集字节的 `data` 上（原先误 reserve 在随后被整体
    // 替换的旧 func.data 上，热路径每次序列化都要多次扩容重分配）。
    let mut data = Vec::with_capacity(32 + self.insns.len() * 7);
    self.write_function(&mut data, current_function as u32, flags, cost);
    self.functions[current_function].data = data;

    self.current_function = None;
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

// ── abs-r139：并自 `methods/bytecode_builder_push_debug_local.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn push_debug_local(&mut self, name: StringRef<'a>, reg: u8, startpc: u32, endpc: u32) {
    let index = self.add_string_table_entry(name);

    let local = DebugLocal {
      name: index,
      reg,
      startpc,
      endpc,
    };

    self.debug_locals.push(local);
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_push_debug_upval.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn push_debug_upval(&mut self, name: StringRef<'a>) {
    let index = self.add_string_table_entry(name);

    let upval = DebugUpval { name: index };

    self.debug_upvals.push(upval);
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_push_local_type_info.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn push_local_type_info(
    &mut self,
    r#type: LuauBytecodeType,
    reg: u8,
    startpc: u32,
    endpc: u32,
  ) {
    let local = TypedLocal {
      r#type,
      reg,
      startpc,
      endpc,
    };

    self.typed_locals.push(local);
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_push_upval_type_info.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn push_upval_type_info(&mut self, r#type: LuauBytecodeType) {
    let upval = TypedUpval { r#type };
    self.typed_upvals.push(upval);
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_set_debug_function_line_defined.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn set_debug_function_line_defined(&mut self, line: i32) {
    // cpp `functions[currentFunction]` 在 -1（未 begin）时是越界 UB；这里
    // None 直接跳过写入，断言兜住 debug 下的协议违例。
    LUAU_ASSERT!(self.current_function.is_some());
    if let Some(id) = self.current_function {
      self.functions[id as usize].debuglinedefined = line;
    }
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_set_debug_function_name.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn set_debug_function_name(&mut self, name: StringRef<'a>) {
    let index = self.add_string_table_entry(name.clone());

    LUAU_ASSERT!(self.current_function.is_some());
    if let Some(id) = self.current_function {
      self.functions[id as usize].debugname = index;

      // cpp `functions[currentFunction].dumpname = std::string(name.data, name.length)`：
      // dumpname 是函数调试名（如 'foo'），供 DUPCLOSURE 常量 dump 展示，不是字节码
      // dump 文本。移植曾误在此处调用 dump 函数指针，故仅在 dump 开启时填充。
      if self.dump_function_ptr.is_some() {
        self.functions[id as usize].dumpname =
          String::from_utf8_lossy(name.as_bytes()).into_owned();
      }
    }
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_set_function_type_info.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn set_function_type_info(&mut self, value: Vec<u8>) {
    LUAU_ASSERT!(self.current_function.is_some());
    if let Some(id) = self.current_function {
      self.functions[id as usize].typeinfo = value;
    }
  }
}

// ── abs-r139：并自 `methods/bytecode_builder_set_main_function.rs` ──
impl<'a> BytecodeBuilder<'a> {
  pub fn set_main_function(&mut self, fid: u32) {
    LUAU_ASSERT!(fid < self.functions.len() as u32);

    self.main_function = fid;
  }
}
