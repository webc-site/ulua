use alloc::vec::Vec;
use core::{ffi::c_void, mem::size_of, slice::from_raw_parts_mut};

use crate::{
  enums::{options::Arch, size_x_64::SizeX64},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    register_a_64::RegisterA64,
    register_x_64::RegisterX64,
    unwind::{UnwindCodeWin, UnwindFunctionWin},
    unwind_info_win::UnwindInfoWin,
  },
};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct UnwindBuilderWin {
  pub(crate) begin_offset: usize,
  /// Windows unwind 表暂存缓冲；`pos` 为其内单调前进的字节偏移游标。
  pub(crate) raw_data: [u8; UnwindBuilderWin::K_RAW_DATA],
  pub(crate) pos: usize,
  pub(crate) unwind_functions: Vec<UnwindFunctionWin>,
  pub(crate) unwind_codes: Vec<UnwindCodeWin>,
  pub(crate) prolog_size: u8,
  pub(crate) frame_reg: RegisterX64,
  pub(crate) frame_reg_offset: u8,
}

impl UnwindBuilderWin {
  /// raw_data 缓冲容量，对应 cpp UnwindBuilderWin.h `char rawData[1024]`。
  pub(crate) const K_RAW_DATA: usize = 1024;

  /// 整块级函数的 end_offset 哨兵，对应 cpp `kFullBlockFunction = 0xFFFFFFFF`。
  const K_FULL_BLOCK_FUNCTION: u32 = 0xFFFF_FFFF;

  /// 游标界内不变式：任何一次写入前校验 `pos + n` 不越出 raw_data。
  fn reserve(&self, n: usize) {
    CODEGEN_ASSERT!(self.pos + n <= Self::K_RAW_DATA);
  }

  /// # Safety
  /// `target` 指向本块 `block_size` 字节可写代码内存，且可写范围覆盖
  /// `get_unwind_info_size(block_size)`（调用方 create_block_unwind_info 已断言
  /// `block_size >= get_unwind_info_size`）。
  pub unsafe fn finalize(
    &self,
    target: *mut u8,
    offset: usize,
    _func_address: *mut c_void,
    block_size: usize,
  ) -> usize {
    let unwind_len = self.get_unwind_info_size(block_size);

    // Safety: 调用方 create_block_unwind_info 以 CODEGEN_ASSERT!(block_size >=
    // get_unwind_info_size) 保证 target 指向长度覆盖 unwind_len 的可写代码块；
    // 逐字节写入无对齐要求，源（本对象 raw_data / 栈上字段）与目的不相交。
    let block = unsafe { from_raw_parts_mut(target, unwind_len) };

    let functions_len = self.unwind_functions.len();
    let mut at = 0;

    for func in &self.unwind_functions {
      let mut adjusted = *func;

      adjusted.begin_offset += offset as u32;

      adjusted.end_offset = if adjusted.end_offset == Self::K_FULL_BLOCK_FUNCTION {
        block_size as u32
      } else {
        adjusted.end_offset + offset as u32
      };

      adjusted.unwind_info_offset += (size_of::<UnwindFunctionWin>() * functions_len) as u32;

      // UnwindFunctionWin 为 repr(C) 三个 u32 字段，按字段 LE 落盘与原结构体
      // 内存拷贝在（JIT 仅运行的）小端目标上逐字节等价。
      block[at..at + 4].copy_from_slice(&adjusted.begin_offset.to_le_bytes());
      block[at + 4..at + 8].copy_from_slice(&adjusted.end_offset.to_le_bytes());
      block[at + 8..at + 12].copy_from_slice(&adjusted.unwind_info_offset.to_le_bytes());
      at += size_of::<UnwindFunctionWin>();
    }

    block[at..at + self.pos].copy_from_slice(&self.raw_data[..self.pos]);

    functions_len
  }

  pub fn finish_function(&mut self, begin_offset: u32, end_offset: u32) {
    let last = self
      .unwind_functions
      .last_mut()
      // 不变式（cpp 同源直接 back()）：finish_function 只能配对 start_function 调用，
      // 后者已推入当条 unwind function，栈空即调用序被破坏。
      .expect("unwind_functions 非空：finish_function 必配对先前 start_function 的压栈");
    last.begin_offset = begin_offset;
    last.end_offset = end_offset;

    CODEGEN_ASSERT!(self.unwind_codes.len() < 256);

    let mut info = UnwindInfoWin::default();
    info.set_version(1);
    info.set_flags(0);
    info.prologsize = self.prolog_size;
    info.unwindcodecount = self.unwind_codes.len() as u8;

    CODEGEN_ASSERT!(self.frame_reg.index() < 16);
    info.set_framereg(self.frame_reg.index());

    CODEGEN_ASSERT!(self.frame_reg_offset < 16);
    info.set_frameregoff(self.frame_reg_offset);

    // unwind code 每项 2 字节；奇数项时 cpp 追加一格补齐（补齐字节保留缓冲区
    // 既有内容不写出，OS 不读取该字节，故与原实现字节级一致）。
    let codes_len = size_of::<UnwindCodeWin>() * self.unwind_codes.len();
    let pad = if self.unwind_codes.len().is_multiple_of(2) {
      0
    } else {
      size_of::<UnwindCodeWin>()
    };

    self.reserve(size_of::<UnwindInfoWin>() + codes_len + pad);

    // UnwindInfoWin 为四个 u8 字段的 repr(C) POD，逐字段落盘与结构体拷贝等价。
    let at = self.pos;
    self.raw_data[at] = info.version_flags;
    self.raw_data[at + 1] = info.prologsize;
    self.raw_data[at + 2] = info.unwindcodecount;
    self.raw_data[at + 3] = info.framereg_frameregoff;

    let codes_at = at + size_of::<UnwindInfoWin>();

    // cpp 自尾部向前逆序写入 unwind code，落盘字节位置为 codes_at + 2*(len-1-i)。
    for (i, code) in self.unwind_codes.iter().enumerate() {
      let dst = codes_at + size_of::<UnwindCodeWin>() * (self.unwind_codes.len() - 1 - i);
      self.raw_data[dst] = code.offset;
      self.raw_data[dst + 1] = code.opcode_opinfo;
    }

    self.pos = codes_at + codes_len + pad;
  }

  pub fn finish_info(&mut self) {}

  /// 薄 getter `get_begin_offset` 已字段化：调用点直读 pub(crate) 字段 `begin_offset`。
  /// `get_unwind_info_size` 因 ulua-unit-test 外部消费保留 pub。
  pub fn get_unwind_info_size(&self, _block_size: usize) -> usize {
    size_of::<UnwindFunctionWin>() * self.unwind_functions.len() + self.pos
  }

  pub fn prologue_a_64(&mut self, _prologue_size: u32, _stack_size: u32, _regs: &[RegisterA64]) {
    CODEGEN_ASSERT!(false);
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
    CODEGEN_ASSERT!(prologue_size < 256);

    let mut stack_offset: u32 = 8;
    let mut prologue_offset: u32 = 0;

    if setup_frame {
      stack_offset += 8;
      prologue_offset += 2;
      self.unwind_codes.push(unwind_code(
        prologue_offset as u8,
        UWOP_PUSH_NONVOL,
        RegisterX64::RBP.index(),
      ));

      prologue_offset += 3;
      self.frame_reg = RegisterX64::RBP;
      self.frame_reg_offset = 0;
      self.unwind_codes.push(unwind_code(
        prologue_offset as u8,
        UWOP_SET_FPREG,
        self.frame_reg_offset,
      ));
    }

    for reg in gpr {
      CODEGEN_ASSERT!(reg.size() == SizeX64::Qword);

      stack_offset += 8;
      prologue_offset += 2;
      self.unwind_codes.push(unwind_code(
        prologue_offset as u8,
        UWOP_PUSH_NONVOL,
        reg.index(),
      ));
    }

    CODEGEN_ASSERT!(!setup_frame || simd.is_empty());

    let mut simd_storage_size = simd.len() as u32 * 16;

    if !simd.is_empty() && stack_offset % 16 == 8 {
      simd_storage_size += 8;
    }

    if stack_size <= 128 {
      stack_offset += stack_size;
      prologue_offset += if stack_size == 128 { 7 } else { 4 };
      self.unwind_codes.push(unwind_code(
        prologue_offset as u8,
        UWOP_ALLOC_SMALL,
        ((stack_size - 8) / 8) as u8,
      ));
    } else {
      CODEGEN_ASSERT!(stack_size < 4096);

      stack_offset += stack_size;
      prologue_offset += 7;

      let encoded_offset = (stack_size / 8) as u16;
      let bytes = encoded_offset.to_le_bytes();
      self.unwind_codes.push(UnwindCodeWin {
        offset: bytes[0],
        opcode_opinfo: bytes[1],
      });
      self
        .unwind_codes
        .push(unwind_code(prologue_offset as u8, UWOP_ALLOC_LARGE, 0));
    }

    let mut xmm_store_offset = stack_size - simd_storage_size;

    for reg in simd {
      CODEGEN_ASSERT!(reg.size() == SizeX64::Xmmword);
      CODEGEN_ASSERT!(
        xmm_store_offset.is_multiple_of(16),
        "simd stores have to be performed to aligned locations"
      );

      prologue_offset += if xmm_store_offset >= 128 { 10 } else { 7 };
      self
        .unwind_codes
        .push(unwind_code((xmm_store_offset / 16) as u8, 0, 0));
      self.unwind_codes.push(unwind_code(
        prologue_offset as u8,
        UWOP_SAVE_XMM128,
        reg.index(),
      ));
      xmm_store_offset += 16;
    }

    CODEGEN_ASSERT!(stack_offset.is_multiple_of(16));
    CODEGEN_ASSERT!(prologue_offset == prologue_size);

    self.prolog_size = prologue_size as u8;
  }

  pub fn set_begin_offset(&mut self, begin_offset: usize) {
    self.begin_offset = begin_offset;
  }

  pub fn start_function(&mut self) {
    // End offset 稍后填充，最后统一调整所有偏移
    let func = UnwindFunctionWin {
      begin_offset: 0,
      end_offset: 0,
      unwind_info_offset: self.pos as u32,
    };
    self.unwind_functions.push(func);

    self.unwind_codes.clear();
    self.unwind_codes.reserve(16);

    self.prolog_size = 0;

    // rax 寄存器编号为 0，在 Windows unwind info 中表示未使用 frame register
    self.frame_reg = RegisterX64::RAX;
    self.frame_reg_offset = 0;
  }

  pub fn start_info(&mut self, arch: Arch) {
    CODEGEN_ASSERT!(arch == Arch::X64);

    self.begin_offset = 0;
    self.pos = 0;
    self.unwind_functions.clear();
    self.unwind_codes.clear();
    self.prolog_size = 0;
    self.frame_reg = RegisterX64::RAX;
    self.frame_reg_offset = 0;
  }
}

impl Default for UnwindBuilderWin {
  fn default() -> Self {
    Self {
      begin_offset: 0,
      raw_data: [0; Self::K_RAW_DATA],
      pos: 0,
      unwind_functions: Vec::new(),
      unwind_codes: Vec::new(),
      prolog_size: 0,
      // cpp `frameReg = X64::noreg`：noreg = {None, 16}，旧实现 zeroed 误为 RIP，已修正
      frame_reg: RegisterX64::NOREG,
      frame_reg_offset: 0,
    }
  }
}

const UWOP_PUSH_NONVOL: u8 = 0;

const UWOP_ALLOC_LARGE: u8 = 1;

const UWOP_ALLOC_SMALL: u8 = 2;

const UWOP_SET_FPREG: u8 = 3;

const UWOP_SAVE_XMM128: u8 = 8;

fn unwind_code(offset: u8, opcode: u8, opinfo: u8) -> UnwindCodeWin {
  let mut result = UnwindCodeWin {
    offset,
    opcode_opinfo: 0,
  };
  result.set_opcode(opcode);
  result.set_opinfo(opinfo);
  result
}
