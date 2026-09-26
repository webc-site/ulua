//! 值键 `default()` 门面单测（b14-dense-keys）：`AstName`/`Entry` 补上
//! [`DenseDefault`] 后，`DenseHashMap/DenseHashSet::default()` 与旧形
//! `new(AstName::new())` / `new(Entry{null,0,EOF})` 起步逐位等价；并按占用位图
//! 契约（见 ulua-common `dense_hash_table` 模块文档）验证"位图判占用、哨兵可
//! 存取"：哨兵（null AstName / 默认 Entry）作为普通键可 get/insert/erase，且
//! 擦除哨兵不波及真实键。

use core::ptr::null_mut;

use ulua_ast::{
  enums::type_lexer::Type,
  records::{ast_local::AstLocal, ast_name::AstName, entry::Entry, entry_hash::EntryHash},
};
use ulua_common::records::{
  dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet, dense_hash_table::DenseDefault,
};

/// `AstName::dense_default()` 必须是 null 指针哨兵，与 `Default`/`new()` 一致
/// （真实键由 AstNameTable 驻留、恒非 null，故不撞车）。
#[test]
fn ast_name_dense_default_is_null_sentinel() {
  assert!(AstName::dense_default().is_null());
  assert!(AstName::dense_default().value.is_null());
  assert_eq!(AstName::dense_default(), AstName::new());
  assert_eq!(AstName::dense_default(), AstName::default());
}

/// `DenseHashMap<AstName, _>::default()` 与旧形 `new(AstName::default())` 起步
/// 等价；null 哨兵键可正常存取擦，真实驻留键不受波及（位图判占用契约）。
#[test]
fn ast_name_map_default_facade_stores_null_key() {
  let mut map: DenseHashMap<AstName, *mut AstLocal> = DenseHashMap::default();
  let legacy: DenseHashMap<AstName, *mut AstLocal> = DenseHashMap::new(AstName::new());
  assert_eq!(map.size(), legacy.size(), "default() 与 new(哨兵) 起步等价");
  assert!(map.is_empty());

  let real = AstName::from_static(b"foo");
  map.insert(real, null_mut::<AstLocal>());
  map.insert(AstName::new(), null_mut::<AstLocal>());
  assert_eq!(map.size(), 2, "哨兵键与真实键并存，占用由位图判定");
  assert!(map.contains(&AstName::new()), "null 哨兵键应可命中");
  assert!(map.find(&real).is_some());

  map.erase(&AstName::new());
  assert!(!map.contains(&AstName::new()));
  assert!(
    map.find(&real).is_some(),
    "擦除哨兵键不得波及真实键（旧代哨兵比较实现的丢键陷阱）"
  );
  map.erase(&real);
  assert!(map.empty());
}

/// `DenseHashSet<AstName>::default()`：哨键 try_insert 的 fresh 语义与指针键
/// 门面测试对齐——首次 fresh、重复不 fresh。
#[test]
fn ast_name_set_default_facade_sentinel_fresh_semantics() {
  let mut set: DenseHashSet<AstName> = DenseHashSet::default();
  assert!(
    set.try_insert(AstName::default()),
    "首次插入哨兵键应为 fresh"
  );
  assert!(
    !set.try_insert(AstName::default()),
    "重复插入哨兵键不应判 fresh"
  );
  set.insert(AstName::from_static(b"bar"));
  assert_eq!(set.size(), 2);
  assert!(set.contains(&AstName::default()));
  set.erase(&AstName::default());
  assert!(!set.contains(&AstName::default()));
  assert_eq!(set.size(), 1);
}

/// `Entry::dense_default()` 必须等于 `Entry::default()`（null 名/0 长/EOF），
/// 即 AstNameTable 旧形内联字面量的逐位同源哨兵。
#[test]
fn entry_dense_default_matches_legacy_literal() {
  let e = Entry::dense_default();
  assert!(e.value.is_null());
  assert_eq!(e.length, 0);
  assert_eq!(e.r#type, Type::EOF);
}

/// `DenseHashSet<Entry, EntryHash>::default()`（AstNameTable::data 的门面化形
/// 态）：哨兵 Entry 可存取擦；等内容的非 null 指针条目与哨兵在同一等价类
/// （PartialEq 按长度+内容、hash 按名字字节），但位图占用判定独立于键值。
#[test]
fn entry_set_default_facade_stores_sentinel_key() {
  let mut set: DenseHashSet<Entry, EntryHash> = DenseHashSet::default();
  assert!(set.empty(), "default() 起步为空表");

  let sentinel = Entry::default();
  assert!(set.try_insert(sentinel), "首次插入哨兵 Entry 应为 fresh");
  assert!(!set.try_insert(sentinel), "重复插入哨兵 Entry 不应判 fresh");

  // 真实驻留条目：非 null 指针、非零长度。
  let real = Entry::new(AstName::from_static(b"abc"), 3, Type::EOF);
  set.insert(real);
  assert_eq!(set.size(), 2);
  assert!(set.contains(&sentinel));

  // 内容等价的异指针条目命中同一项（AstNameTable 查重语义），与哨兵占用互不干扰；
  // `type` 取不同值以证明相等/哈希关系只看名字字节。
  let alias = Entry::new(AstName::from_static(b"abc"), 3, Type(255));
  assert!(set.contains(&alias), "按内容等价命中真实条目");

  set.erase(&sentinel);
  assert!(!set.contains(&sentinel));
  assert!(
    set.contains(&real),
    "擦除哨兵 Entry 不得波及真实条目（位图判占用契约）"
  );
  assert_eq!(set.size(), 1);
}
