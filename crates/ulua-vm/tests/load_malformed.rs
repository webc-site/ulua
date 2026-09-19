//! loadsafe（不可信字节码反序列化）的畸形/截断输入契约测试。
//!
//! 对照 cpp/VM/src/lvmload.cpp：cpp 只靠 `TempBuffer::operator[]` 与
//! `LUAU_ASSERT` 兜底（release 编译掉即越界读），Rust 侧必须把
//! 不可信 id / 长度 / count 转成「损坏字节码」错误（返回 1 + 错误字符串）。
//!
//! blob 手工按 v9（typesversion 2）格式拼装，不依赖编译器 crate：
//! `version | typesversion | stringCount | strings | protoCount | protos | mainid`。

use std::{
  ffi::c_char,
  ptr::{eq, null},
  slice::from_raw_parts,
};

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
const LBC_CONSTANT_STRING: u8 = 3;
const LBC_CONSTANT_CLOSURE: u8 = 6;

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
    unsafe {
      luau_load(
        self.l,
        c"@malformed".as_ptr(),
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
