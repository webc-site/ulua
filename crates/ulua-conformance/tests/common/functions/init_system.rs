//! cpp `tests/main.cpp:517-527` 的 `initSystem()`。

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use core::arch::asm;

/// MXCSR bit 15：flush-to-zero mode（`_MM_SET_FLUSH_ZERO_MODE` 操作的那一位）。
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const FLUSH_TO_ZERO_BIT: u32 = 1 << 15;

/// MXCSR bit 6：denormals-are-zero mode（`_MM_SET_DENORMALS_ZERO_MODE` 操作的那一位）。
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const DENORMALS_ARE_ZERO_BIT: u32 = 1 << 6;

/// 关闭 x86/x86_64 的 flush-to-zero 与 denormals-are-zero。
///
/// 上游函数体是 `_MM_SET_FLUSH_ZERO_MODE(_MM_FLUSH_ZERO_OFF)` +
/// `_MM_SET_DENORMALS_ZERO_MODE(_MM_DENORMALS_ZERO_OFF)`（`main.cpp:519-526`），用于
/// 保证依赖非规格化数的用例结果稳定（`conformance/strconv.luau` 里那条
/// “If the assert below fires it may indicate floating point denormalized values”）。
/// 这里用 `stmxcsr`/`ldmxcsr` 读改写同样这两个位：`core::arch` 的
/// `_mm_getcsr`/`_mm_setcsr` 在新工具链上已 deprecated，会在 `-D warnings` 的构建
/// 门禁上直接报错。
///
/// 上游 `main()` 只在进程启动时调用一次；MXCSR 是**每线程**状态，而 libtest/nextest
/// 把用例放到各自的线程上跑，因此本端口在每次 `run_conformance` 进入时调用 —— 即
/// “该线程开始执行用例之前”，与上游在每个执行线程上的效果一致；写操作幂等，重复
/// 调用无副作用。
///
/// 上游的 `#if defined(CODEGEN_TARGET_X64)` 门控在本端口等价于 x86/x86_64 架构门控：
/// `CODEGEN_TARGET_X64`（`ulua-code-gen/src/macros/codegen_target_x_64.rs`）当且仅当
/// `target_arch = "x86" | "x86_64"` 为真，而这两个架构之外也不存在 MXCSR 寄存器。
pub fn init_system() {
  #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
  unsafe {
    let mut mxcsr: u32 = 0;
    // Safety: 两条指令只经由 `[{}]` 内存操作数访问 `mxcsr` 这个本地 4 字节槽
    // （`stmxcsr` 写入、`ldmxcsr` 读出后装载控制寄存器），不改动栈指针（`nostack`）；
    // `stmxcsr` 一侧不声明 `readonly`/`nomem`，编译器因此保守地认为内存可能被写，
    // 其后对 `mxcsr` 的读取不会被重排到写入之前。MXCSR 是线程私有状态，不影响其他
    // 线程；写入值仅清了上面两个非保留的模式位。
    asm!("stmxcsr [{}]", in(reg) &mut mxcsr, options(nostack));
    mxcsr &= !(FLUSH_TO_ZERO_BIT | DENORMALS_ARE_ZERO_BIT);
    asm!("ldmxcsr [{}]", in(reg) &mxcsr, options(nostack, readonly));
  }
}
