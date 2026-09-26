//! 内建迭代器游标（FORGLOOP 进度指针）写入的唯一构造收口。
//!
//! 与 [`crate::functions::set_iterator_done`] 同族：iterator-done 标记与游标共用
//! `LU_TAG_ITERATOR` extra tag，载荷为「下一步游标」的整数值指针形态（cpp
//! `setpvalue(ra + 2, (void*)(uintptr_t)(index + 1), LU_TAG_ITERATOR)`）。本文件把
//! 该整数↔指针折算与 tag 常量收口到一处，读侧（FORGLOOP 快路径与 codegen 回调）
//! 经 `pvalue! … as usize as i32` 逆运算取回游标。

use core::ffi::c_void;

use crate::{
  macros::{lu_tag_iterator::LU_TAG_ITERATOR, setpvalue::setpvalue},
  type_aliases::t_value::TValue,
};

/// 把「访问完 `index` 之后的下一个游标」写入迭代器槽：载荷为
/// `(index + 1) as usize as *mut c_void`，extra tag 为 `LU_TAG_ITERATOR`。
///
/// # Safety
/// `slot` 须指向本帧内建迭代协议预留的可写栈槽（FORGLOOP 的 `ra+2`）；`index` 为
/// 当前数组+哈希段合并游标（有界 i32）。载荷为地址形态的纯游标数值，永不被解引用。
pub unsafe fn set_iterator_index(slot: *mut TValue, index: i32) {
  // Safety: 调用方契约保证 slot 为可写栈槽；整转指针是把游标按迭代器协议装箱为
  // 指针载荷（读侧 `as usize as i32` 原样取回），不构造可解引用的指针。
  unsafe { setpvalue!(slot, (index + 1) as usize as *mut c_void, LU_TAG_ITERATOR) };
}
