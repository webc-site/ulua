//! loadsafe（不可信字节码反序列化）的畸形/截断输入契约测试。
//!
//! 对照 cpp/VM/src/lvmload.cpp：cpp 只靠 `TempBuffer::operator[]` 与
//! `LUAU_ASSERT` 兜底（release 编译掉即越界读），Rust 侧必须把
//! 不可信 id / 长度 / count 转成「损坏字节码」错误（返回 1 + 错误字符串）。
//!
//! blob 拼装器（`Blob`/`ProtoSpec`/`proto_blob`）与 VM 状态守卫（`State`）
//! 在 `tests/common`；栈顶串/第 4 lane 探针仅本文件用，就近放文件尾。

use core::{mem::size_of, slice::from_raw_parts};

use ulua_common::enums::luau_bytecode_tag::LuauBytecodeTag;
use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{lua_gettop::lua_gettop, lua_tolstring::lua_tolstring_ref, lua_type::lua_type},
  macros::{lua_vector_size::LUA_VECTOR_SIZE, ttype::ttype},
  type_aliases::t_value::TValue,
};

#[path = "common/mod.rs"]
mod common;
#[path = "common/state.rs"]
mod state;

use common::{Blob, ProtoSpec, TYPES_VERSION, VERSION, load, proto_blob};
use state::State;

/// 上游 `LBC_CONSTANT_*` 未定义的值（cpp `LUAU_ASSERT(!"Unexpected constant kind")`）
const LBC_CONSTANT_UNKNOWN: u8 = 200;

/// 测试用 chunkname：`lua_o_chunkid` 会剥掉 `@` sigil，故错误前缀是 `malformed`
const CHUNK: &str = "@malformed";

/// 空 proto 的最小合法 blob
fn empty_proto() -> Vec<u8> {
  proto_blob(&[], &[ProtoSpec::default()], 0)
}

/// 加载失败：返回 1，栈顶为以 chunkid 起头的损坏字节码错误
fn assert_malformed(blob: &[u8], expect: &str) {
  let s = State::new();

  unsafe {
    assert_eq!(load(s.l, CHUNK, blob), 1, "畸形字节码必须以错误返回");
    assert_eq!(lua_gettop(s.l), 1, "栈上只应留下错误字符串");
    let message = top_string(&s);
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
    assert_eq!(load(s.l, CHUNK, &empty_proto()), 0);
    assert_eq!(lua_gettop(s.l), 1);
    assert_eq!(lua_type(s.l, -1), LuaType::Function as i32);
  }
}

/// debugname 的 string id 越界（strings 表为空）不得越界读 TempBuffer
#[test]
fn string_id_beyond_empty_string_table_is_rejected() {
  let protos = [ProtoSpec {
    debugname: 1,
    ..ProtoSpec::default()
  }];

  assert_malformed(
    &proto_blob(&[], &protos, 0),
    "debug name string id is out of range",
  );
}

/// debugname 的 string id 超出字符串表长度
#[test]
fn string_id_beyond_string_table_is_rejected() {
  let protos = [ProtoSpec {
    debugname: 7,
    ..ProtoSpec::default()
  }];

  assert_malformed(
    &proto_blob(&[b"a"], &protos, 0),
    "debug name string id is out of range",
  );
}

/// 字符串长度与 blob 尾部不一致（count 自洽但字节不够）
#[test]
fn truncated_string_body_is_rejected() {
  let mut blob = Blob::default();

  blob.byte(VERSION).byte(TYPES_VERSION).varint(1);
  blob.varint(64).raw(b"abc");

  assert_malformed(&blob.bytes, "bytecode string table is truncated");
}

/// sizecode 大于 blob 剩余字节：不得据此构造越界切片
#[test]
fn oversized_code_size_is_rejected() {
  let protos = [ProtoSpec {
    sizecode: Some(1000),
    ..ProtoSpec::default()
  }];

  assert_malformed(&proto_blob(&[], &protos, 0), "code size is out of range");
}

/// 闭包常量引用未装载的 proto（protoCount=1 却取 fid=1）
#[test]
fn closure_proto_id_beyond_loaded_protos_is_rejected() {
  let constants = [LuauBytecodeTag::LBC_CONSTANT_CLOSURE.0 as u8, 1];
  let protos = [ProtoSpec {
    constants: &constants,
    nconstants: 1,
    ..ProtoSpec::default()
  }];

  assert_malformed(
    &proto_blob(&[], &protos, 0),
    "closure proto id is out of range",
  );
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

  assert_malformed(
    &proto_blob(&[], &protos, 1),
    "inner proto id is out of range",
  );
}

/// main proto id 越界
#[test]
fn main_proto_id_beyond_table_is_rejected() {
  assert_malformed(
    &proto_blob(&[], &[ProtoSpec::default()], 5),
    "main proto id is out of range",
  );
}

/// protoCount=0 时 mainid 无从指向
#[test]
fn main_proto_id_with_empty_proto_table_is_rejected() {
  assert_malformed(&proto_blob(&[], &[], 0), "main proto id is out of range");
}

/// 字符串常量 id 越界：走 read_string 的硬校验而非裸 .data.add
#[test]
fn string_constant_id_beyond_table_is_rejected() {
  let constants = [LuauBytecodeTag::LBC_CONSTANT_STRING.0 as u8, 3];
  let protos = [ProtoSpec {
    constants: &constants,
    nconstants: 1,
    ..ProtoSpec::default()
  }];

  assert_malformed(
    &proto_blob(&[], &protos, 0),
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
  let too_new = u8::try_from(LuauBytecodeTag::LBC_VERSION_MAX.0 + 1).expect("上限 +1 必在 u8 内");

  assert_malformed(
    &[too_new],
    &format!(
      "bytecode version mismatch (expected [{}..{}], got {too_new})",
      LuauBytecodeTag::LBC_VERSION_MIN.0,
      LuauBytecodeTag::LBC_VERSION_MAX.0
    ),
  );
}

/// 版本号低于支持下限（1/2 已由更老的运行时发出）
#[test]
fn too_old_version_is_rejected() {
  let too_old = u8::try_from(LuauBytecodeTag::LBC_VERSION_MIN.0 - 1).expect("下限 -1 必在 u8 内");

  assert_malformed(
    &[too_old],
    &format!(
      "bytecode version mismatch (expected [{}..{}], got {too_old})",
      LuauBytecodeTag::LBC_VERSION_MIN.0,
      LuauBytecodeTag::LBC_VERSION_MAX.0
    ),
  );
}

/// 常量体宽度大于剩余字节：tag 之后凑不满一个 f64
#[test]
fn truncated_number_constant_is_rejected() {
  let protos = [ProtoSpec {
    constants: &[LuauBytecodeTag::LBC_CONSTANT_NUMBER.0 as u8, 1, 2, 3],
    nconstants: 1,
    truncate_after_constants: true,
    ..ProtoSpec::default()
  }];

  assert_malformed(&proto_blob(&[], &protos, 0), "number constant is truncated");
}

/// vector 常量按 4 个 f32 逐个读：第 4 个分量落在 blob 末尾之外
#[test]
fn truncated_vector_constant_is_rejected() {
  let protos = [ProtoSpec {
    constants: &[
      LuauBytecodeTag::LBC_CONSTANT_VECTOR.0 as u8,
      1,
      2,
      3,
      4,
      5,
      6,
      7,
      8,
      9,
      10,
      11,
      12,
    ],
    nconstants: 1,
    truncate_after_constants: true,
    ..ProtoSpec::default()
  }];

  assert_malformed(
    &proto_blob(&[], &protos, 0),
    "vector constant w is truncated",
  );
}

/// 未知常量 tag：cpp 落 `default: LUAU_ASSERT(!"Unexpected constant kind")`，
/// release 编译掉后带着未消费的 payload 继续错位解析后面的指令/行号；
/// Rust 侧必须收口成损坏字节码错误（fail-closed）。
#[test]
fn unknown_constant_tag_is_rejected() {
  let constants = [LBC_CONSTANT_UNKNOWN];
  let protos = [ProtoSpec {
    constants: &constants,
    nconstants: 1,
    ..ProtoSpec::default()
  }];

  assert_malformed(&proto_blob(&[], &protos, 0), "constant tag is unknown");
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
  let mut bytes = proto_blob(&[], &[ProtoSpec::default()], 0);
  let last = bytes.len() - 1;
  assert_eq!(bytes[last], 0, "mainid=0 应是单字节 varint");
  bytes[last] = 0x80;

  assert_malformed(&bytes, "main proto id is truncated");
}

/// 字符串常量 id 的 varint 截断：与 id 越界同走 read_string 的 `ReadStringError::Truncated` 通道
#[test]
fn truncated_string_constant_id_varint_is_rejected() {
  let protos = [ProtoSpec {
    constants: &[LuauBytecodeTag::LBC_CONSTANT_STRING.0 as u8, 0x80],
    nconstants: 1,
    truncate_after_constants: true,
    ..ProtoSpec::default()
  }];

  assert_malformed(
    &proto_blob(&[], &protos, 0),
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
    assert_eq!(load(s.l, CHUNK, &blob), 1, "version 0 必须以错误返回");
    assert_eq!(lua_gettop(s.l), 1);
    // lua_o_chunkid 剥掉 '@' sigil 后与 payload 直接拼接，无 ": " 分隔
    assert_eq!(
      top_string(&s),
      concat!("malformed", "compiler refused to emit this")
    );
  }
}

/// version == 0 且无 payload：拼接退化为纯 chunkid，不得越读
#[test]
fn version_zero_without_payload_is_rejected() {
  let s = State::new();

  unsafe {
    assert_eq!(load(s.l, CHUNK, &[0]), 1);
    assert_eq!(lua_gettop(s.l), 1);
    assert_eq!(top_string(&s), "malformed");
  }
}

/// version == 0 的 payload 含嵌入 NUL：按 size 而非 strlen 透传
#[test]
fn version_zero_payload_keeps_embedded_nul() {
  let s = State::new();

  unsafe {
    assert_eq!(load(s.l, CHUNK, &[0, b'a', 0, b'b']), 1);
    assert_eq!(lua_gettop(s.l), 1);
    assert_eq!(top_string(&s), "malformeda\0b");
  }
}

/// LBC_CONSTANT_VECTORD：32 字节 payload 必须被完整消费，并按上游
/// `setvvalue`（float 分支的 `float(x)`）窄化进内联 3-lane 槽，第 4 分量丢弃。
#[test]
fn vectord_constant_is_consumed_and_narrowed() {
  let mut constants = Blob::default();

  constants.byte(LuauBytecodeTag::LBC_CONSTANT_VECTORD.0 as u8);
  for value in [1.5f64, -2.25, 3.75, 9.5] {
    constants.raw(&value.to_le_bytes());
  }

  let protos = [ProtoSpec {
    constants: &constants.bytes,
    nconstants: 1,
    ..ProtoSpec::default()
  }];

  let s = State::new();
  unsafe {
    assert_eq!(
      load(s.l, CHUNK, &proto_blob(&[], &protos, 0)),
      0,
      "VECTORD 必须可装载"
    );
    assert_eq!(lua_gettop(s.l), 1);

    let p = (*(*s.l).top.sub(1)).as_closure().inner.l.p;
    assert_eq!((*p).sizek, 1);
    let k = (*p).k;
    assert_eq!(ttype!(k), LuaType::Vector as u32);
    assert_eq!((*k).as_vector(), [1.5f32, -2.25, 3.75]);
    // 3-lane 构建里「第 4 分量」槽与 tt 重叠：上游 `condvector4` 既不读也不写，
    // 这里按字节确认它仍是向量 tag，`9.5` 没有被窄化写进去。
    assert_eq!(
      lane4_slot(&*k),
      (LuaType::Vector as i32).to_le_bytes(),
      "VECTORD 的第 4 分量不得落到向量槽位里"
    );
  }
}

/// LBC_CONSTANT_VECTOR（f32 形态）的第 4 分量同样只用于推进流偏移：
/// 上游 `read<float>` 后 `(void)w`，Rust 侧必须消费 16 字节 payload 但不写入。
#[test]
fn vector_constant_fourth_lane_is_consumed_not_stored() {
  let mut constants = Blob::default();

  constants.byte(LuauBytecodeTag::LBC_CONSTANT_VECTOR.0 as u8);
  for lane in [1.0f32, -2.0, 3.0, f32::from_bits(0xDEAD_BEEF)] {
    constants.raw(&lane.to_le_bytes());
  }

  let protos = [ProtoSpec {
    constants: &constants.bytes,
    nconstants: 1,
    ..ProtoSpec::default()
  }];

  let s = State::new();
  unsafe {
    assert_eq!(load(s.l, CHUNK, &proto_blob(&[], &protos, 0)), 0);
    assert_eq!(lua_gettop(s.l), 1);

    let k = (*(*s.l).top.sub(1)).as_closure().inner.l.p;
    let constant = (*k).k;
    assert_eq!(ttype!(constant), LuaType::Vector as u32);
    assert_eq!((*constant).as_vector(), [1.0f32, -2.0, 3.0]);
    assert_ne!(
      lane4_slot(&*constant),
      f32::from_bits(0xDEAD_BEEF).to_le_bytes(),
      "常量流里的第 4 分量哨兵不得出现在 TValue 里"
    );
    assert_eq!(
      lane4_slot(&*constant),
      (LuaType::Vector as i32).to_le_bytes(),
      "第 4 分量槽必须仍是向量 tag"
    );
  }
}

/// 栈顶字符串（错误消息）；非字符串即失败。
fn top_string(s: &State) -> String {
  // Safety: `s.l` 存活且栈顶已断言为字符串，`lua_tolstring_ref` 走纯字符串分支恒
  // 返回全字节切片（失败态 `None` 为不可达路径，以 expect 收敛为测试断言）。
  unsafe {
    assert_eq!(
      lua_type(s.l, -1),
      LuaType::String as i32,
      "栈顶必须是错误字符串"
    );
    let bytes = lua_tolstring_ref(s.l, -1).expect("栈顶为串，tolstring 必返回 Some");
    String::from_utf8_lossy(bytes).into_owned()
  }
}

/// TValue 的原字节视图（`LUA_VECTOR_SIZE == 3` 构建下共 16 字节）。
fn tvalue_bytes(t: &TValue) -> &[u8] {
  // Safety: 指针与长度都出自同一个 `&TValue`（addr-of 视图，非独立来源），对齐由
  // `*const u8` 放宽满足；u8 无无效位模式，按字节只读窥视任意 repr(C) 布局是安全的。
  unsafe { from_raw_parts((t as *const TValue).cast::<u8>(), size_of::<TValue>()) }
}

/// 「第 4 分量」在内存里占据的 4 字节。
///
/// `LUA_VECTOR_SIZE == 3` 时 `TValue` 是 `value.f`(8B) + `extra[0]`(4B) +
/// `tt`(4B)，`vvalue!` 把前 12 字节读成 `[f32; 3]`，于是第 4 个 lane 的槽位与
/// `tt` 完全重叠（cpp `lobject.h:132-151` 的 `condvector4` 在 3-lane 构建里既不
/// 读也不写它）。断言这 4 个字节仍是 tag，即可证明没有任何代码把第 4 分量的
/// 算术结果（或常量流里的哨兵位模式）写回向量。
fn lane4_slot(t: &TValue) -> [u8; 4] {
  let start = size_of::<[f32; LUA_VECTOR_SIZE as usize]>();
  tvalue_bytes(t)[start..start + 4]
    .try_into()
    .expect("TValue 第 4 lane 槽固定 4 字节")
}
