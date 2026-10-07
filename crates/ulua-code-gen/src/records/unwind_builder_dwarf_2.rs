//! DWARF CFI unwind 构建器：`raw_data` 内联缓冲 + 字节偏移游标（无裸指针，写入界内断言兜底）。
//!
//! 发射原语（`put_*` 系列）以 `self.pos`（字节偏移）在 `[u8; 1024]` 缓冲上做小端非对齐
//! 写，界内性由 `reserve` 的 CODEGEN_ASSERT 保证；唯 `finalize` 把表写入调用方给出的
//! 可执行代码块，属 JIT 代码缓冲边界（见其 `# Safety`）。

use alloc::vec::Vec;
use core::{ffi::c_void, mem::size_of, slice::from_raw_parts_mut};

use crate::{
  enums::{kind_a_64::KindA64, options::Arch, size_x_64::SizeX64},
  functions::unwind::reg_index_to_dw,
  macros::{
    codegen_assert::CODEGEN_ASSERT,
    dwarf_reg::{
      DW_CFA_ADVANCE_LOC1, DW_CFA_DEF_CFA, DW_CFA_DEF_CFA_OFFSET, DW_CFA_NOP, DW_CFA_OFFSET,
      DW_CFA_OFFSET_EXTENDED, DW_REG_A64_LR, DW_REG_A64_SP, DW_REG_X64_RA, DW_REG_X64_RBP,
      DW_REG_X64_RSP,
    },
  },
  records::{register_a_64::RegisterA64, register_x_64::RegisterX64, unwind::UnwindFunctionDwarf2},
};

/// CFI 原始缓冲容量（cpp `rawData[kRawDataLimit]`）。
const K_RAW_DATA: usize = 1024;

/// DWARF FDE 内 Initial Location 字段的字节偏移。
const K_FDE_INITIAL_LOCATION_OFFSET: usize = 8;
/// DWARF FDE 内 Address Range 字段的字节偏移。
const K_FDE_ADDRESS_RANGE_OFFSET: usize = 16;
/// 单条 FDE 头固定长度（4 长度 + 4 CIE 指针 + 8 起始位置 + 8 地址范围）。
const K_FDE_HEADER_SIZE: usize = 24;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct UnwindBuilderDwarf2 {
  pub(crate) begin_offset: usize,
  pub(crate) unwind_functions: Vec<UnwindFunctionDwarf2>,
  pub(crate) raw_data: [u8; K_RAW_DATA],
  /// 写游标：`raw_data` 内字节偏移。
  pub(crate) pos: usize,
  /// 当前 FDE 头起点偏移；`None` 对应 cpp 的空哨兵（start_function 前不得 finish）。
  pub(crate) fde_entry_start: Option<usize>,
}

impl UnwindBuilderDwarf2 {
  /// `const int kCodeAlignFactor = 1;` (UnwindBuilderDwarf2.cpp:75)
  pub const K_CODE_ALIGN_FACTOR: i32 = 1;
  /// `const int kDataAlignFactor = 8;` (UnwindBuilderDwarf2.cpp:76)
  pub const K_DATA_ALIGN_FACTOR: i32 = 8;

  pub(crate) const K_RAW_DATA_LIMIT: u32 = K_RAW_DATA as u32;

  // —— 缓冲写游标原语（小端非对齐，界内断言与 cpp 末尾断言同效）——

  #[inline]
  fn reserve(&self, n: usize) {
    CODEGEN_ASSERT!(self.pos + n <= K_RAW_DATA);
  }

  #[inline]
  fn put_u8(&mut self, value: u8) {
    self.reserve(1);
    self.raw_data[self.pos] = value;
    self.pos += 1;
  }

  #[inline]
  fn put_u32(&mut self, value: u32) {
    self.reserve(4);
    self.raw_data[self.pos..self.pos + 4].copy_from_slice(&value.to_le_bytes());
    self.pos += 4;
  }

  #[inline]
  fn put_u64(&mut self, value: u64) {
    self.reserve(8);
    self.raw_data[self.pos..self.pos + 8].copy_from_slice(&value.to_le_bytes());
    self.pos += 8;
  }

  /// 在既有偏移 `at` 处回填 4 字节小端字段（长度等占位字段收尾时写入）。
  #[inline]
  fn put_u32_at(&mut self, at: usize, value: u32) {
    CODEGEN_ASSERT!(at + 4 <= self.pos);
    self.raw_data[at..at + 4].copy_from_slice(&value.to_le_bytes());
  }

  #[inline]
  fn put_uleb128(&mut self, mut value: u64) {
    loop {
      let mut byte = (value & 0x7f) as u8;
      value >>= 7;
      if value != 0 {
        byte |= 0x80;
      }
      self.put_u8(byte);
      if value == 0 {
        return;
      }
    }
  }

  /// cpp `advanceLocation`：DW_CFA_advance_loc1 + 单字节偏移。
  #[inline]
  fn advance_location(&mut self, offset: u32) {
    assert!(offset < 256);
    self.put_u8(DW_CFA_ADVANCE_LOC1);
    self.put_u8(offset as u8);
  }

  /// cpp `defineCfaExpression`：DW_CFA_def_cfa + 两操作数 LEB128。
  #[inline]
  fn define_cfa_expression(&mut self, dw_reg: i32, stack_offset: u32) {
    self.put_u8(DW_CFA_DEF_CFA);
    self.put_uleb128(dw_reg as u64);
    self.put_uleb128(stack_offset as u64);
  }

  /// cpp `defineCfaExpressionOffset`：DW_CFA_def_cfa_offset + LEB128 偏移。
  #[inline]
  fn define_cfa_expression_offset(&mut self, stack_offset: u32) {
    self.put_u8(DW_CFA_DEF_CFA_OFFSET);
    self.put_uleb128(stack_offset as u64);
  }

  /// cpp `defineSavedRegisterLocation`：DW_CFA_offset（紧凑或扩展形态）+ offset/8。
  #[inline]
  fn define_saved_register_location(&mut self, dw_reg: i32, stack_offset: u32) {
    assert!(
      stack_offset.is_multiple_of(Self::K_DATA_ALIGN_FACTOR as u32),
      "stack offsets have to be measured in K_DATA_ALIGN_FACTOR units"
    );

    if dw_reg <= 0x3f {
      self.put_u8(DW_CFA_OFFSET + dw_reg as u8);
    } else {
      self.put_u8(DW_CFA_OFFSET_EXTENDED);
      self.put_uleb128(dw_reg as u64);
    }
    self.put_uleb128((stack_offset / Self::K_DATA_ALIGN_FACTOR as u32) as u64);
  }

  /// cpp `alignPosition`：从 `start` 起把已写长度补齐到指针宽度（sizeof(size_t)），NOP 填充。
  #[inline]
  fn align_position(&mut self, start: usize) {
    let size = self.pos - start;
    let align = size_of::<usize>();
    let pad = size.next_multiple_of(align) - size;
    for _ in 0..pad {
      self.put_u8(DW_CFA_NOP);
    }
  }

  /// # Safety
  /// `target` 指向本块 `block_size` 字节可写代码内存，且可写范围覆盖
  /// `get_unwind_info_size(block_size)`（调用方 create_block_unwind_info 已断言
  /// `block_size >= unwind_size`）；`func_address`/`offset` 仅参与地址算术。
  pub unsafe fn finalize(
    &self,
    target: *mut u8,
    offset: usize,
    func_address: *mut c_void,
    block_size: usize,
  ) -> usize {
    let unwind_size = self.get_unwind_info_size(block_size);
    // Safety: 依本函数契约——target 起 unwind_size 字节可写；切片仅覆盖已定稿的表体。
    let block = unsafe { from_raw_parts_mut(target, unwind_size) };
    block.copy_from_slice(&self.raw_data[..unwind_size]);

    let k_full_block_function: u32 = u32::MAX;
    let base = func_address as usize as u64 + offset as u64;

    for func in &self.unwind_functions {
      let fde = func.fde_entry_start_pos as usize;
      // FDE 头随前缀拷入 block，其起点 + 定长 24 必在已写入的 unwind_size 之内。
      CODEGEN_ASSERT!(fde + K_FDE_HEADER_SIZE <= unwind_size);

      let initial_location = base + func.begin_offset as u64;
      block[fde + K_FDE_INITIAL_LOCATION_OFFSET..fde + K_FDE_INITIAL_LOCATION_OFFSET + 8]
        .copy_from_slice(&initial_location.to_le_bytes());

      let address_range = if func.end_offset == k_full_block_function {
        (block_size as u64) - (offset as u64)
      } else {
        (func.end_offset as u64) - (func.begin_offset as u64)
      };
      block[fde + K_FDE_ADDRESS_RANGE_OFFSET..fde + K_FDE_ADDRESS_RANGE_OFFSET + 8]
        .copy_from_slice(&address_range.to_le_bytes());
    }

    self.unwind_functions.len()
  }

  pub fn finish_function(&mut self, begin_offset: u32, end_offset: u32) {
    if let Some(last_func) = self.unwind_functions.last_mut() {
      last_func.begin_offset = begin_offset;
      last_func.end_offset = end_offset;
    }

    // 不变式（cpp 同源 LUAU_ASSERT+解引用）：finish_function 必配对先前 start_function。
    let fde_entry_start = self
      .fde_entry_start
      .expect("fde_entry_start 有效：finish_function 必配对先前 start_function");

    self.align_position(fde_entry_start);
    // Length 字段不含自身 4 字节
    let length = (self.pos - fde_entry_start - 4) as u32;
    self.put_u32_at(fde_entry_start, length);
  }

  pub fn finish_info(&mut self) {
    // 结束本 section：终止 4 字节零记录，随后复核总尺寸未越界
    self.put_u32(0);

    ulua_common::LUAU_ASSERT!(self.get_unwind_info_size(0) <= Self::K_RAW_DATA_LIMIT as usize);
  }

  /// 薄 getter `get_begin_offset` 已字段化：调用点直读 pub(crate) 字段 `begin_offset`。
  pub fn get_unwind_info_size(&self, _block_size: usize) -> usize {
    self.pos
  }

  pub fn prologue_a_64(&mut self, prologue_size: u32, stack_size: u32, regs: &[RegisterA64]) {
    CODEGEN_ASSERT!(stack_size.is_multiple_of(16));
    CODEGEN_ASSERT!(regs.len() >= 2 && regs[0].index() == 29 && regs[1].index() == 30);
    CODEGEN_ASSERT!((regs.len() as u32) * 8 <= stack_size);

    self.advance_location(4);
    self.define_cfa_expression_offset(stack_size);
    self.advance_location(prologue_size - 4);

    // 偏移按保存顺序递减：stack_size - 8*i
    for (i, reg) in regs.iter().enumerate() {
      CODEGEN_ASSERT!(reg.kind() == KindA64::X);
      self.define_saved_register_location(reg.index() as i32, stack_size - (i as u32 * 8));
    }
  }

  pub fn prologue_x_64(
    &mut self,
    prologue_size: u32,
    stack_size: u32,
    setup_frame: bool,
    gpr: &[RegisterX64],
    simd: &[RegisterX64],
  ) {
    CODEGEN_ASSERT!(stack_size > 0 && stack_size < 4096 && stack_size.is_multiple_of(8));

    let mut stack_offset: u32 = 8; // 返回地址已由 call 压栈
    let mut prologue_offset: u32 = 0;

    if setup_frame {
      // push rbp
      stack_offset += 8;
      prologue_offset += 2;
      self.advance_location(2);
      self.define_cfa_expression_offset(stack_offset);
      self.define_saved_register_location(DW_REG_X64_RBP, stack_offset);

      // mov rbp, rsp
      prologue_offset += 3;
      self.advance_location(3);
    }

    // push reg
    for reg in gpr {
      CODEGEN_ASSERT!(reg.size() == SizeX64::Qword);
      let dw_reg = reg_index_to_dw(reg.index());

      stack_offset += 8;
      prologue_offset += 2;
      self.advance_location(2);
      self.define_cfa_expression_offset(stack_offset);
      self.define_saved_register_location(dw_reg, stack_offset);
    }

    CODEGEN_ASSERT!(simd.is_empty());

    // sub rsp, stackSize
    stack_offset += stack_size;
    let sub_len = if stack_size >= 128 { 7 } else { 4 };
    prologue_offset += sub_len;
    self.advance_location(4);
    self.define_cfa_expression_offset(stack_offset);

    CODEGEN_ASSERT!(stack_offset.is_multiple_of(16));
    CODEGEN_ASSERT!(prologue_offset == prologue_size);
  }

  pub fn set_begin_offset(&mut self, begin_offset: usize) {
    self.begin_offset = begin_offset;
  }

  pub fn start_function(&mut self) {
    // End offset 稍后填充，最后统一调整所有偏移
    self.unwind_functions.push(UnwindFunctionDwarf2 {
      begin_offset: 0,
      end_offset: 0,
      fde_entry_start_pos: self.pos as u32,
    });

    self.fde_entry_start = Some(self.pos);

    self.put_u32(0); // Length (to be filled later)
    self.put_u32(self.pos as u32); // CIE pointer（相对缓冲起点）
    self.put_u64(0); // Initial location (to be filled later)
    self.put_u64(0); // Address range (to be filled later)

    // 可选的 CIE augmentation 段（不存在）；其后是函数 call frame 指令
  }

  pub fn start_info(&mut self, arch: Arch) {
    CODEGEN_ASSERT!(matches!(arch, Arch::A64 | Arch::X64));

    self.begin_offset = 0;
    self.unwind_functions.clear();
    self.pos = 0;
    self.fde_entry_start = None;

    self.put_u32(0); // Length (to be filled later)
    self.put_u32(0); // CIE id. 0 -- .eh_frame
    self.put_u8(1); // Version
    self.put_u8(0); // CIE augmentation String ""

    let ra = if arch == Arch::A64 {
      DW_REG_A64_LR
    } else {
      DW_REG_X64_RA
    };

    self.put_uleb128(Self::K_CODE_ALIGN_FACTOR as u64); // Code align factor
    self.put_uleb128((-Self::K_DATA_ALIGN_FACTOR & 0x7f) as u64); // data align factor（有符号 LEB128）
    self.put_u8(ra as u8); // Return address register

    // 可选的 CIE augmentation 段（不存在）；call frame 指令（所有 FDE 共用）
    if arch == Arch::A64 {
      self.define_cfa_expression(DW_REG_A64_SP, 0); // CFA = sp
    } else {
      self.define_cfa_expression(DW_REG_X64_RSP, 8); // CFA = rsp + 8
      self.define_saved_register_location(DW_REG_X64_RA, 8); // RA 位于 CFA - 8
    }

    self.align_position(0);
    // Length 字段自身不计入长度
    self.put_u32_at(0, (self.pos - 4) as u32);
  }
}

impl Default for UnwindBuilderDwarf2 {
  fn default() -> Self {
    Self {
      begin_offset: 0,
      unwind_functions: Vec::new(),
      raw_data: [0; K_RAW_DATA],
      pos: 0,
      fde_entry_start: None,
    }
  }
}
