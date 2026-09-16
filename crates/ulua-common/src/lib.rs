extern crate alloc;

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
/// 需要；在带 libc 的 wasm（如 `wasm32-wasip1`，用于在 32 位指针平台跑套件）上，
/// 分配器 shim 在模块内部被关掉以避免与 wasi-libc 冲突，而 wasi 缺失的函数
/// （mmap stub 等）仍由本模块提供。
#[cfg(target_arch = "wasm32")]
pub mod wasm_libc;

/// 纯 Rust strtod 实现（fast-float2 驱动）。全平台可用；wasm 的
/// `#[unsafe(no_mangle)]` C 入口仍按 wasm 门控。
pub mod strtod_shim;

/// 纯 Rust strtoull 实现（单次遍历状态机，C 语义对齐：0x 前缀 /
/// base 自动探测 / 溢出饱和 u64::MAX）。全平台可用，取代 VM 的 libc FFI。
pub mod strtoull_shim;

/// 平台时钟 FFI 统一封装（QPC / mach / clock_gettime / clock 兜底），
/// 供本 crate 与 `ulua-vm` 的 `get_clock_timestamp` / `clock_timestamp` 共用。
pub mod clock_shim;

// C++ 在命名空间作用域暴露；codegen_assert! 等宏直接引用
// `ulua_common::assert_call_handler`。
// C++ 风格命名空间别名：代码以 `FFlag::Name` / `FInt::Name` 等路径读取。
pub use dfflag as DFFlag;
pub use dfint as DFInt;
pub use fflag as FFlag;
pub use fint as FInt;
pub use functions::{assert_call_handler::assert_call_handler, set_all_flags::set_all_flags};
pub use records::f_value::set_luau_bool_flags;

#[cfg(test)]
mod fastflag_timetrace_tests {
  use crate::{
    FFlag::DebugLuauTimeTracing,
    LUAU_TIMETRACE_ARGUMENT, LUAU_TIMETRACE_OPTIONAL_TAIL_SCOPE, LUAU_TIMETRACE_SCOPE,
    functions::{
      create_scope_data::create_scope_data, create_token::create_token,
      get_global_context::get_global_context,
    },
    records::thread_context::ThreadContext,
  };

  /// 宏定义的 flag 读取默认值；TimeTrace 消费宏默认展开为空操作
  /// （默认 `LUAU_ENABLE_TIME_TRACE` 关闭）。
  #[test]
  fn flag_default_and_timetrace_noops() {
    assert!(!DebugLuauTimeTracing.get());
    LUAU_TIMETRACE_SCOPE!("name", "category");
    LUAU_TIMETRACE_OPTIONAL_TAIL_SCOPE!("name", "category", 100);
    LUAU_TIMETRACE_ARGUMENT!("k", "v");
    DebugLuauTimeTracing.set(true);
    assert!(DebugLuauTimeTracing.get());
  }

  #[test]
  fn timetrace_token_and_scope_data() {
    let tok_id = create_scope_data("testScope", "Category");
    let ctx = get_global_context();
    let tok_id2 = create_token(&ctx, "testScope2", "Category2");
    assert_ne!(tok_id, tok_id2);

    let mut thread_ctx = ThreadContext::new();
    thread_ctx.event_enter_u16(tok_id);
    thread_ctx.event_argument("arg_name", "arg_value");
    thread_ctx.event_leave();
    assert_eq!(thread_ctx.events.len(), 4);
  }
}
