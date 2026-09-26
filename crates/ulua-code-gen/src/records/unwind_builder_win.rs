use alloc::vec::Vec;
use core::{
  ffi::c_void,
  mem::size_of,
  ptr,
  ptr::{copy_nonoverlapping, null_mut},
};

use crate::{
  enums::{arch::Arch, size_x_64::SizeX64},
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{
    register_a_64::RegisterA64, register_x_64::RegisterX64, unwind_builder::UnwindBuilder,
    unwind_code_win::UnwindCodeWin, unwind_function_win::UnwindFunctionWin,
    unwind_info_win::UnwindInfoWin,
  },
};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct UnwindBuilderWin {
  pub base: UnwindBuilder,
  pub(crate) begin_offset: usize,
  pub(crate) raw_data: [u8; 1024],
  pub(crate) raw_data_pos: *mut u8,
  pub(crate) unwind_functions: Vec<UnwindFunctionWin>,
  pub(crate) unwind_codes: Vec<UnwindCodeWin>,
  pub(crate) prolog_size: u8,
  pub(crate) frame_reg: RegisterX64,
  pub(crate) frame_reg_offset: u8,
}

impl UnwindBuilderWin {
  pub(crate) const K_RAW_DATA_LIMIT: u32 = 1024;

  pub fn finalize(
    &self,
    target: *mut u8,
    offset: usize,
    _func_address: *mut c_void,
    block_size: usize,
  ) -> usize {
    let mut current_target = target;
    let k_full_block_function: u32 = 0xFFFFFFFF;

    for func in &self.unwind_functions {
      let mut adjusted_func = *func;

      adjusted_func.begin_offset += offset as u32;

      if adjusted_func.end_offset == k_full_block_function {
        adjusted_func.end_offset = block_size as u32;
      } else {
        adjusted_func.end_offset += offset as u32;
      }

      adjusted_func.unwind_info_offset +=
        (size_of::<UnwindFunctionWin>() * self.unwind_functions.len()) as u32;

      // Safety: 源为栈上 adjusted_func 的合法可读对象; 目标 current_target 从 target 起每个函数
      // 条目推进 size_of::<UnwindFunctionWin>() 字节, 全部条目 + 后续 raw_data 的总长恰为
      // get_unwind_info_size, 调用方 create_block_unwind_info 有 CODEGEN_ASSERT!(block_size >=
      // unwind_size) 保证 target 可写范围覆盖; 按 u8 拷贝故无对齐要求, 源目标不相交。
      unsafe {
        ptr::copy_nonoverlapping(
          &adjusted_func as *const UnwindFunctionWin as *const u8,
          current_target,
          size_of::<UnwindFunctionWin>(),
        );
        current_target = current_target.add(size_of::<UnwindFunctionWin>());
      }
    }

    // Safety: raw_data_pos 于 Default 中由 raw_data.as_mut_ptr() 初始化后仅在 [u8;1024] 数组内
    // 单调前进, 与 raw_data.as_ptr() 同属同一分配, 满足 offset_from 前提; 差值非负且 <=1024。
    let raw_data_len = unsafe { self.raw_data_pos.offset_from(self.raw_data.as_ptr()) } as usize;
    // Safety: 目标 current_target 位于 target 之后 functions_len*size_of::<UnwindFunctionWin>()
    // 字节处, 拷贝 raw_data_len 字节后总写入量 = get_unwind_info_size <= block_size(调用方断言),
    // 源为同一 raw_data 数组的前 raw_data_len 字节, 均为有效可读范围; u8 拷贝无对齐要求。
    unsafe {
      ptr::copy_nonoverlapping(self.raw_data.as_ptr(), current_target, raw_data_len);
    }

    self.unwind_functions.len()
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

    // 契约（贯穿下述各 unsafe 块）：raw_data_pos 为 Default 中由 raw_data.as_mut_ptr() 初始化、
    // 在本数组内单调前进的光标; raw_end = as_mut_ptr()+K_RAW_DATA_LIMIT(1024) 与之同属同一
    // [u8;1024] 分配, 故其间的 .add/.sub 与比较均为同一分配内的合法指针算术。每次
    // copy_nonoverlapping/advance 前后都有 CODEGEN_ASSERT 校验界内, 配合 unwind_codes.len()<256
    // 及各 size_of 字节量, 保证信息头与逆序写入的 unwind code 均不越出缓冲; 拷贝源 &info /
    // *code 为紧邻栈/切片元素的合法可读对象。&mut self 独占期无并存别名。
    // Safety: raw_end 与 raw_data_pos 同属 raw_data 同一分配, 指针算术界内。
    let raw_end = unsafe {
      self
        .raw_data
        .as_mut_ptr()
        .add(UnwindBuilderWin::K_RAW_DATA_LIMIT as usize)
    };

    // Safety: 见上; 写入前断言校验头部长不越界, 拷贝源 &info 为合法可读栈对象。
    unsafe {
      CODEGEN_ASSERT!(self.raw_data_pos.add(size_of::<UnwindInfoWin>()) <= raw_end);

      copy_nonoverlapping(
        &info as *const UnwindInfoWin as *const u8,
        self.raw_data_pos,
        size_of::<UnwindInfoWin>(),
      );
      self.raw_data_pos = self.raw_data_pos.add(size_of::<UnwindInfoWin>());
    }

    if !self.unwind_codes.is_empty() {
      // Safety: 见上; 起点 = 游标 + 逆序区总长, 前置断言复核不越出 raw_data 同一分配。
      let mut unwind_code_pos = unsafe {
        let pos = self
          .raw_data_pos
          .add(size_of::<UnwindCodeWin>() * (self.unwind_codes.len() - 1));
        CODEGEN_ASSERT!(pos <= raw_end);
        pos
      };

      for code in &self.unwind_codes {
        // Safety: 见上; 每次落子后回退一格, len<256 与起点断言共同保证全程界内。
        unsafe {
          copy_nonoverlapping(
            code as *const UnwindCodeWin as *const u8,
            unwind_code_pos,
            size_of::<UnwindCodeWin>(),
          );
          unwind_code_pos = unwind_code_pos.sub(size_of::<UnwindCodeWin>());
        }
      }
    }

    // Safety: 见上; 游标跳过 unwind code 区与奇数补齐后, 末尾断言复核不越界。
    unsafe {
      self.raw_data_pos = self
        .raw_data_pos
        .add(size_of::<UnwindCodeWin>() * self.unwind_codes.len());

      if !self.unwind_codes.len().is_multiple_of(2) {
        self.raw_data_pos = self.raw_data_pos.add(size_of::<UnwindCodeWin>());
      }

      CODEGEN_ASSERT!(self.raw_data_pos <= raw_end);
    }
  }

  pub fn finish_info(&mut self) {}

  pub fn get_begin_offset(&self) -> usize {
    self.begin_offset
  }

  pub fn get_unwind_info_size(&self, _block_size: usize) -> usize {
    let raw_data_ptr = self.raw_data.as_ptr();
    let raw_data_pos = self.raw_data_pos as *const u8;
    // Safety: raw_data_pos 于 Default 中由 raw_data.as_mut_ptr() 初始化后仅在 [u8;1024] 数组内前进, 与本行
    // 的 raw_data_ptr 同属同一分配, 满足 offset_from 的"同一分配"前提; 差值非负且 <=1024, 转 usize 合法。
    let raw_data_diff = unsafe { raw_data_pos.offset_from(raw_data_ptr) } as usize;

    size_of::<UnwindFunctionWin>() * self.unwind_functions.len() + raw_data_diff
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
      // Safety: raw_data_pos 于 Default 中初始化为 raw_data.as_mut_ptr() 后仅在 [u8;1024] 数组内单调前进,
      // 与 raw_data.as_ptr() 同属同一分配, 满足 offset_from 的"同一分配"前提; 结果为光标相对缓冲起点的字节偏移。
      unwind_info_offset: unsafe { self.raw_data_pos.offset_from(self.raw_data.as_ptr()) as u32 },
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
    self.raw_data_pos = self.raw_data.as_mut_ptr();
    self.unwind_functions.clear();
    self.unwind_codes.clear();
    self.prolog_size = 0;
    self.frame_reg = RegisterX64::RAX;
    self.frame_reg_offset = 0;
  }
}

impl Default for UnwindBuilderWin {
  fn default() -> Self {
    let mut builder = Self {
      // vtable 槽 cpp 侧为空指针占位，null() 与 zeroed 位等价
      base: UnwindBuilder::default(),
      begin_offset: 0,
      raw_data: [0; 1024],
      raw_data_pos: null_mut(),
      unwind_functions: Vec::new(),
      unwind_codes: Vec::new(),
      prolog_size: 0,
      // cpp `frameReg = X64::noreg`：noreg = {None, 16}，旧实现 zeroed 误为 RIP，已修正
      frame_reg: RegisterX64::NOREG,
      frame_reg_offset: 0,
    };
    builder.raw_data_pos = builder.raw_data.as_mut_ptr();
    builder
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
