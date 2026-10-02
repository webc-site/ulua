//! 内建迭代器游标（FORGLOOP 进度指针）写入的唯一构造收口。
//!
//! 与 [`crate::functions::set_iterator_done`] 同族：iterator-done 标记与游标共用
//! `LU_TAG_ITERATOR` extra tag，载荷为「下一步游标」的整数值指针形态（cpp
//! `setpvalue(ra + 2, (void*)(uintptr_t)(index + 1), LU_TAG_ITERATOR)`）。本文件把
//! 该整数↔指针折算与 tag 常量收口到一处，读侧（FORGLOOP 快路径与 codegen 回调）
//! 经 `pvalue! … as usize as i32` 逆运算取回游标。
//!
//! B1c 票面 1 / r16-v6 收准：写入面全退 unsafe——`slot` 转 `&mut TValue`（借用承载
//! 非空/对齐/独占可写），句柄经 safe 构造子 [`Slot::from_mut`] 构造、再走
//! [`TValue::set_pvalue`] 安全写面，函数体零 `unsafe`、签名 `pub fn`。载荷/tag/写面
//! 逐位语义不变（消费点 `luau_execute.rs` FORGLOOP 臂与 code-gen 回调仅调用形变）。

use core::ffi::c_void;

use crate::{
  macros::lu_tag_iterator::LU_TAG_ITERATOR, records::slot::Slot, type_aliases::t_value::TValue,
};

/// 把「访问完 `index` 之后的下一个游标」写入迭代器槽：载荷为
/// `(index + 1) as usize as *mut c_void`，extra tag 为 `LU_TAG_ITERATOR`。
///
/// 调用序契约（正确性，非内存安全；r16-v6 起形参为 `&mut TValue`——借用承载
/// 非空/对齐/独占可写，[`Slot::from_mut`] 为 safe 构造子，体零 `unsafe`）：`slot`
/// 须为本帧内建迭代协议预留的可写栈槽（FORGLOOP 的 `ra+2`）；`index` 为当前
/// 数组+哈希段合并游标（有界 i32）。载荷为地址形态的纯游标数值，读侧只做整数
/// 折算（`as usize as i32` 原样取回），永不被解引用。
pub fn set_iterator_index(slot: &mut TValue, index: i32) {
  Slot::from_mut(slot)
    .as_mut()
    .set_pvalue((index + 1) as usize as *mut c_void, LU_TAG_ITERATOR);
}
