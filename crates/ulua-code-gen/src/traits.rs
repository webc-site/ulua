//! 汇编 builder 共享的行为 trait。
//!
//! C++ 中对 `AssemblyBuilder`（X64/A64）的模板代码会调用 `build.logAppend(...)`；
//! Rust 版把该能力类型擦除，收敛到此 trait 之后。

use core::fmt::Arguments;
pub trait LogAppend {
  fn log_append(&mut self, args: Arguments<'_>);
}

use crate::{
  enums::options::Arch,
  records::{label::Label, unwind_builder::UnwindBuilderImpl, vm_exit::EntryLocations},
};

/// gate 入口汇编（`functions::init_header_functions`）骨架所需的平台差异接口。
///
/// a64 与 x64 两份 `init_header_functions` 结构同构（构建临时 builder → 发射入口
/// → finalize → 分配 gate 代码块 → 回填入口地址），原先逐字重复；本 trait 只暴露
/// 两平台真正的分歧点（builder 构造/入口发射实现、指令元素宽度、unwind 架构标记），
/// 骨架收敛为单一泛型实现。
pub trait HeaderEntryBuilder {
  /// 本平台 unwind 架构标记。
  const ARCH: Arch;

  /// 构造 gate 入口用临时汇编 builder（跨平台约定：不输出日志、不启用 CPU 特性）。
  fn new_for_header() -> Self;

  /// 发射 gate 入口机器码并返回入口标签组（各平台唯一实现）。
  fn build_header_entry(&mut self, unwind: &mut UnwindBuilderImpl) -> EntryLocations;

  /// 链接/落定临时汇编（骨架不消费返回值，与原两份实现一致）。
  fn finalize_header(&mut self);

  /// 数据段字节视图（gate 入口必须为空，由骨架断言）。
  fn data_bytes(&self) -> &[u8];

  /// 代码段字节视图（a64 的 `Vec<u32>` 指令流按小端字节序列取视图，与 x64 的
  /// `Vec<u8>` 同构收口到分配接口）。
  fn code_bytes(&self) -> &[u8];

  /// 标签在代码段内的字节偏移。
  fn header_label_offset(&self, label: &Label) -> u32;
}

pub mod tag_access;
