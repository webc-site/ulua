//! `Identifier` 键面 `default()` 门面单测（b16-identifier-dense 收口波）：
//! analysis 本地 `DenseDefault for Identifier` impl（impl 落定义包、孤儿合法，
//! 对照 compiler `Symbol` 案先例）落地后，
//! `DenseHashMap<Identifier, NodeId, IdentifierHash>::default()` 与旧形
//! `new(Identifier::new(String::new(), null()))` 起步逐位等价；并按占用位图
//! 契约（见 ulua-common `dense_hash_table` 模块文档）验证"位图判占用、空键
//! 占位可存取"：与占位同形的真实键 `("", null)` 照常 insert/命中/erase，
//! 与 cpp toposort（`TopoSortStatements.cpp:203` `DenseHashMap<Identifier,
//! Node*, IdentifierHash> map;` 的 `map{}` 起步）同构，撞车系上游固有行为。

use core::ptr::null;

use ulua_analysis::records::{
  identifier::Identifier, identifier_hash::IdentifierHash, node::NodeId,
};
use ulua_ast::records::ast_local::AstLocal;
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_table::DenseDefault};

type IdentMap = DenseHashMap<Identifier, NodeId, IdentifierHash>;

/// `Identifier::default()`（即 `dense_default()`）必须是 `("", null)`，与旧调用点
/// 内联传参 `Identifier::new(String::new(), null())` 逐位等价。
#[test]
fn identifier_dense_default_is_empty_name_null_ctx() {
  let sentinel = Identifier::default();
  assert_eq!(sentinel, Identifier::new(String::new(), null()));
  assert_eq!(
    Identifier::dense_default(),
    sentinel,
    "DenseDefault::dense_default() 与 Default::default() 同形"
  );
  assert_eq!(sentinel.name(), "");
  assert!(sentinel.ctx().is_null());
}

/// `default()` 门面与旧形 `new(("", null))` 起步等价；与空键占位同形的真实键
/// `("", null)`（位图判占用，占位非保留键）照常存取，擦除不波及并存真键。
#[test]
fn identifier_map_default_facade_stores_sentinel_shaped_key() {
  let mut map: IdentMap = IdentMap::default();
  let legacy: IdentMap = IdentMap::new(Identifier::new(String::new(), null()));
  assert_eq!(
    map.size(),
    legacy.size(),
    "default() 与 new(旧哨兵) 起步等价"
  );
  assert!(map.is_empty());

  let sentinel = Identifier::default();
  // 真键二：同名空串、借用栈上活对象的地址伪装 ctx（仅按指针哈希/比较，不解引用）。
  let marker: u8 = 0;
  let with_ctx = Identifier::new(String::new(), &marker as *const u8 as *const AstLocal);

  map.insert(sentinel.clone(), 7);
  map.insert(with_ctx.clone(), 9);
  assert_eq!(map.size(), 2, "占位同形键与真键并存，占用由位图判定");
  assert_eq!(map.find(&sentinel), Some(&7), "（“”, null）形状键应可命中");
  assert_eq!(map.find(&with_ctx), Some(&9));

  map.erase(&sentinel);
  assert!(!map.contains(&sentinel));
  assert!(
    map.find(&with_ctx).is_some(),
    "擦除占位同形键不得波及真键（旧代哨兵比较实现的丢键陷阱）"
  );
  map.erase(&with_ctx);
  assert!(map.empty());
}

/// 占位同形键 `try_insert` 的 fresh 语义与真键一致：首次 fresh、重复不 fresh——
/// 其参与的是位图占用判定，不触发旧代"键等于空键即视为空槽"的误判。
#[test]
fn identifier_map_sentinel_shaped_try_insert_fresh_semantics() {
  let mut map: IdentMap = IdentMap::default();
  let sentinel = Identifier::default();
  assert!(
    map.try_insert(sentinel.clone(), 1).1,
    "首次插入占位同形键应为 fresh"
  );
  assert!(
    !map.try_insert(sentinel.clone(), 2).1,
    "重复插入占位同形键不应判 fresh"
  );
  assert_eq!(
    map.find(&sentinel),
    Some(&1),
    "重复 try_insert 不得覆盖旧值"
  );
  assert_eq!(map.size(), 1);
}
