use core::ffi::c_void;

/// 覆盖率回调（cpp `lua_CoverageFunction` 的 `const char* funcName` 面，review.md §10
/// 收形为原生 `Option<&[u8]>` 串体窗——`None` 即原 null；`hits` 收 `&[i32]`，
/// `size` 参数由切片长度承载）。
///
/// DELIBERATE DEVIATION（review.md §0）：形签从 `unsafe extern "C-unwind" fn` 收为
/// `unsafe fn`——实测全仓无真实 C 消费者（`ulua-capi` 不导出 getcoverage 面，消费者
/// 仅 `ulua-repl-cli` 与 conformance 测试门面的 Rust 回调），C ABI 声明是虚假边界；
/// cpp 仅行为 oracle。`unsafe fn` 保留：`context` 为调用方裸指针，回调内解引用。
pub type LuaCoverage = Option<
  for<'a, 'b> unsafe fn(
    context: *mut c_void,
    function: Option<&'a [u8]>,
    linedefined: i32,
    depth: i32,
    hits: &'b [i32],
  ),
>;
