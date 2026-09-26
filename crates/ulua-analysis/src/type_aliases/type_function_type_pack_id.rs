use crate::records::type_function_type_pack_var::TypeFunctionTypePackVar;

pub type TypeFunctionTypePackId = *const TypeFunctionTypePackVar;

/// 节点解引用门面：将 runtime `type_pack_arena` 内的 pack 节点指针转换为普通 Rust 借用。
pub trait AsTypeFunctionTypePack {
  /// 解引用为 pack 引用（由 runtime `type_pack_arena` 保活）。
  fn as_pack(&self) -> &TypeFunctionTypePackVar;

  /// 可空形态解引用为 Option 引用。
  fn as_pack_opt(&self) -> Option<&TypeFunctionTypePackVar>;
}

impl AsTypeFunctionTypePack for TypeFunctionTypePackId {
  #[inline]
  fn as_pack(&self) -> &TypeFunctionTypePackVar {
    // SAFETY: `TypeFunctionTypePackId` 是 runtime `type_pack_arena` 分配的节点指针，
    // arena 块地址稳定，外层借用存续期间节点恒存活且只读访问无并存可变别名。
    unsafe { &**self }
  }

  #[inline]
  fn as_pack_opt(&self) -> Option<&TypeFunctionTypePackVar> {
    if self.is_null() {
      None
    } else {
      Some(self.as_pack())
    }
  }
}
