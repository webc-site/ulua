use core::ptr::null;

use ulua_ast::records::ast_name::AstName;
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_table::DenseDefault};

use crate::type_aliases::type_id::TypeId;

/// C++ `ClassDeclRecord`（`ConstraintGenerator.h:70-75`）。
#[derive(Debug, Clone)]
pub struct ClassDeclRecord {
  /// 类实例类型（instance ExternType）。
  pub ty: TypeId,
  /// 成员名（属性/方法）→ 原型阶段登记的 BlockedType；visit(AstStatClass) 时就地填充。
  pub member_types: DenseHashMap<AstName, TypeId>,
  /// 显式 `__init` 生成的 `.new` 构造函数的 BlockedType 占位。
  pub new_blocked_ty: Option<TypeId>,
}

impl DenseDefault for ClassDeclRecord {
  fn dense_default() -> Self {
    Self {
      ty: null(),
      member_types: DenseHashMap::default(),
      new_blocked_ty: None,
    }
  }
}
