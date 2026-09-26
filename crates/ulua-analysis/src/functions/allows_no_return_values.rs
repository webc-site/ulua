/// C++ `static bool allowsNoReturnValues(const TypePackId tp)`.
use crate::{
  functions::{begin_type_pack::begin, follow_type, get_type},
  type_aliases::{error_type::ErrorType, type_pack_id::TypePackId},
};
pub fn allows_no_return_values(tp: TypePackId) -> bool {
  // 全部元素都是 ErrorType 才返回 true，遇到非 ErrorType 提前终止
  begin(tp).all(|ty| get_type::get::<ErrorType>(follow_type::follow(ty)).is_some())
}
