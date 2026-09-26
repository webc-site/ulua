//! 解释器常量表寻址封装（cpp `k` 窗口宏的 Rust 对应）。
//!
//! # Safety（由展开点 unsafe 上下文承担）
//! `k` 指向当前闭包 proto 的常量数组首元素（`cl` 存活时 `k` 在闭包存续期内有效），
//! `i` 为字节码 D 字段（已由 LUAU_ASSERT 校验 `i < sizek`，release 下由
//! 合法字节码保证）；与 cpp 的盲索引 + assert 语义一致。
//!
//! See also: ulua-code-gen `functions/vm_const_op.rs`（形似义异：IR 操作数编号
//! 提取）；其慢路径门面 `VmFrame::kv` 已单源复用本宏。
#[macro_export]
macro_rules! VM_KV {
  ($i:expr, $cl:expr, $k:expr) => {{
    let i = $i;
    let cl = $cl;
    let k = $k;
    ulua_common::LUAU_ASSERT!((i as u32) < ((*(*cl).inner.l.p).sizek as u32));
    // 返回 `*mut TValue`（与 `VM_REG!` 同一约定）：调用方无需再 `as *mut/*const TValue`
    // 转换，传 `*const TValue` 形参或解引用宏时由 mut→const 隐式强转覆盖。
    k.add(i as usize)
  }};
}

pub use VM_KV;
