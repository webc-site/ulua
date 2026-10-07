//! 内建迭代器「无更多值」标记（iterator-done sentinel）的唯一构造收口。
//!
//! 源形为 cpp `setpvalue(..., nullptr, LU_TAG_ITERATOR)`（lvmexecute.cpp FORGPREP*/
//! FORGLOOP）。§2 禁 `null_mut()` 哨兵：本文件把 null 载荷收敛为具名 setter，调用点
//! 不再出现裸 null。`lua_TValue` 的内存布局与 `LU_TAG_ITERATOR` 数值 tag 均不变
//! （JIT 经 `offset_of!` 消费布局，`translate_inst_for_g_prep_*` 仍按同一 tag 值生成
//! IR 写入）。读侧识别见 [`crate::enums::value_view::ValueView::IteratorDone`]。
//!
//! B1c 票面 1 / r16-v6 收准：写入面全退 unsafe——`slot` 转 `&mut TValue`（借用承载
//! 非空/对齐/独占可写），句柄经 safe 构造子 [`Slot::from_mut`] 构造、再走
//! [`TValue::set_pvalue`] 安全写面，函数体零 `unsafe`、签名 `pub fn`。载荷/tag/写面
//! 逐位语义不变（消费点 `luau_execute.rs` FORGPREP*/FORGLOOP 臂与 code-gen 回调
//! 仅调用形变）。

use core::ptr::null_mut;

use crate::{
  macros::lu_tag_iterator::LU_TAG_ITERATOR, records::slot::Slot, type_aliases::t_value::TValue,
};

/// 向 `slot` 写入内建迭代器的「迭代结束/游标为 0」标记：base tag 取
/// LightUserData、extra tag 取 `LU_TAG_ITERATOR`、载荷为 null（即数组段起点
/// 游标 0，FORGLOOP 快路径以 `pvalue!` 读回作整数游标，从不解引用）。
///
/// 调用序契约（正确性，非内存安全；r16-v6 起形参为 `&mut TValue`——借用承载
/// 非空/对齐/独占可写，[`Slot::from_mut`] 为 safe 构造子，体零 `unsafe`）：`slot`
/// 须为本帧内建迭代协议预留的可写栈槽（FORGPREP*/FORGLOOP 的 `ra+2`），且该槽
/// 按迭代器协议使用（读侧只做 null 判定与整数折算）。null 载荷是协议游标值 0 的
/// 指针形态表示（对应 `set_iterator_index` 的 `index + 1` 编码在游标 -1 处），
/// 永不被解引用；`set_pvalue` 写面与旧 `setpvalue!` 宏体逐位一致。
pub fn set_iterator_done(slot: &mut TValue) {
  Slot::from_mut(slot)
    .as_mut()
    .set_pvalue(null_mut(), LU_TAG_ITERATOR);
}
