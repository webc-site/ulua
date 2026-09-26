//! 内建迭代器「无更多值」标记（iterator-done sentinel）的唯一构造收口。
//!
//! 源形为 cpp `setpvalue(..., nullptr, LU_TAG_ITERATOR)`（lvmexecute.cpp FORGPREP*/
//! FORGLOOP）。§2 禁 `null_mut()` 哨兵：本文件把 null 载荷收敛为具名 setter，调用点
//! 不再出现裸 null。`lua_TValue` 的内存布局与 `LU_TAG_ITERATOR` 数值 tag 均不变
//! （JIT 经 `offset_of!` 消费布局，`translate_inst_for_g_prep_*` 仍按同一 tag 值生成
//! IR 写入）。读侧识别见 [`crate::enums::value_view::ValueView::IteratorDone`]。

use core::ptr::null_mut;

use crate::{
  macros::{lu_tag_iterator::LU_TAG_ITERATOR, setpvalue::setpvalue},
  type_aliases::t_value::TValue,
};

/// 向 `slot` 写入内建迭代器的「迭代结束/游标为 0」标记：base tag 取
/// LightUserData、extra tag 取 `LU_TAG_ITERATOR`、载荷为 null（即数组段起点
/// 游标 0，FORGLOOP 快路径以 `pvalue!` 读回作整数游标，从不解引用）。
///
/// # Safety
/// `slot` 须指向本帧内建迭代协议预留的可写栈槽（FORGPREP*/FORGLOOP 的 `ra+2`），
/// 且该槽按迭代器协议使用（读侧只做 null 判定与整数折算）。
pub unsafe fn set_iterator_done(slot: *mut TValue) {
  // Safety: 调用方契约保证 slot 为可写栈槽；null 载荷是协议游标值 0 的指针形态
  // 表示（对应 `set_iterator_index` 的 `index + 1` 编码在游标 -1 处），永不被解引用。
  unsafe { setpvalue!(slot, null_mut(), LU_TAG_ITERATOR) };
}
