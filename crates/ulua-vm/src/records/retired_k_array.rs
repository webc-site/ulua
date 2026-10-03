//! JIT call inlining（NAMECALL 阶段）：proto 常量表扩容的退役旧块链节点。
//!
//! 观测路径内联把 callee 字符串常量物化进 caller 常量表（`proto_k_intern_string`）
//! 时按「新块拷贝 + 换指针」扩容 `proto->k`。旧块不能立即释放——在途 native 帧
//! 的 `R_CONSTANTS` 寄存器仍指着它（指针稳定窗口覆盖到本帧退出），也不能泄漏——
//! VM 页池按块记账，`lua_close` 校验所有页归还（`memcatbytes` 归零）。退役链挂
//! 在 Proto 上，随 [`crate::functions::lua_f_freeproto::lua_f_freeproto`] 统一
//! 释放：proto 死亡即无闭包可引用，绝无在途帧，释放安全。

use crate::type_aliases::t_value::TValue;

/// 退役 k 数组节点（单链）。
#[repr(C)]
pub struct RetiredKArray {
  pub next: *mut RetiredKArray,
  pub ptr: *mut TValue,
  /// `ptr` 的 TValue 元素数（`lua_m_free` 的释放尺寸）。
  pub size: usize,
  pub memcat: u8,
}

impl Default for RetiredKArray {
  fn default() -> Self {
    Self {
      next: core::ptr::null_mut(),
      ptr: core::ptr::null_mut(),
      size: 0,
      memcat: 0,
    }
  }
}
