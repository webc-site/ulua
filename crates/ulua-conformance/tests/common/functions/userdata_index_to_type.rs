use crate::common::records::bytecode_types::LBC_TYPE_TAGGED_USERDATA_BASE;

/// `cpp/tests/ConformanceIrHooks.h` `userdataIndexToType`：把 userdata 索引编码成
/// 带 tag 的 LBC 类型码。
pub fn userdata_index_to_type(userdata_index: u8) -> u8 {
  LBC_TYPE_TAGGED_USERDATA_BASE + userdata_index
}
