//! Source: `VM/src/lvmexecute.cpp:71` (hand-ported)
//!
//! `uprefs` is the C flexible-array-member idiom (`TValue uprefs[1]` with
//! over-allocation) — indexing the Rust `[TValue; 1]` would PANIC for i >= 1,
//! so the element is reached via pointer arithmetic, exactly like C.
//!
//! # Safety（由展开点 unsafe 上下文承担）
//! `cl` 指向存活的 LClosure 且其 uprefs 尾部按 `nupvalues` 过分配（lua_f_new_lclosure
//! 布局保证）；`i` 为字节码 B 字段，已由 LUAU_ASSERT 校验 `i < nupvalues`。
//! 与 cpp 柔性数组成员寻址语义一致。
//!
//! See also: ulua-code-gen `functions/vm_upvalue_op.rs`（形似义异：IR 操作数编号
//! 提取，与本宏无关，勿合并）。

#[macro_export]
macro_rules! VM_UV {
  ($i:expr, $cl:expr) => {{
    let i = $i;
    let cl = $cl;
    ulua_common::LUAU_ASSERT!((i as u32) < ((*cl).nupvalues as u32));
    let l = &mut (*cl).inner.l;
    // 返回 `*mut TValue`（与 `VM_REG!` 同一约定），调用方免再 cast
    l.uprefs.as_mut_ptr().add(i as usize)
  }};
}

pub use VM_UV;
