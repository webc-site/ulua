//! StringRef 语义测试，对应 C++ `BytecodeBuilder::StringRef::operator==` 与
//! `StringRefHash::operator()`（BytecodeBuilder.cpp）。

use std::{
  collections::hash_map::DefaultHasher,
  hash::{Hash, Hasher},
};

use ulua_bytecode::records::string_ref::StringRef;

/// 等内容字符串经两个不同缓冲进入时必须判等——
/// C++ 语义：双方指针非空时按内容比较，否则退化为指针相等。
#[test]
fn eq_same_content_from_different_buffers() {
  let buf_a = b"f".to_vec();
  let buf_b = b"f".to_vec();
  let a = StringRef::from_slice(&buf_a);
  let b = StringRef::from_slice(&buf_b);

  assert_eq!(a, b);
  assert_eq!(b, a);
}

#[test]
fn ne_different_content() {
  let a = StringRef::from_slice(b"f");
  let b = StringRef::from_slice(b"g");

  assert_ne!(a, b);
}

#[test]
fn ne_different_length() {
  let a = StringRef::from_slice(b"ab");
  let b = StringRef::from_slice(b"abc");

  assert_ne!(a, b);
}

/// {null, 0} 空键哨兵必须与自身相等（DenseHashMap 的空键约定）。
#[test]
fn null_sentinel_eq_itself() {
  let a = StringRef::default();
  let b = StringRef::default();

  assert!(a.is_null());
  assert!(a.is_empty());
  assert_eq!(a, b);
}

/// C++ StringRefHash 对内容范围哈希：等内容必须等哈希。
#[test]
fn hash_by_content() {
  let buf_a = b"identifier".to_vec();
  let buf_b = b"identifier".to_vec();

  let hash = |s: &StringRef| {
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    h.finish()
  };

  assert_eq!(
    hash(&StringRef::from_slice(&buf_a)),
    hash(&StringRef::from_slice(&buf_b))
  );
}

#[test]
fn as_bytes_roundtrip() {
  let s = StringRef::from_slice(b"hello");
  assert_eq!(s.as_bytes(), b"hello");
  assert_eq!(s.len(), 5);

  let s = StringRef::from_slice("héllo".as_bytes());
  assert!(s.as_str().is_ok());
  assert_eq!(s.as_str().unwrap(), "héllo");
}
