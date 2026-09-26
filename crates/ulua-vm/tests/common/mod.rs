//! loadsafe / luau_execute 契约测试共用的字节码 blob 拼装器。
//!
//! 只覆盖 `LBC_VERSION_TARGET`(=9) + typesversion 2 的最小形态，字段顺序对照
//! cpp/VM/src/lvmload.cpp 的 `readHeader`/`readProto`：
//! `version | typesversion | stringCount | strings | protoCount | protos | mainid`。
//! v9 早于 feedbackvec（v11）与 protoSize（v12），因此没有那两段。
//!
//! 本模块以 `#[path = "common/mod.rs"] mod common;` 的形式被每个测试 binary
//! 单独编进自己的 crate 里，因此这里只允许放「每个使用者都会用到」的条目
//! （不允许 `#[allow(dead_code)]`，未用到的项会在别的 binary 里报 dead_code）。
//! VM 状态 RAII 守卫在 `common/state.rs`（同为按 binary 单独编入）。

use ulua_common::enums::luau_bytecode_tag::LuauBytecodeTag;
use ulua_vm::{functions::luau_load::luau_load, records::lua_state::LuaState};

/// 装载 blob：返回 `luau_load` 的状态码（0 成功；1 表示栈顶已是错误字符串）。
/// 空 blob 以 `&[]` 传入，覆盖「长度为 0」的边界。
pub fn load(l: *mut LuaState, chunkname: &str, blob: &[u8]) -> i32 {
  // Safety: `l` 为测试的 State 守卫独占持有的存活 VM；chunkname/blob 借用覆盖整个调用，
  // 空 blob 以 `&[]` 走长度 0 分支，均满足 luau_load 的 C ABI 前置条件。
  unsafe { luau_load(l, chunkname, blob, 0) }
}

/// blob 的字节码版本；< 11 无 feedbackvec，< 12 无 protoSize
pub const VERSION: u8 = LuauBytecodeTag::LBC_VERSION_TARGET.0 as u8;
/// typesversion 2：走 `varint typesize` 分支，无 v1 变换与 v3 重映射
pub const TYPES_VERSION: u8 = 2;

/// 手工拼装 blob 的最小写入器（不依赖编译器 crate）
#[derive(Default)]
pub struct Blob {
  pub bytes: Vec<u8>,
}

impl Blob {
  pub fn byte(&mut self, v: u8) -> &mut Self {
    self.bytes.push(v);
    self
  }

  /// 无符号 LEB128：与 `read_var_int` 对偶
  pub fn varint(&mut self, v: u32) -> &mut Self {
    let mut rest = v;

    loop {
      let mut b = u8::try_from(rest & 127).expect("低 7 位必在 u8 内");
      rest >>= 7;

      if rest != 0 {
        b |= 128;
      }

      self.byte(b);

      if rest == 0 {
        return self;
      }
    }
  }

  /// 指令字按小端 4 字节存储（cpp `read<uint32_t>`）
  pub fn u32le(&mut self, v: u32) -> &mut Self {
    self.bytes.extend_from_slice(&v.to_le_bytes());
    self
  }

  /// 原字节 payload（f32/f64 常量体、字符串体都由调用方 `to_le_bytes` 后写入）
  pub fn raw(&mut self, bytes: &[u8]) -> &mut Self {
    self.bytes.extend_from_slice(bytes);
    self
  }
}

/// 一个 proto 的可变部分：常量原始字节、内层 proto id、debugname 的 string id、
/// 寄存器规模，以及故意与 blob 不一致的 sizecode
#[derive(Default)]
pub struct ProtoSpec<'a> {
  pub code: &'a [u32],
  /// 写进 blob 的 sizecode；`None` 表示取 `code.len()`
  pub sizecode: Option<u32>,
  pub constants: &'a [u8],
  pub nconstants: u32,
  pub inner: &'a [u32],
  pub debugname: u32,
  pub maxstacksize: u8,
  pub numparams: u8,
  /// 写完 constants 段即截断 blob：构造「常量体读到一半没了」的畸形输入
  pub truncate_after_constants: bool,
}

impl ProtoSpec<'_> {
  pub fn write(&self, blob: &mut Blob) {
    // maxstacksize / numparams / nups / is_vararg / flags
    for value in [self.maxstacksize, self.numparams, 0, 0, 0] {
      blob.byte(value);
    }

    blob.varint(0); // typesize：无类型信息
    blob.varint(self.sizecode.unwrap_or(self.code.len() as u32));

    for insn in self.code {
      blob.u32le(*insn);
    }

    blob.varint(self.nconstants);
    blob.raw(self.constants);

    if self.truncate_after_constants {
      return;
    }

    blob.varint(self.inner.len() as u32);

    for fid in self.inner {
      blob.varint(*fid);
    }

    blob.varint(0); // linedefined
    blob.varint(self.debugname); // debugname string id
    blob.byte(0); // 无 lineinfo
    blob.byte(0); // 无 debuginfo
  }
}

/// 拼出完整 blob：`strings` 是字符串表的原始字节
pub fn proto_blob(strings: &[&[u8]], protos: &[ProtoSpec<'_>], mainid: u32) -> Vec<u8> {
  let mut blob = Blob::default();

  blob
    .byte(VERSION)
    .byte(TYPES_VERSION)
    .varint(strings.len() as u32);

  for string in strings {
    blob.varint(string.len() as u32).raw(string);
  }

  blob.varint(protos.len() as u32);

  for proto in protos {
    proto.write(&mut blob);
  }

  // 截断型用例：blob 必须真的停在 constants 段末尾，不补 mainid
  if protos.iter().any(|proto| proto.truncate_after_constants) {
    return blob.bytes;
  }

  blob.varint(mainid);

  blob.bytes
}
