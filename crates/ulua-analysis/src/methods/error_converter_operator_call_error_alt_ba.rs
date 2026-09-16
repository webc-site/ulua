use alloc::string::String;

use crate::records::{
  error_converter::ErrorConverter, unexpected_array_like_table_item::UnexpectedArrayLikeTableItem,
};

impl ErrorConverter {
  pub fn operator_call_57(&self, _error: &UnexpectedArrayLikeTableItem) -> String {
    String::from(
      "Unexpected array-like table item: the indexer key type of this table is not `number`.",
    )
  }
}
