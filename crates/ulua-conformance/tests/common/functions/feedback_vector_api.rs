//! feedback vector / Proto 裸内存遍历用例的安全门面（review.md §0/§2）。
//!
//! [`Proto`] 及其 `code`/`p`/`feedbackvec` 子数组、`FeedbackVectorSlot` 的联合体
//! 读取、`ecbdata` 断言数据都是 VM 内部裸内存边界的固有 `unsafe`；本模块把每个动作
//! 各自收口成一次性 `unsafe`，用例侧只做 safe 调用。集中契约：
//!
//! - **`top` / `*mut Proto` 契约**：由 `FeedbackVectorFixture::load()` 及其 `p` 数组交回、
//!   在本用例 [`fixture.run`](crate::common::records::feedback_vector_fixture) 前后均存活的
//!   GC proto 指针（cpp FeedbackVector.test.cpp 同名布线前提）。
//! - **`slot` 契约**：`feedback_slot(f, i)` 返回的 `i < f.feedbackvecsize` 槽指针；`kind`
//!   为 CallTarget 时其 `data.call_target` 联合体读取才合法（各用例逐字段核验 kind）。
//! - **`code` 下标契约**：`code_at`/`set_code_at` 的 `index` 落在 `f.code` 的
//!   `[0, f.sizecode)` 内（用例取 `call_target.pc + 1`）。

use ulua_vm::{
  enums::feedback_vector_slot_kind::FeedbackVectorSlotKind,
  records::{feedback_vector_slot::FeedbackVectorSlot, lua_state::LuaState, proto::Proto},
  type_aliases::instruction::Instruction,
};

use crate::common::{
  functions::assert_inliner_data::install_assert_inliner_data,
  records::assert_inliner_data::AssertInlinerData,
};

/// `*(*top).p.add(i)`：取顶层 proto 的第 `i` 个编译期子 proto。
pub fn child_proto(top: *mut Proto, i: usize) -> *mut Proto {
  // Safety: `top` 存活且其 `p` 数组至少含 `i + 1` 个子 proto 指针。
  unsafe { *(*top).p.add(i) }
}

/// `(*f).feedbackvec.add(i)`：取 proto 第 `i` 个反馈槽地址。
pub fn feedback_slot(f: *mut Proto, i: usize) -> *mut FeedbackVectorSlot {
  // Safety: `f` 存活且 `i < (*f).feedbackvecsize`。
  unsafe { (*f).feedbackvec.add(i) }
}

/// 读反馈槽 `kind`。
pub fn slot_kind(slot: *mut FeedbackVectorSlot) -> FeedbackVectorSlotKind {
  // Safety: `slot` 指向存活的反馈槽。
  unsafe { (*slot).kind }
}

/// 读 CallTarget 槽的 `pc`。
pub fn slot_pc(slot: *mut FeedbackVectorSlot) -> u32 {
  // Safety: `slot` 为 CallTarget 槽（union 活动变体，用例先核验 kind）。
  unsafe { (*slot).data.call_target.pc }
}

/// 读 CallTarget 槽的 `proto`。
pub fn slot_proto(slot: *mut FeedbackVectorSlot) -> u32 {
  // Safety: 同 [`slot_pc`]。
  unsafe { (*slot).data.call_target.proto }
}

/// 读 CallTarget 槽的 `hits`。
pub fn slot_hits(slot: *mut FeedbackVectorSlot) -> u32 {
  // Safety: 同 [`slot_pc`]。
  unsafe { (*slot).data.call_target.hits }
}

/// `*(*f).code.add(index)`：读第 `index` 条指令字。
pub fn code_at(f: *mut Proto, index: usize) -> Instruction {
  // Safety: `f` 存活且 `index` 落在其 `code` 数组内。
  unsafe { *(*f).code.add(index) }
}

/// `*(*f).code.add(index) = value`：写第 `index` 条指令字。
pub fn set_code_at(f: *mut Proto, index: usize, value: Instruction) {
  // Safety: 同 [`code_at`]，另该写与 cpp 手工预置密封位同形。
  unsafe { *(*f).code.add(index) = value }
}

/// 读 proto 的 `feedbackvecsize`。
pub fn feedbackvecsize(f: *mut Proto) -> u32 {
  // Safety: `f` 存活。
  unsafe { (*f).feedbackvecsize }
}

/// 读 proto 的 `flags`。
pub fn proto_flags(f: *mut Proto) -> u8 {
  // Safety: `f` 存活。
  unsafe { (*f).flags }
}

/// 读 proto 的 `funid`。
pub fn proto_funid(f: *mut Proto) -> u32 {
  // Safety: `f` 存活。
  unsafe { (*f).funid }
}

/// 读 `install_assert_inliner_data` 登记数据的 `called` 标志。
pub fn inliner_data_called(data: *mut AssertInlinerData) -> bool {
  // Safety: `data` 为 `install_assert_inliner_data` 登记、状态机存活期内可读的指针。
  unsafe { (*data).called }
}

/// [`install_assert_inliner_data`] 的 safe 收口：向 `l` 的 `ecbdata` 写入断言数据。
pub fn assert_inliner_install(
  l: *mut LuaState,
  proto: *mut Proto,
  target: *mut Proto,
  pc: u32,
) -> *mut AssertInlinerData {
  // Safety: `l` 为活跃状态机；`proto`/`target` 为存活 GC proto；满足被调例程 `# Safety`。
  unsafe { install_assert_inliner_data(l, proto, target, pc) }
}
