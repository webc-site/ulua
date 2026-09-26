use crate::records::string_ref::StringRef;

/// cpp `BytecodeBuilder::UserDataType`（`Bytecode/include/Luau/BytecodeBuilder.h`）。
/// `name` 与 cpp 一样是**视图**（指向上游注册 userdata 类型名的名表缓冲），不再拷贝成
/// `String`；builder 的 `'a` 就是这些名字必须存活的寿命，`finalize` 直接按值取用。
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub(crate) struct UserdataType<'a> {
  pub(crate) name: StringRef<'a>,
  pub(crate) name_ref: u32,
  pub(crate) used: bool,
}
