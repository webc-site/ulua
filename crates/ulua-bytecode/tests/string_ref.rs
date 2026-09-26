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
  assert_eq!(s.as_str().unwrap(), "héllo");
}

#[test]
fn eq_same_pointer_is_true_without_content_read() {
  let buf = b"shared".to_vec();
  let a = StringRef::from_slice(&buf);
  let b = StringRef::from_slice(&buf);
  assert!(a == b);
  assert_eq!(a, b);
}

#[test]
fn eq_different_pointers_zero_length() {
  let buf_a = [1u8; 4];
  let buf_b = [2u8; 4];
  // Safety: `length == 0` 时只要求指针可转成空切片；两个缓冲本身在作用内存活。
  let a = unsafe { StringRef::new(buf_a.as_ptr() as *const _, 0) };
  // Safety: 同上。
  let b = unsafe { StringRef::new(buf_b.as_ptr() as *const _, 0) };
  // 新表示把空视图归一为 `Cow::Borrowed(&[])`，比较只看内容：零长度必然相等。
  assert!(a == b);
  assert_eq!(a, b);
}

#[test]
fn eq_null_against_non_null_falls_back_to_pointer() {
  let real = StringRef::from_slice(b"x");
  let null = StringRef::default();
  assert_ne!(real, null);
  assert_ne!(null, real);
}

#[test]
fn new_from_c_char_matches_from_slice() {
  let bytes = b"c api\0\xff".to_vec();
  // Safety: `bytes` 在断言期间存活且 `length == bytes.len()`。
  let via_c = unsafe { StringRef::new(bytes.as_ptr() as *const _, bytes.len()) };
  let via_slice = StringRef::from_slice(&bytes);
  assert_eq!(via_c, via_slice);
  assert_eq!(via_c.as_bytes(), b"c api\0\xff");
}

/// `owned`（`Cow::Owned`）与 `borrowed`（`Cow::Borrowed`）按内容互等：
/// 图序列化路径（`StringRef::owned`）拷贝的字节要能在 stringTable 里正确去重。
#[test]
fn owned_matches_borrowed_by_content() {
  let borrowed = StringRef::from_slice(b"shared-key");
  let owned = StringRef::owned(b"shared-key".to_vec());

  assert_eq!(borrowed, owned);
  assert_eq!(owned, borrowed);

  let hash = |s: &StringRef| {
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    h.finish()
  };
  assert_eq!(hash(&borrowed), hash(&owned));
  assert_eq!(owned.as_bytes(), b"shared-key");
  assert_eq!(owned.as_str().unwrap(), "shared-key");
  assert!(!owned.is_null());
}

/// `owned` 对任意生命周期可用（`StringRef<'static>` 经协变适配 `StringRef<'a>`），
/// 这是 `toFunctionBytecode` 路径绕开借用冲突的关键。
#[test]
fn owned_is_usable_at_shorter_lifetime() {
  fn take<'a>(s: StringRef<'a>) -> usize {
    s.len()
  }
  let o = StringRef::owned(b"abcd".to_vec());
  assert_eq!(take(o), 4);
}
