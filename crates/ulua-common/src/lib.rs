extern crate alloc;

pub mod collections;
pub mod dfflag;
pub mod dfint;
pub mod enums;
pub mod fflag;
pub mod fint;
pub mod functions;
pub mod macros;
pub mod methods;
pub mod records;
pub mod type_aliases;

/// wasm 的最小 libc 面。在 `wasm32-unknown-unknown`（无 libc）上所有 shim 都
/// 需要；带 libc 的 wasm 目标（如 `wasm32-wasip1`）下整个模块门出，符号由
/// wasi-libc 提供，避免重定义。
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub mod wasm_libc;

/// 纯 Rust strtod 实现（fast-float2 驱动），全平台单一形态；wasm 的
/// `#[unsafe(no_mangle)]` C 垫片已随 b22 退役（零 extern 消费实证）。
pub mod strtod_shim;

/// 纯 Rust strtoull 实现（单次遍历状态机，C 语义对齐：0x 前缀 /
/// base 自动探测 / 溢出饱和 u64::MAX）。全平台可用，取代 VM 的 libc FFI。
pub mod strtoull_shim;

/// 平台高精度单调时钟封装（基于 `std::time::Instant`），
/// 供本 crate 与 `ulua-vm` 的单调计时与 `os.clock` 共用。
pub mod clock_shim;

// C++ 在命名空间作用域暴露；codegen_assert! 等宏直接引用
// `ulua_common::assert_call_handler`。标志命名空间即 `dfflag` / `dfint` /
// `fflag` / `fint` 四个模块，读取写作 `fflag::NAME.get()`。
pub use functions::assert_call_handler::assert_call_handler;
pub use records::f_value::set_luau_bool_flags;

// §8 留证：测 `ThreadContext` 的 pub(crate) 事件缓冲 `events`（外部测试无从读取，
// tests/thread_context.rs 只能比较实例地址），且 TimeTrace 打点宏默认展开为空操作，
// 无公开可驱动路径，保留 src。
#[cfg(test)]
mod timetrace_event_tests {
  use crate::{
    functions::{
      create_scope_data::create_scope_data, create_token::create_token,
      get_global_context::get_global_context,
    },
    records::thread_context::ThreadContext,
  };

  /// 打点事件进入 `ThreadContext` 缓冲的条数。
  ///
  /// 留在 `src` 的理由：断言读的是 `pub(crate)` 的事件缓冲 `events`，
  /// 外部测试无从读取（`tests/thread_context.rs` 只能退化为比较实例地址）。
  /// TimeTrace 的三个消费宏默认展开为空操作（`LUAU_ENABLE_TIME_TRACE` 关闭），
  /// 无可断言内容，故不在此覆盖；flag 默认值见 `tests/flag_defaults.rs`，
  /// `push_test_override` 的栈语义见 `ulua-conformance` 的 ScopedFValue 套件。
  #[test]
  fn timetrace_token_and_scope_data() {
    let tok_id = create_scope_data("testScope", "Category");
    let ctx = get_global_context();
    let tok_id2 = create_token(&ctx, "testScope2", "Category2");
    assert_ne!(tok_id, tok_id2);

    let mut thread_ctx = ThreadContext::new();
    thread_ctx.event_enter(tok_id);
    thread_ctx.event_argument("arg_name", "arg_value");
    thread_ctx.event_leave();
    assert_eq!(thread_ctx.events.len(), 4);
  }
}
