use alloc::string::String;
use core::option::Option;

use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetatableType {
  /// Should always be a TableType.
  pub(crate) table: TypeId,
  /// Should almost always either be a TableType or another MetatableType,
  /// though it is possible for other types (like AnyType and ErrorType) to
  /// find their way here sometimes.
  pub(crate) metatable: TypeId,
  pub(crate) synthetic_name: Option<String>,
}

impl MetatableType {
  pub fn table(&self) -> TypeId {
    self.table
  }

  pub fn metatable(&self) -> TypeId {
    self.metatable
  }

  pub fn synthetic_name(&self) -> Option<&str> {
    self.synthetic_name.as_deref()
  }
}

/// # Safety
///
/// `table`/`metatable` 为 `TypeId = *const Type`，令自动 Send 失效；它们仅是
/// 借用自类型 arena 的身份值，从不解引用。只要 arena 在 `MetatableType` 存活期内
/// 有效，将其转移到其它线程即可靠。
// Safety: table/metatable 仅是 arena 借出的裸指针身份值（TypeId），Send 只需转移
// 这层"地址拷贝"；其可解引用的前提（arena 存活）由使用方线程另行保证，与本 impl
// 要回答的"移动所有权是否破坏不变量"无关，移动不复制、无并发双持。
unsafe impl Send for MetatableType {}
/// # Safety
///
/// 同上：裸 `TypeId` 仅作只读身份，`synthetic_name` 为自有 `String`；共享引用
/// 不产生数据竞争。
// Safety: Sync 要求 &MetatableType 可跨线程共享；字段仅为不可变裸指针句柄与
// 不可变 String，共享借用下无可变状态、无内部可变性，读读并发不构成数据竞争。
unsafe impl Sync for MetatableType {}
