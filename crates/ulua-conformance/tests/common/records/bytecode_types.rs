//! conformance 侧 IR hook 用到的 `LBC_TYPE_*` 类型码。
//!
//! 全部由 `ulua_common` 的上游枚举定义（`cpp/Common/include/Luau/Bytecode.h:549-568`）
//! 推导，禁止在本 crate 内再写死数值：原先 5 个 `*_bytecode_type.rs` 各自定义本地
//! 常量且取值互相漂移，其中 `userdata_access_bytecode_type.rs` /
//! `userdata_namecall_bytecode_type.rs` 把兜底的 `LBC_TYPE_ANY` 写成了 `0`
//! （＝`LBC_TYPE_NIL`），而上游 `ConformanceIrHooks.h:379/:694` 的兜底是
//! `LBC_TYPE_ANY=15`（“类型未知”）。这个 u8 会经 `codegen_ir_hook_callbacks.rs`
//! 的回调直接喂给被测 CodeGen 的类型判断，误报成 nil 会让 CodeGen 走错分支。

use ulua_common::enums::luau_bytecode_type::LuauBytecodeType as luau_bytecode_type;

/// `cpp/Common/include/Luau/Bytecode.h:552` `LBC_TYPE_NUMBER`。
pub const LBC_TYPE_NUMBER: u8 = luau_bytecode_type::LBC_TYPE_NUMBER.0 as u8;

/// `cpp/Common/include/Luau/Bytecode.h:558` `LBC_TYPE_VECTOR`。
pub const LBC_TYPE_VECTOR: u8 = luau_bytecode_type::LBC_TYPE_VECTOR.0 as u8;

/// `cpp/Common/include/Luau/Bytecode.h:562` `LBC_TYPE_ANY`：类型未知。
pub const LBC_TYPE_ANY: u8 = luau_bytecode_type::LBC_TYPE_ANY.0 as u8;

/// `cpp/Common/include/Luau/Bytecode.h:564` `LBC_TYPE_TAGGED_USERDATA_BASE`。
pub const LBC_TYPE_TAGGED_USERDATA_BASE: u8 =
  luau_bytecode_type::LBC_TYPE_TAGGED_USERDATA_BASE.0 as u8;
