//! 架构无关的汇编缓冲内核。
//!
//! x64 与 a64 两个 JIT builder 的 `code`/`data` 缓冲、label 表与日志/relocation
//! 骨架逐字重复，此处收口为单实现：x64 用 [`AssemblyBufferCore<u8>`]（`code` 存
//! 字节），a64 用 [`AssemblyBufferCore<u32>`]（`code` 存指令字）。builder 组合持有
//! 内核并经 `Deref`/`DerefMut` 透传字段访问；真正的 ISA 差异（指令编码、`commit`
//! 触发条件、label fixup 语义、`place` 越界诊断串、按尺寸缩放的 label 偏移）留在
//! 各自 builder，本模块不掺入任何分支硬拼。

use alloc::{string::String, vec::Vec};
use core::fmt::{Arguments, write};

use crate::{macros::codegen_assert::CODEGEN_ASSERT, records::label::Label};

/// code/data 缓冲初始容量，对应 cpp 构造预分配 4096（x64 字节、a64 数据段字节）。
pub const K_INITIAL_CAPACITY: usize = 4096;

/// 数据段分配支持的最大对齐；超过即内部不变量破坏。
pub const K_MAX_DATA_ALIGN: usize = 16;

/// 尚未绑定地址的 label 哨兵（cpp `~0u`）。
pub const LABEL_UNBOUND: u32 = !0u32;

/// 缓冲容量不足时的翻倍因子。
const K_GROWTH_FACTOR: usize = 2;

/// 架构无关的汇编缓冲内核：写入游标 + 常量数据段 + label 表 + 日志。
#[derive(Debug, Clone)]
pub struct AssemblyBufferCore<W: Copy + Default> {
  /// 常量数据段，自尾部向前分配（`finalize` 时搬到头部）。
  pub data: Vec<u8>,
  /// 指令缓冲，x64 为 `u8` 字节、a64 为 `u32` 指令字。
  pub code: Vec<W>,
  /// 可读汇编文本，仅 `log_text` 开启时填充。
  pub text: String,
  /// 是否记录汇编文本。
  pub log_text: bool,
  pub(crate) next_label: u32,
  pub(crate) label_locations: Vec<u32>,
  pub(crate) finalized: bool,
  /// `data` 尾部向前分配的游标。
  pub(crate) data_pos: usize,
  /// 写游标：`code` 内元素下标；界内性由 `put`/`extend` 维持。
  pub(crate) code_pos: usize,
}

impl<W: Copy + Default> AssemblyBufferCore<W> {
  /// 初始预分配 `K_INITIAL_CAPACITY` 字节 data（游标定在尾部）与 `code_len` 个 code 元素。
  pub fn new(log_text: bool, code_len: usize) -> Self {
    let mut core = Self {
      data: Vec::new(),
      code: Vec::new(),
      text: String::new(),
      log_text,
      next_label: 1,
      label_locations: Vec::new(),
      finalized: false,
      data_pos: 0,
      code_pos: 0,
    };
    core.data.resize(K_INITIAL_CAPACITY, 0);
    core.data_pos = core.data.len();
    core.code.resize(code_len, W::default());
    core
  }

  /// 当前 code 游标对应的长度（x64 为字节数、a64 为指令字数）。
  pub fn get_code_size(&self) -> u32 {
    u32::try_from(self.code_pos).unwrap_or(u32::MAX)
  }

  /// 常量数据段自末尾向前分配：剩余空间不足时容量翻倍，把已写入内容整体搬到
  /// 新尾部（旧头部清零），`finalize_data_tail` 据此按 `data_pos..len` 取段。
  pub fn allocate_data(&mut self, size: usize, align: usize) -> usize {
    CODEGEN_ASSERT!(align > 0 && align <= K_MAX_DATA_ALIGN && align.is_power_of_two());

    if self.data_pos < size {
      let old_size = self.data.len();
      self.data.resize(old_size * K_GROWTH_FACTOR, 0);

      // 把已写入内容整体搬到新尾部，旧头部清零（与 finalize_data_tail 配套）。
      self.data.copy_within(0..old_size, old_size);
      self.data[..old_size].fill(0);

      self.data_pos += old_size;
    }

    self.data_pos = (self.data_pos - size) & !(align - 1);
    self.data_pos
  }

  /// code 缓冲容量翻倍；索引游标随扩容天然保持有效（cpp 需重锚 codePos/codeEnd）。
  pub fn extend(&mut self) {
    let new_size = self.code.len() * K_GROWTH_FACTOR;
    self.code.resize(new_size, W::default());
  }

  /// 写入一个 code 元素并前移游标。
  ///
  /// 调用方（builder 的 `place`）须先经 `commit` 维持不变量 `code_pos < code.len()`，
  /// 故此处按下标写入必在界内；越界即内部不变量破坏，由下标 panic 兜底。
  pub fn put(&mut self, elem: W) {
    self.code[self.code_pos] = elem;
    self.code_pos += 1;
  }

  /// 记录一行裸助记符（`" <opcode>\n"`）。
  pub fn log(&mut self, opcode: &str) {
    self.log_append(format_args!(" {}\n", opcode));
  }

  /// 追加格式化文本到 `text`，写失败忽略（汇编日志尽力而为）。
  pub fn log_append(&mut self, args: Arguments<'_>) {
    let _ = write(&mut self.text, args);
  }

  /// 为 label 分配 id 并在 `label_locations` 预留哨兵槽位（`id == 0` 视为未登记）。
  pub fn reserve_label(&mut self, label: &mut Label) {
    if label.id == 0 {
      label.id = self.next_label;
      self.next_label = self.next_label.wrapping_add(1);
      self.label_locations.push(LABEL_UNBOUND);
    }
  }

  /// 把 label 绑定到当前 code 游标，开启日志时输出 `.L<id>:`。
  ///
  /// 对应 cpp `setLabel(Label&)`：x64 `set_label` 与 a64 `set_label_label` 共用此实现，
  /// 日志文本 `.L{}:\n` 两侧逐字一致，故不再各留一份。
  pub fn bind_label(&mut self, label: &mut Label) {
    self.reserve_label(label);
    label.location = self.get_code_size();
    self.label_locations[(label.id - 1) as usize] = label.location;

    if self.log_text {
      self.log_append(format_args!(".L{}:\n", label.id));
    }
  }

  /// `finalize` 前段：把 code 截断到实际写入长度。
  pub fn truncate_code(&mut self) {
    self.code.resize(self.code_pos, W::default());
  }

  /// `finalize` 后段：把自尾部向前分配的 data 段搬到头部并截断，随后标记已定型。
  pub fn finalize_data_tail(&mut self) {
    let data_size = self.data.len() - self.data_pos;

    if data_size > 0 {
      self.data.copy_within(self.data_pos.., 0);
    }

    self.data.resize(data_size, 0);
    self.finalized = true;
  }
}
