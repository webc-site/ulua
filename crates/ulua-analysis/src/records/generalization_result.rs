use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneralizationResult {
  pub result: Option<TypeId>,
  pub was_replaced_by_generic: bool,
  pub resource_limits_exceeded: bool,
}

// Safety: `result` 为 `TypeId = *const Type`，令自动 Send 失效；它只是借用自类型 arena
// 的身份值，从不被解引用。只要该 arena 在 `GeneralizationResult` 存活期内有效，将其转移
// 到其它线程即可靠。
unsafe impl Send for GeneralizationResult {}
// Safety: 同上——裸 `TypeId` 仅作只读身份值、从不解引用，其余字段为普通 `bool`；
// 只读共享引用无数据竞争，故 `Sync` 成立。
unsafe impl Sync for GeneralizationResult {}
