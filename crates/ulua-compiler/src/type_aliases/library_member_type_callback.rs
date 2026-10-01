/// 宿主注册的库成员类型回调类型
pub type LibraryMemberTypeCallback =
  Option<unsafe extern "C-unwind" fn(library: *const u8, member: *const u8) -> i32>;
