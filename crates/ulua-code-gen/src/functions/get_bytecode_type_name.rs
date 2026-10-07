use alloc::string::String;

use ulua_common::enums::luau_bytecode_type::LuauBytecodeType;

const LBC_TYPE_TAGGED_USERDATA_BASE: u8 = LuauBytecodeType::LBC_TYPE_TAGGED_USERDATA_BASE.0 as u8;
const LBC_TYPE_TAGGED_USERDATA_END: u8 = LuauBytecodeType::LBC_TYPE_TAGGED_USERDATA_END.0 as u8;
const LBC_TYPE_OPTIONAL_BIT: u8 = LuauBytecodeType::LBC_TYPE_OPTIONAL_BIT.0 as u8;

/// cpp `CodeGenOptions.h` 的 `const char* const* userdataTypes` 的惯用 Rust 形态：
/// 借用 `CompilationOptions.userdata_types`（`&'a [String]`），`get` 返回的
/// `&'a str` 与该借用同寿。越界下标降级为 `None`（cpp 的 null 槽语义）。
#[derive(Debug, Clone, Copy)]
pub struct UserdataTypes<'a> {
  names: &'a [String],
}

impl<'a> UserdataTypes<'a> {
  pub fn new(names: &'a [String]) -> Self {
    Self { names }
  }

  /// 读取第 `index` 个类型名（对应 cpp `userdataTypes[type - LBC_TYPE_TAGGED_USERDATA_BASE]`）。
  pub fn get(self, index: usize) -> Option<&'a str> {
    self.names.get(index).map(String::as_str)
  }
}

/// cpp `getBytecodeTypeName`（`IrDump.cpp:837`）。
pub fn get_bytecode_type_name(r#type: u8, userdata_types: UserdataTypes<'_>) -> &str {
  // Optional 位应由外部处理
  let r#type = r#type & !LBC_TYPE_OPTIONAL_BIT;

  if (LBC_TYPE_TAGGED_USERDATA_BASE..LBC_TYPE_TAGGED_USERDATA_END).contains(&r#type) {
    return userdata_types
      .get((r#type - LBC_TYPE_TAGGED_USERDATA_BASE) as usize)
      .unwrap_or("userdata");
  }

  // 基础类型名查表收敛到单一真相（见 LuauBytecodeType::base_name）
  match LuauBytecodeType(r#type as u16).base_name() {
    Some(name) => name,
    None => {
      ulua_common::LUAU_ASSERT!(false);
      "unknown"
    }
  }
}
