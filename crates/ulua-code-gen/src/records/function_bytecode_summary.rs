use alloc::{string::String, vec::Vec};

use ulua_common::{
  enums::luau_opcode::LuauOpcode, functions::get_op_length::get_op_length,
  macros::luau_insn_op::luau_insn_op,
};
use ulua_vm::records::proto::Proto;

use crate::functions::proto_views::{code, name_bytes};

extern crate alloc;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct FunctionBytecodeSummary {
  pub source: String,
  pub name: String,
  pub line: i32,
  pub nesting_limit: u32,
  pub counts: Vec<Vec<u32>>,
}

impl FunctionBytecodeSummary {
  pub const LOP__COUNT: u32 = LuauOpcode::LOP__COUNT as u32;

  /// 从存活 `Proto` 汇总指令直方图（cpp `FunctionBytecodeSummary::fromProto`）。
  ///
  /// 契约：`proto` 指向存活 Proto（编译会话内由 VM 持有）；`source`/`debugname` 为空或指向
  /// 存活且 NUL 结尾的 TString；`code`/`sizecode` 成界。
  /// 原型字段一律经 [`proto_views`] 的安全视图读取，本函数不再有裸指针解引用。
  pub fn from_proto(proto: &Proto, nesting_limit: u32) -> Self {
    // cpp：`getstr(source)` 整串取回后按前导 '=' / '@' 剥离；无源名或前缀不匹配则记 "[string]"。
    // 空指针经 `name_bytes` 折成 None，与原 `cstr_cow` 的「空串」语义同落 `_` 分支。
    let source = match name_bytes(proto.source.cast_const(), proto) {
      Some(bytes) if matches!(bytes.first(), Some(b'=') | Some(b'@')) => {
        String::from_utf8_lossy(&bytes[1..]).into_owned()
      }
      _ => String::from("[string]"),
    };

    let name = match name_bytes(proto.debugname.cast_const(), proto) {
      Some(bytes) => String::from_utf8_lossy(bytes).into_owned(),
      None => String::new(),
    };

    let mut summary = Self::new(source, name, proto.linedefined, nesting_limit);

    let code = code(proto);

    // 与原 while 语义一致：按指令自身长度步进，只统计每条指令的首字操作码。
    let mut i = 0usize;
    while i < code.len() {
      let op = luau_insn_op(code[i]) as u8;
      summary.inc_count(0, op);
      i += get_op_length(LuauOpcode::from(op)) as usize;
    }

    summary
  }

  pub fn new(source: String, name: String, line: i32, nesting_limit: u32) -> Self {
    let mut summary = Self {
      source,
      name,
      line,
      nesting_limit,
      counts: Vec::new(),
    };

    let op_limit = summary.get_op_limit() as usize;
    let mut counts: Vec<Vec<u32>> = Vec::with_capacity((1 + nesting_limit) as usize);
    for _ in 0..(1 + nesting_limit) {
      counts.push(vec![0u32; op_limit]);
    }

    summary.counts = counts;
    summary
  }

  pub fn get_counts(&self, nesting: u32) -> &[u32] {
    debug_assert!(nesting <= self.get_nesting_limit());
    &self.counts[nesting as usize]
  }

  pub fn get_line(&self) -> i32 {
    self.line
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn get_nesting_limit(&self) -> u32 {
    self.nesting_limit
  }

  pub fn get_op_limit(&self) -> u32 {
    Self::LOP__COUNT
  }

  pub fn get_source(&self) -> &str {
    &self.source
  }
}
