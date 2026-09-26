use alloc::string::String;
use core::ptr::null;

use ulua_ast::records::ast_local::AstLocal;
use ulua_common::records::dense_hash_table::DenseDefault;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
  pub(crate) name: String,
  pub(crate) ctx: *const AstLocal,
}

impl Identifier {
  pub fn new(name: String, ctx: *const AstLocal) -> Self {
    Self { name, ctx }
  }
}

/// 规范空标识符 `("", null)`，与旧调用点内联传参
/// `Identifier::new(String::new(), null())` 逐位等价（cpp `Identifier()` 默认值）。
impl Default for Identifier {
  fn default() -> Self {
    Self::new(String::new(), null())
  }
}

/// `DenseHashMap<Identifier, ..>::default()` 的空键占位来源，与 cpp toposort
/// （`Analysis/src/TopoSortStatements.cpp:203` `DenseHashMap<Identifier, Node*,
/// IdentifierHash> map;`）的 `map{}` 起步同构。
/// 契约：本哨兵**不是占用哨兵**——槽位占用由位图判定（见 ulua-common
/// `dense_hash_table` 模块文档），真实键 `("", null)`（无名字绑定 ctx 的
/// `mk_name` 产物）照常可插入/命中/擦除，与哨兵占位共存；`default()` 门面与
/// 旧形 `new(("", null))` 起步行为逐位一致，撞车系上游固有行为（行为 oracle
/// 对齐，不改键语义）。
impl DenseDefault for Identifier {
  fn dense_default() -> Self {
    Self::default()
  }
}

impl Identifier {
  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn ctx(&self) -> *const AstLocal {
    self.ctx
  }
}
