use crate::records::type_function_type::TypeFunctionType;

pub type TypeFunctionTypeId = *const TypeFunctionType;

/// 节点解引用门面：将 runtime `type_arena` 内的节点指针转换为普通 Rust 借用。
pub trait AsTypeFunctionType {
  /// 解引用为类型引用（由 runtime `type_arena` 保活）。
  fn as_type(&self) -> &TypeFunctionType;

  /// 可空形态解引用为 Option 引用。
  fn as_type_opt(&self) -> Option<&TypeFunctionType>;
}

impl AsTypeFunctionType for TypeFunctionTypeId {
  #[inline]
  fn as_type(&self) -> &TypeFunctionType {
    // SAFETY: `TypeFunctionTypeId` 是 runtime `type_arena` 分配的节点指针，
    // arena 块地址稳定，外层借用存续期间节点恒存活且只读访问无并存可变别名。
    unsafe { &**self }
  }

  #[inline]
  fn as_type_opt(&self) -> Option<&TypeFunctionType> {
    if self.is_null() {
      None
    } else {
      Some(self.as_type())
    }
  }
}
