use crate::type_aliases::compile_constant::CompileConstant;

/// 宿主注册的库成员常量回调类型
pub type LibraryMemberConstantCallback = Option<
  unsafe extern "C-unwind" fn(
    library: *const u8,
    member: *const u8,
    constant: *mut CompileConstant,
  ),
>;
