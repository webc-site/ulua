use ulua_ast::records::{
  ast_array::AstArray, ast_generic_type::AstGenericType, ast_name::AstName, node_handle::Nodes,
};

/// 「泛型名表」只读视图：旧 `AstArray<*mut AstGenericType>` 与句柄化
/// `Nodes<AstGenericType>` 两种存储形态共用同一判定（cpp `isGeneric` 的
/// 名字线性查找），records 全量引用化后旧实现随之退役。
pub trait GenericList {
  fn contains_name(&self, name: AstName) -> bool;
}

impl GenericList for AstArray<*mut AstGenericType> {
  #[inline]
  fn contains_name(&self, name: AstName) -> bool {
    // Safety 前提见 `AstArray::iter_nodes`：元素为 parser 登记的存活指针。
    self.iter_nodes().any(|gt| gt.name == name)
  }
}

impl GenericList for Nodes<AstGenericType> {
  #[inline]
  fn contains_name(&self, name: AstName) -> bool {
    self.iter().any(|gt| gt.name == name)
  }
}

/// 空泛型名表：调用点无泛型上下文（cpp `getType(ty, {}, ...)` 的传空数组）时
/// 使用的单例视图，避免为 `Default::default()` 引入存储形态歧义。
pub struct NoGenerics;

impl GenericList for NoGenerics {
  #[inline]
  fn contains_name(&self, _name: AstName) -> bool {
    false
  }
}

pub fn is_generic<G: GenericList + ?Sized>(name: AstName, generics: &G) -> bool {
  generics.contains_name(name)
}
