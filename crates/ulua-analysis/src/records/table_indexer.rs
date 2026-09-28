use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TableIndexer {
  pub index_type: TypeId,
  pub index_result_type: TypeId,
  pub is_read_only: bool,
}

// Safety: `index_type`/`index_result_type` 为 `TypeId = *const Type`，令自动 Send 失效；
// 它们仅借用自类型 arena 作身份值、从不被解引用。只要 arena 在 `TableIndexer` 存活期内
// 有效，将其转移到其它线程即可靠。
unsafe impl Send for TableIndexer {}
// Safety: 同上——裸 `TypeId` 仅作只读身份值、从不解引用，`is_read_only` 为普通 `bool`；
// 只读共享遍历不产生数据竞争，故 `Sync` 成立。
unsafe impl Sync for TableIndexer {}
