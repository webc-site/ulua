use core::{ffi::c_char, str::from_utf8};

use ulua_common::{enums::luau_bytecode_type::LuauBytecodeType, functions::c_str::cstr_bytes};

const LBC_TYPE_TAGGED_USERDATA_BASE: u8 = LuauBytecodeType::LBC_TYPE_TAGGED_USERDATA_BASE.0 as u8;
const LBC_TYPE_TAGGED_USERDATA_END: u8 = LuauBytecodeType::LBC_TYPE_TAGGED_USERDATA_END.0 as u8;
const LBC_TYPE_OPTIONAL_BIT: u8 = LuauBytecodeType::LBC_TYPE_OPTIONAL_BIT.0 as u8;

/// C-ABI `const char* const* userdataTypes`（cpp `CodeGenOptions.h`）的安全视图。
///
/// cpp 里字符串存活期是隐式契约（宿主注册的类型名与编译选项字段同寿）；这里把
/// 契约锚定为对指针槽位的借用 `'a`：调用方传 `&options.compilation_options.userdata_types`，
/// 返回的 `&'a str` 至多活到该借用结束，堵住旧版「调用方任选无界 `'a`」可伪造
/// `'static` 字符串引用的 soundness 缺口。null 数组/内层 null 均降级为 `None`。
#[derive(Debug, Clone, Copy)]
pub struct UserdataTypes<'a> {
  raw: &'a *const *const c_char,
}

impl<'a> UserdataTypes<'a> {
  pub const fn new(raw: &'a *const *const c_char) -> Self {
    Self { raw }
  }

  /// 读取第 `index` 个类型名（对应 cpp `userdataTypes[type - LBC_TYPE_TAGGED_USERDATA_BASE]`）。
  pub fn get(self, index: usize) -> Option<&'a str> {
    let array = *self.raw;
    if array.is_null() {
      return None;
    }

    // Safety: `new` 的调用方契约保证 `array` 为可读的 `*const c_char` 数组，
    // 各槽位要么为 null，要么指向存活满 `'a` 的以 NUL 结尾字符串。
    let ptr = unsafe { array.add(index).read() };
    if ptr.is_null() {
      return None;
    }

    // Safety: ptr 已在上方判空为 null，此处非 null；UserdataTypes::new 的调用方契约保证
    // 该槽指向存活满 'a 的 NUL 结尾字符串，满足 `cstr_bytes` 门面（全仓唯一
    // `CStr::from_ptr` 收口点）对非空/有效/NUL 结尾的前置条件。
    let name: &'a [u8] = unsafe { cstr_bytes(ptr) };
    // Lua 宿主类型名按 UTF-8 展示；非法字节按缺省名处理（dump 展示路径）
    from_utf8(name).ok()
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
