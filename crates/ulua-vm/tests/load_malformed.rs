//! loadsafe（不可信字节码反序列化）的畸形/截断输入契约测试。
//!
//! 对照 cpp/VM/src/lvmload.cpp：cpp 只靠 `TempBuffer::operator[]` 与
//! `LUAU_ASSERT` 兜底（release 编译掉即越界读），Rust 侧必须把
//! 不可信 id / 长度 / count 转成「损坏字节码」错误（返回 1 + 错误字符串）。
//!
//! blob 手工按 v9（typesversion 2）格式拼装，不依赖编译器 crate：
//! `version | typesversion | stringCount | strings | protoCount | protos | mainid`。

use std::{
  ffi::{CStr, c_char},
  ptr::{eq, null},
  slice::from_raw_parts,
};

use ulua_common::enums::luau_bytecode_tag::{LBC_VERSION_MAX, LBC_VERSION_MIN};
use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{
    lua_close::lua_close, lua_gettop::lua_gettop, lua_l_newstate::lua_l_newstate,
    lua_tolstring::lua_tolstring, lua_type::lua_type, luau_load::luau_load,
  },
  type_aliases::lua_state::lua_State,
};

/// 与 cpp LBC_VERSION_TARGET 一致；< 11 无 feedbackvec，< 12 无 protoSize
const VERSION: u8 = 9;
/// typesversion 2：走 `varint typesize` 分支，无 v1 变换与 v3 重映射
const TYPES_VERSION: u8 = 2;
const LBC_CONSTANT_NUMBER: u8 = 2;
const LBC_CONSTANT_STRING: u8 = 3;
const LBC_CONSTANT_CLOSURE: u8 = 6;
const LBC_CONSTANT_VECTOR: u8 = 7;
/// 未定义的常量 tag（cpp LBC_CONSTANT__COUNT 起）
const UNKNOWN_CONSTANT_TAG: u8 = 200;

/// 独立 VM 状态的 RAII 守卫
struct State {
  l: *mut lua_State,
}

impl State {
  fn new() -> Self {
    let l = lua_l_newstate();
    assert!(!l.is_null(), "lua_l_newstate 失败");
    Self { l }
  }

  /// 加载 blob：返回 `luau_load` 的状态码（0 成功，1 表示栈上已是错误字符串）
  unsafe fn load(&self, blob: &[u8]) -> i32 {
    unsafe { self.load_as(c"@malformed", blob) }
  }

  /// 指定 chunkname 加载：version==0 通道会把 chunkid 与 payload 直接拼接
  unsafe fn load_as(&self, chunkname: &CStr, blob: &[u8]) -> i32 {
    unsafe {
      luau_load(
        self.l,
        chunkname.as_ptr(),
        if blob.is_empty() {
          null()
        } else {
          blob.as_ptr().cast::<c_char>()
        },
        blob.len(),
        0,
      )
    }
  }

  /// 栈顶字符串（错误消息）
  unsafe fn top_string(&self) -> String {
    unsafe {
      assert_eq!(
        lua_type(self.l, -1),
        LuaType::String as i32,
        "栈顶必须是错误字符串"
      );
      let mut len = 0;
      let p = lua_tolstring(self.l, -1, &mut len);
      assert!(!eq(p, null()));
      String::from_utf8_lossy(from_raw_parts(p.cast::<u8>(), len)).into_owned()
    }
  }
}

impl Drop for State {
  fn drop(&mut self) {
    unsafe { lua_close(self.l) };
  }
}

/// 最小 LEB128 varint 写入器（与 `read_var_int_64` 对偶）
#[derive(Default)]
struct Blob {
  bytes: Vec<u8>,
}

impl Blob {
  fn byte(&mut self, v: u8) -> &mut Self {
    self.bytes.push(v);
    self
  }

  fn varint(&mut self, v: u32) -> &mut Self {
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
  fn u32le(&mut self, v: u32) -> &mut Self {
    self.bytes.extend_from_slice(&v.to_le_bytes());
    self
  }

  fn raw(&mut self, bytes: &[u8]) -> &mut Self {
    self.bytes.extend_from_slice(bytes);
    self
  }
}

/// 一个 proto 的可变部分：常量原始字节、内层 proto id、debugname 的 string id、
/// 以及故意与 blob 不一致的 sizecode
#[derive(Default)]
struct ProtoSpec<'a> {
  code: &'a [u32],
  /// 写进 blob 的 sizecode；`None` 表示取 `code.len()`
  sizecode: Option<u32>,
  constants: &'a [u8],
  nconstants: u32,
  inner: &'a [u32],
  debugname: u32,
  /// 写完 constants 段即截断 blob：构造「常量体读到一半没了」的畸形输入
  truncate_after_constants: bool,
}

impl ProtoSpec<'_> {
  fn write(&self, blob: &mut Blob) {
    // maxstacksize / numparams / nups / is_vararg / flags
    for value in [1, 0, 0, 0, 0] {
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

/// 空 proto 的最小合法 blob
fn empty_proto() -> Vec<u8> {
  proto_blob(&[ProtoSpec::default()], 0)
}

fn proto_blob(protos: &[ProtoSpec<'_>], mainid: u32) -> Vec<u8> {
  let mut blob = Blob::default();

  blob.byte(VERSION).byte(TYPES_VERSION).varint(0);
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

/// 加载失败：返回 1，栈顶为以 chunkid 起头的损坏字节码错误
fn assert_malformed(blob: &[u8], expect: &str) {
  let s = State::new();

  unsafe {
    assert_eq!(s.load(blob), 1, "畸形字节码必须以错误返回");
    assert_eq!(lua_gettop(s.l), 1, "栈上只应留下错误字符串");
    let message = s.top_string();
    // lua_o_chunkid 会剥掉 chunkname 的 '@' sigil
    assert!(
      message.starts_with("malformed: "),
      "错误必须以 chunkid 起头: {message}"
    );
    assert!(
      message.contains(expect),
      "错误消息应含 {expect}，实际 {message}"
    );
  }
}

/// 回归护栏：合法的最小 blob 必须照常装载，栈上留一个闭包
#[test]
fn minimal_blob_loads_as_closure() {
  let s = State::new();

  unsafe {
    assert_eq!(s.load(&empty_proto()), 0);
    assert_eq!(lua_gettop(s.l), 1);
    assert_eq!(lua_type(s.l, -1), LuaType::Function as i32);
  }
}

/// debugname 的 string id 越界（strings 表为空）不得越界读 TempBuffer
#[test]
fn string_id_beyond_empty_string_table_is_rejected() {
  let mut blob = Blob::default();

  blob.byte(VERSION).byte(TYPES_VERSION).varint(0);
  blob.varint(1);
  ProtoSpec {
    debugname: 1,
    ..ProtoSpec::default()
  }
  .write(&mut blob);
  blob.varint(0);

  assert_malformed(&blob.bytes, "debug name string id is out of range");
}

/// debugname 的 string id 超出字符串表长度
#[test]
fn string_id_beyond_string_table_is_rejected() {
  let mut blob = Blob::default();

  blob.byte(VERSION).byte(TYPES_VERSION).varint(1);
  blob.varint(1).raw(b"a");
  blob.varint(1);
  ProtoSpec {
    debugname: 7,
    ..ProtoSpec::default()
  }
  .write(&mut blob);
  blob.varint(0);

  assert_malformed(&blob.bytes, "debug name string id is out of range");
}

/// 字符串长度与 blob 尾部不一致（count 自洽但字节不够）
#[test]
fn truncated_string_body_is_rejected() {
  let mut blob = Blob::default();

  blob.byte(VERSION).byte(TYPES_VERSION).varint(1);
  blob.varint(64).raw(b"abc");

  assert_malformed(&blob.bytes, "string table is truncated");
}

/// sizecode 大于 blob 剩余字节：不得据此构造越界切片
#[test]
fn oversized_code_size_is_rejected() {
  let protos = [ProtoSpec {
    sizecode: Some(1000),
    ..ProtoSpec::default()
  }];

  assert_malformed(&proto_blob(&protos, 0), "code size is out of range");
}

/// 闭包常量引用未装载的 proto（protoCount=1 却取 fid=1）
#[test]
fn closure_proto_id_beyond_loaded_protos_is_rejected() {
  let constants = [LBC_CONSTANT_CLOSURE, 1];
  let protos = [ProtoSpec {
    constants: &constants,
    nconstants: 1,
    ..ProtoSpec::default()
  }];

  assert_malformed(&proto_blob(&protos, 0), "closure proto id is out of range");
}

/// 内层 proto 前向引用：合法字节码的被引用 proto 总先于引用点写入
#[test]
fn forward_inner_proto_ref_is_rejected() {
  let protos = [
    ProtoSpec {
      inner: &[1],
      ..ProtoSpec::default()
    },
    ProtoSpec::default(),
  ];

  assert_malformed(&proto_blob(&protos, 1), "inner proto id is out of range");
}

/// main proto id 越界
#[test]
fn main_proto_id_beyond_table_is_rejected() {
  assert_malformed(
    &proto_blob(&[ProtoSpec::default()], 5),
    "main proto id is out of range",
  );
}

/// protoCount=0 时 mainid 无从指向
#[test]
fn main_proto_id_with_empty_proto_table_is_rejected() {
  assert_malformed(&proto_blob(&[], 0), "main proto id is out of range");
}

/// 字符串常量 id 越界：走 read_string 的硬校验而非裸 .data.add
#[test]
fn string_constant_id_beyond_table_is_rejected() {
  let constants = [LBC_CONSTANT_STRING, 3];
  let protos = [ProtoSpec {
    constants: &constants,
    nconstants: 1,
    ..ProtoSpec::default()
  }];

  assert_malformed(
    &proto_blob(&protos, 0),
    "string constant id is out of range",
  );
}

/// size == 0：cpp 的 `read<uint8_t>` 是越界读，Rust 侧必须停在入口判定
#[test]
fn empty_blob_is_rejected() {
  assert_malformed(&[], "bytecode is empty");
}

/// 只有版本字节：读不出 typesversion
#[test]
fn version_only_blob_is_rejected() {
  assert_malformed(&[VERSION], "bytecode types version is truncated");
}

/// 版本号高于支持上限
#[test]
fn too_new_version_is_rejected() {
  let too_new = u8::try_from(LBC_VERSION_MAX.0 + 1).expect("上限 +1 必在 u8 内");

  assert_malformed(
    &[too_new],
    &format!(
      "bytecode version mismatch (expected [{}..{}], got {too_new})",
      LBC_VERSION_MIN.0, LBC_VERSION_MAX.0
    ),
  );
}

/// 版本号低于支持下限（1/2 已由更老的运行时发出）
#[test]
fn too_old_version_is_rejected() {
  let too_old = u8::try_from(LBC_VERSION_MIN.0 - 1).expect("下限 -1 必在 u8 内");

  assert_malformed(
    &[too_old],
    &format!(
      "bytecode version mismatch (expected [{}..{}], got {too_old})",
      LBC_VERSION_MIN.0, LBC_VERSION_MAX.0
    ),
  );
}

/// 常量体宽度大于剩余字节：tag 之后凑不满一个 f64
#[test]
fn truncated_number_constant_is_rejected() {
  let protos = [ProtoSpec {
    constants: &[LBC_CONSTANT_NUMBER, 1, 2, 3],
    nconstants: 1,
    truncate_after_constants: true,
    ..ProtoSpec::default()
  }];

  assert_malformed(&proto_blob(&protos, 0), "number constant is truncated");
}

/// vector 常量按 4 个 f32 逐个读：第 4 个分量落在 blob 末尾之外
#[test]
fn truncated_vector_constant_is_rejected() {
  let protos = [ProtoSpec {
    constants: &[LBC_CONSTANT_VECTOR, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
    nconstants: 1,
    truncate_after_constants: true,
    ..ProtoSpec::default()
  }];

  assert_malformed(&proto_blob(&protos, 0), "vector constant w is truncated");
}

/// 未知常量 tag：cpp 落 `default: LUAU_ASSERT(!"Unknown constant")`，
/// debug 下是崩溃，Rust 侧必须转成损坏字节码错误
#[test]
fn unknown_constant_tag_is_rejected() {
  let protos = [ProtoSpec {
    constants: &[UNKNOWN_CONSTANT_TAG],
    nconstants: 1,
    ..ProtoSpec::default()
  }];

  assert_malformed(&proto_blob(&protos, 0), "constant tag is unknown");
}

/// 字符串 varint 的续读位落在 blob 末尾之外
#[test]
fn truncated_string_table_varint_is_rejected() {
  let mut blob = Blob::default();

  blob.byte(VERSION).byte(TYPES_VERSION).byte(0x80);

  assert_malformed(&blob.bytes, "string table size is truncated");
}

/// mainid 的 varint 缺终止字节：不得把续读位当值读走
#[test]
fn truncated_main_proto_id_varint_is_rejected() {
  let mut bytes = proto_blob(&[ProtoSpec::default()], 0);
  let last = bytes.len() - 1;
  assert_eq!(bytes[last], 0, "mainid=0 应是单字节 varint");
  bytes[last] = 0x80;

  assert_malformed(&bytes, "main proto id is truncated");
}

/// 字符串常量 id 的 varint 截断：与 id 越界同走 read_string 的 Option 通道
#[test]
fn truncated_string_constant_id_varint_is_rejected() {
  let protos = [ProtoSpec {
    constants: &[LBC_CONSTANT_STRING, 0x80],
    nconstants: 1,
    truncate_after_constants: true,
    ..ProtoSpec::default()
  }];

  assert_malformed(
    &proto_blob(&protos, 0),
    "string constant id is out of range",
  );
}

/// version == 0 通道：cpp 约定「其余字节即错误消息」，整段透传且以 chunkid 起头
#[test]
fn version_zero_payload_becomes_error_message() {
  let s = State::new();

  let mut blob = vec![0u8];
  blob.extend_from_slice(b"compiler refused to emit this");

  unsafe {
    assert_eq!(s.load(&blob), 1, "version 0 必须以错误返回");
    assert_eq!(lua_gettop(s.l), 1);
    // lua_o_chunkid 剥掉 '@' sigil 后与 payload 直接拼接，无 ": " 分隔
    assert_eq!(
      s.top_string(),
      concat!("malformed", "compiler refused to emit this")
    );
  }
}

/// version == 0 且无 payload：拼接退化为纯 chunkid，不得越读
#[test]
fn version_zero_without_payload_is_rejected() {
  let s = State::new();

  unsafe {
    assert_eq!(s.load(&[0]), 1);
    assert_eq!(lua_gettop(s.l), 1);
    assert_eq!(s.top_string(), "malformed");
  }
}

/// version == 0 的 payload 含嵌入 NUL：按 size 而非 strlen 透传
#[test]
fn version_zero_payload_keeps_embedded_nul() {
  let s = State::new();

  unsafe {
    assert_eq!(s.load(&[0, b'a', 0, b'b']), 1);
    assert_eq!(lua_gettop(s.l), 1);
    assert_eq!(s.top_string(), "malformeda\0b");
  }
}
