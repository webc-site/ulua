//! 实现层模块清单。`pub` 项一一对应 cpp `Repl.h` / `ReplRequirer.h` 的导出，
//! `pub(crate)` 项对应同名 cpp 文件里的 `static`。

/// 「字符串写回 C 缓冲」三枚 require 回调的单源生成模板：`get_chunkname` /
/// `get_loadname` / `get_config` 签名同为
/// `(_l, ctx, buffer, buffer_size, size_out) -> LuarequireWriteResult`，流程同为
/// 「由 ctx 重建共享引用 → 构造待写串 → 交给 write 门面按协议写回」，只差串
/// 表达式；C-ABI 签名、`requirer(ctx)` 重建与两处安全契约（requirer 契约 /
/// writer 协议）在此单点表述一次。`$req` 为调用方可见的重建结果绑定名，
/// `$value` 须产出 `Option<&str>`（借自 `$req` 的 vfs，或本条语句内存活的临时量）。
macro_rules! requirer_write_cb {
  ($name:ident | $req:ident -> $value:expr) => {
    /// # Safety
    ///
    /// `ctx` 必须指向有效的 `ReplRequirer`；缓冲区参数遵循 `ulua-require` 协议。
    pub(crate) unsafe extern "C-unwind" fn $name(
      _l: *mut ::core::ffi::c_void,
      ctx: *mut ::core::ffi::c_void,
      buffer: *mut ::core::ffi::c_char,
      buffer_size: usize,
      size_out: *mut usize,
    ) -> ::ulua_require::enums::luarequire_write_result::LuarequireWriteResult {
      // Safety: ctx 是 create_cli_require_context 经 lua_newuserdatadtor 分配、
      // 以地址为键登记进 registry 的 ReplRequirer userdata 指针，require 链路单线程
      // 同步调用窗口内存活且无其它借用者（requirer 契约）。
      let $req = unsafe { $crate::functions::requirer_ref::requirer(ctx) };
      // Safety: 待写串借自 $req.vfs（req 与 ctx 指向对象同存活期，覆盖本调用窗口）
      // 或本条语句的临时量（write 调用结束前可读）；buffer/buffer_size/size_out
      // 遵循 writer 协议，write() 判空+限长后才写。
      unsafe { $crate::functions::write::write($value, buffer, buffer_size, size_out) }
    }
  };
}

/// 「只带 ctx（可另带一枚 C 串参数）的 require 探针回调」单源生成模板：
/// `to_parent` / `get_config_status` / `is_module_present` 与 `to_child` /
/// `jump_to_alias` / `reset` 六枚回调流程同为「由 ctx 重建 requirer 引用
/// （`$acq` 选共享 `requirer` 或独占 `requirer_mut`）→ 可选地把 C 串参数经
/// cstr_cow 门面即时转 owned → 单表达式产出结果」；C-ABI 签名、重建与两处
/// 安全契约（requirer 契约 / call_with_c_str 缓冲协议）在此单点表述一次。
/// `$req` / 串参数名为调用方可见的绑定名，`$body` 须产出 `$ret`。
macro_rules! requirer_ctx_cb {
  ($name:ident | $req:ident = $acq:ident -> $ret:ty => $body:expr) => {
    requirer_ctx_cb!(@fn $name | $req = $acq -> $ret, | $body);
  };
  ($name:ident | $req:ident = $acq:ident, $arg:ident -> $ret:ty => $body:expr) => {
    requirer_ctx_cb!(@fn $name | $req = $acq -> $ret, $arg | $body);
  };
  (@fn $name:ident | $req:ident = $acq:ident -> $ret:ty, $($arg:ident)? | $body:expr) => {
    /// # Safety
    ///
    /// `ctx` 必须指向有效的 `ReplRequirer`。
    pub(crate) unsafe extern "C-unwind" fn $name(
      _l: *mut ::core::ffi::c_void,
      ctx: *mut ::core::ffi::c_void,
      $($arg: *const ::core::ffi::c_char)?
    ) -> $ret {
      // Safety: ctx 是 create_cli_require_context 经 lua_newuserdatadtor 分配、以地址
      // 为键登记进 registry 的 ReplRequirer userdata 指针；require 导航回调单线程串行、
      // 同步调用窗口内存活，重建借用形态（$acq 的共享/独占）按其契约成立。
      let $req = unsafe { $crate::functions::requirer_ref::$acq(ctx) };
      $(
        // Safety: 该 C 串参数由 ulua-require 的 `RuntimeNavigationContext::call_with_c_str`
        // 交出（补 NUL、仅回调调用窗口内存活），本行经 cstr_cow 门面立即转 owned，不外存
        // 指针；取 lossy 文本而非字节：下游是 `&str` 形态的宿主文件系统接口。
        let $arg = unsafe { ::ulua_common::functions::c_str::cstr_cow($arg) };
      )?
      $body
    }
  };
}

/// C-ABI 回调外壳单源模板。`lua_getcounters` / `lua_getcoverage` 只接受
/// `Option<unsafe extern "C-unwind" fn(..*mut c_void..)>`（见 ulua-vm type_aliases），
/// 而真正的计数/覆盖逻辑写在同名**安全** Rust fn 里（`coverage_callback` 还带
/// `Write` 泛型，无法直接充当 C ABI）；VM 裸指针的判空与转借（cstr_cow/c_slice/
/// context 重建）全部收口在外壳臂内，核心 fn 只见 Rust 类型。本宏集中表述
/// 「FFI 外壳」这一共同形式：
/// 属性文档 + `$vis unsafe extern "C-unwind" fn 名(形参…) { 体 }` 骨架（体省略
/// `-> ()`，避免 `unused_unit`）。形参表与 `$body` 由臂给出，`$body` 内自写
/// `unsafe {}` 转调具体核心，使「调用了哪个不安全函数」在调用点可见——宏定义体内
/// 不含任何 metavariable 落进 unsafe 块，edition 2024 下不触发
/// `clippy::macro_metavars_in_unsafe`（与 ulua-vm `lua_lib_arm` 同纪律）。
macro_rules! c_abi_cb {
  ($(#[$meta:meta])* $vis:vis fn $name:ident($($arg:ident : $ty:ty),* $(,)?) $body:block) => {
    $(#[$meta])*
    $vis unsafe extern "C-unwind" fn $name($($arg : $ty),*) $body
  };
}

// —— cpp `Repl.h` / `ReplRequirer.h` 导出的入口（供 ulua-cli-test 夹具复用）——
pub mod create_cli_require_context;
pub mod get_completions;
pub mod repl_main;
pub mod require_config_init;
pub mod run_code;
pub mod setup_state;

// —— 以下为 Repl.cpp / ReplRequirer.cpp 的 crate 内部实现（cpp `static`）——
pub(crate) mod compile_source;
pub(crate) mod complete_indexer;
pub(crate) mod complete_partial_matches;
pub(crate) mod complete_repl;
pub(crate) mod copts;
pub(crate) mod counters_active;
pub(crate) mod counters_dump;
pub(crate) mod counters_function_callback;
pub(crate) mod counters_init;
pub(crate) mod counters_track;
pub(crate) mod counters_value_callback;
pub(crate) mod coverage_active;
pub(crate) mod coverage_callback;
pub(crate) mod coverage_dump;
pub(crate) mod coverage_init;
pub(crate) mod coverage_track;
pub(crate) mod create_dump_writer;
pub(crate) mod error_with_trace;
pub(crate) mod get_chunkname;
pub(crate) mod get_config;
pub(crate) mod get_config_status;
pub(crate) mod get_file_path;
pub(crate) mod get_loadname;
pub(crate) mod ic_get_completions;
pub(crate) mod is_module_present;
pub(crate) mod jump_to_alias;
pub(crate) mod load;
pub(crate) mod load_history;
pub(crate) mod lua_loadstring;
pub(crate) mod main;
pub(crate) mod profiler_dump;
pub(crate) mod profiler_loop;
pub(crate) mod profiler_start;
pub(crate) mod profiler_stop;
pub(crate) mod profiler_trigger;
pub(crate) mod requirer_ref;
pub(crate) mod reset;
pub(crate) mod run_file;
pub(crate) mod run_repl;
pub(crate) mod run_repl_impl;
pub(crate) mod sigint_callback;
pub(crate) mod sigint_handler_repl;
// 收口门面：Ctrl-C 处理的注册/撤回（libc FFI 与 null 协议值只留这里）
pub(crate) mod sigint_setup;
pub(crate) mod stack_function_name;
pub(crate) mod to_child;
pub(crate) mod to_parent;
pub(crate) mod write;
