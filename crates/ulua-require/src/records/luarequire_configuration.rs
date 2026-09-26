use core::ffi::{c_char, c_void};

use crate::enums::{
  luarequire_config_status::LuarequireConfigStatus,
  luarequire_navigate_result::LuarequireNavigateResult,
  luarequire_write_result::LuarequireWriteResult,
};

// C-ABI 边界回调的统一形态（对应 C++ `Require.h` 的函数指针签名）。
// 本模块是全 crate 唯一的 FFI 边界：内部一律使用字节串 / `&T`，
// 只有这里的指针与 `c_char` 属于跨语言契约，不做 Rust 化改写。
//
// 本文件形态已定格（勿再按"消裸指针"口径立项）：
// - 结构与字段名/函数指针类型属跨 crate pub 签名，禁改：ulua-repl-cli 的
//   `require_config_init` 逐字段赋值 `unsafe extern "C-unwind" fn`，
//   ulua-cli-test 等以 `LuarequireConfigurationInit` 回调用该 init；
// - 配置整体以 `zeroed()` userdata 存放、由 `config_init` 回调以
//   `*mut luarequire_Configuration` 跨 C-ABI 填充，`#[repr(C)]` 是该契约的
//   一部分；
// - `l`/`ctx` 分别是 lua_State 与 lightuserdata 的不透明裸句柄，
//   按 cyclic_placeholder 批裁定归 ulua-vm C-API 真边界；
// - 可空回调（cpp 侧 null 函数指针）全部以 `Option<fn>` 收口。

/// 仅凭 `l`/`ctx` 即可作答的查询回调，返回类型按查询各异
/// （`is_module_present`/`get_config_status`/`get_luau_config_timeout` 共用此形态）。
pub(crate) type QueryFn<T> = unsafe extern "C-unwind" fn(l: *mut c_void, ctx: *mut c_void) -> T;

/// 无附加输入、返回导航结果的回调（`to_parent`）。
pub(crate) type NavFn = QueryFn<LuarequireNavigateResult>;

/// 带一个 NUL 结尾字节串输入、返回导航结果的回调
/// （`reset`/`jump_to_alias`/`to_alias_override`/`to_alias_fallback`/`to_child` 共用）。
pub(crate) type NavWithInputFn = unsafe extern "C-unwind" fn(
  l: *mut c_void,
  ctx: *mut c_void,
  input: *const c_char,
) -> LuarequireNavigateResult;

/// 以 NUL 结尾字节串入参做布尔判定的回调（`is_require_allowed`）。
pub(crate) type PredicateWithInputFn =
  unsafe extern "C-unwind" fn(l: *mut c_void, ctx: *mut c_void, input: *const c_char) -> bool;

/// 向调用方缓冲区写入字节串的回调
/// （`get_chunkname`/`get_loadname`/`get_cache_key`/`get_config` 共用）。
pub(crate) type WriterFn = unsafe extern "C-unwind" fn(
  l: *mut c_void,
  ctx: *mut c_void,
  buffer: *mut c_char,
  buffer_size: usize,
  size_out: *mut usize,
) -> LuarequireWriteResult;

/// 带一个 NUL 结尾字节串输入、向缓冲区写入字节串的回调（`get_alias`）。
pub(crate) type AliasWriterFn = unsafe extern "C-unwind" fn(
  l: *mut c_void,
  ctx: *mut c_void,
  alias: *const c_char,
  buffer: *mut c_char,
  buffer_size: usize,
  size_out: *mut usize,
) -> LuarequireWriteResult;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct luarequire_Configuration {
  pub is_require_allowed: Option<PredicateWithInputFn>,
  pub reset: Option<NavWithInputFn>,
  pub jump_to_alias: Option<NavWithInputFn>,
  pub to_alias_override: Option<NavWithInputFn>,
  pub to_alias_fallback: Option<NavWithInputFn>,
  pub to_parent: Option<NavFn>,
  pub to_child: Option<NavWithInputFn>,
  pub is_module_present: Option<QueryFn<bool>>,
  pub get_chunkname: Option<WriterFn>,
  pub get_loadname: Option<WriterFn>,
  pub get_cache_key: Option<WriterFn>,
  pub get_config_status: Option<QueryFn<LuarequireConfigStatus>>,
  pub get_alias: Option<AliasWriterFn>,
  pub get_config: Option<WriterFn>,
  pub get_luau_config_timeout: Option<QueryFn<i32>>,
  // `load` 可能抛出 Lua 错误（如被 require 模块运行期失败）：panic 式
  // `lua_d_throw` 会从回调中 unwind，边界必须允许 unwind（`C-unwind`）。
  pub load: Option<
    unsafe extern "C-unwind" fn(
      l: *mut c_void,
      ctx: *mut c_void,
      path: *const c_char,
      chunkname: *const c_char,
      loadname: *const c_char,
    ) -> i32,
  >,
}

/// 配置初始化回调（对应 C++ `luarequire_Configuration_init`）。
pub type LuarequireConfigurationInit =
  Option<unsafe extern "C-unwind" fn(config: *mut luarequire_Configuration)>;
