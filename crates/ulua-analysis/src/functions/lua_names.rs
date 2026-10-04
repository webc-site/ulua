//! 与 Lua VM 交互时用到的静态名字常量。
//!
//! 本 crate 不是 C ABI 实现层，故名字一律以**原生 `&[u8]` 常量**（'static、不含终止
//! NUL）表达：VM 侧 `lua_l_*`/`lua_*` 的字节窗门面（`set_field_bytes`、
//! `push_c_function` 的 `debugname` 等）直取切片，`LuaLReg::name` 字段本身即
//! `&'static [u8]`。既消灭了重复字面量，也让「哪些字符串是 VM 契约的一部分」集中可审。

/// type userdata 的元表名，同时是 `__type` 字段的值。
pub(crate) const TYPE: &[u8] = b"type";
/// 元表锁定提示文本（cpp `The metatable is locked`）。
pub(crate) const METATABLE_LOCKED: &[u8] = b"The metatable is locked";

// —— VM 侧保留的元方法/元表字段名 ——
/// 元表字段：类型名标记。
pub(crate) const FIELD_TYPE_TAG: &[u8] = b"__type";
/// 元表字段：锁定保护用的假元表。
pub(crate) const FIELD_METATABLE: &[u8] = b"__metatable";
/// 元方法：相等比较。
pub(crate) const FIELD_EQ: &[u8] = b"__eq";
/// 元方法：动态字段索引闭包。
pub(crate) const FIELD_INDEX_CLOSURE: &[u8] = b"__index";
/// `typeUserdataMethods` 里 `issubtypeof` 方法名。
pub(crate) const METHOD_IS_SUBTYPE_OF: &[u8] = b"issubtypeof";

// —— 类型函数在 Lua 栈上传递结构化值时的字段名 ——
/// 函数类型的参数类型包。
pub(crate) const FIELD_HEAD: &[u8] = b"head";
/// 函数类型的返回类型包。
pub(crate) const FIELD_TAIL: &[u8] = b"tail";
/// 索引器的键。
pub(crate) const FIELD_INDEX: &[u8] = b"index";
/// 索引器的值（读写同型时）。
pub(crate) const FIELD_RESULT: &[u8] = b"result";
/// 读侧索引器的值。
pub(crate) const FIELD_READ_RESULT: &[u8] = b"readresult";
/// 写侧索引器的值。
pub(crate) const FIELD_WRITE_RESULT: &[u8] = b"writeresult";
/// 属性的读类型。
pub(crate) const FIELD_READ: &[u8] = b"read";
/// 属性的写类型。
pub(crate) const FIELD_WRITE: &[u8] = b"write";

// —— 类型函数环境（沙箱）里的全局名 ——
/// 被屏蔽的全局函数：`gcinfo`。
pub(crate) const GLOBAL_GCINFO: &[u8] = b"gcinfo";
/// 被屏蔽的全局函数：`getfenv`。
pub(crate) const GLOBAL_GETFENV: &[u8] = b"getfenv";
/// 被屏蔽的全局函数：`newproxy`。
pub(crate) const GLOBAL_NEWPROXY: &[u8] = b"newproxy";
/// 被屏蔽的全局函数：`setfenv`。
pub(crate) const GLOBAL_SETFENV: &[u8] = b"setfenv";
/// 被屏蔽的全局函数：`pcall`。
pub(crate) const GLOBAL_PCALL: &[u8] = b"pcall";
/// 被屏蔽的全局函数：`xpcall`。
pub(crate) const GLOBAL_XPCALL: &[u8] = b"xpcall";
/// 替换为受限打印的全局名。
pub(crate) const GLOBAL_PRINT: &[u8] = b"print";
/// 复位随机数用的标准库表名。
pub(crate) const GLOBAL_MATH: &[u8] = b"math";
/// `math` 表里需复位的全局函数名。
pub(crate) const GLOBAL_RANDOMSEED: &[u8] = b"randomseed";
/// `types` 库的注册名。
pub(crate) const LIB_TYPES: &[u8] = b"types";
