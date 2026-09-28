//! 解释器热路径的寄存器窗口寻址封装（cpp `RA()` 族宏的 Rust 对应）。
//!
//! See also: ulua-code-gen `functions/vm_reg_op.rs`（形似义异：IR 操作数编号提取，
//! 与本宏无关，勿合并）。
//!
//! # Safety（由展开点 unsafe 上下文承担）
//! `l`/`base` 满足 `luau_execute` 入口契约：base 指向存活栈且 base..(*l).top 为
//! 当前帧寄存器窗口；`i` 为字节码 A/B 字段（u8，已由 LUAU_ASSERT 校验
//! `i < top - base`，release 下由帧布局保证）。断言/越界语义与 cpp 一致。
#[macro_export]
macro_rules! VM_REG {
  ($i:expr, $l:expr, $base:expr) => {{
    let i = $i;
    let l = $l;
    let base = $base;
    ulua_common::LUAU_ASSERT!((i as u32) < ((*l).top.offset_from(base) as u32));
    // 返回 `*mut TValue`（StkId）；调用方无需再 `as *mut TValue` 转换，
    // 传 `*const TValue` 形参时由 mut->const 隐式强转覆盖。
    base.add(i as usize)
  }};
}

pub use VM_REG;
