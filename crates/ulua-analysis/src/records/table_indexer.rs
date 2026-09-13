use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TableIndexer {
  pub index_type: TypeId,
  pub index_result_type: TypeId,
  pub is_read_only: bool,
}

unsafe impl Send for TableIndexer {}
unsafe impl Sync for TableIndexer {}
