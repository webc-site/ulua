use crate::records::unexpected_array_like_table_item::UnexpectedArrayLikeTableItem;

impl UnexpectedArrayLikeTableItem {
  #[inline]
  pub fn operator_eq(&self, _other: &UnexpectedArrayLikeTableItem) -> bool {
    true
  }
}
