use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct VariadicTypePack {
  pub(crate) ty: TypeId,
  pub(crate) hidden: bool,
}

impl VariadicTypePack {
  pub fn new(ty: TypeId) -> Self {
    Self { ty, hidden: false }
  }
}

// Safety: `ty: TypeId = *const Type` 借用自类型 arena，令自动 Send 失效。它仅作
// 变参包元素类型的身份值、从不解引用；只要 arena 在其存活期内有效，转移即可靠。
unsafe impl Send for VariadicTypePack {}
// Safety: 同上——裸 `TypeId` 仅只读身份，`hidden` 为 `bool`，共享引用不产生数据竞争。
unsafe impl Sync for VariadicTypePack {}
