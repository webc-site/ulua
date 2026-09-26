use crate::common::records::bytecode_types::LBC_TYPE_TAGGED_USERDATA_BASE;

/// `cpp/tests/ConformanceIrHooks.h` `typeToUserdataIndex`：还原带 tag 的 LBC 类型码。
pub fn type_to_userdata_index(r#type: u8) -> u8 {
  // 上游注释：下溢会把 type 变成不可能等于任何 kUserdata* 常量的值（cpp 靠 switch
  // default 兜底），故这里用 wrapping_sub 而不是 checked_sub。
  r#type.wrapping_sub(LBC_TYPE_TAGGED_USERDATA_BASE)
}
