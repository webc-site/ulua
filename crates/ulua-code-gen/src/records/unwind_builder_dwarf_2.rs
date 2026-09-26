use alloc::vec::Vec;
use core::{
  ffi::c_void,
  ptr::{copy_nonoverlapping, null_mut},
};

use crate::{
  enums::{arch::Arch, kind_a_64::KindA64, size_x_64::SizeX64},
  functions::{
    advance_location::advance_location,
    align_position::align_position,
    define_cfa_expression::define_cfa_expression,
    define_cfa_expression_offset::define_cfa_expression_offset,
    define_saved_register_location::define_saved_register_location,
    reg_index_to_dw_reg_x_64::reg_index_to_dw_reg_x_64,
    write_unaligned::{writeu_8, writeu_32, writeu_64},
    writeuleb_128::writeuleb_128,
  },
  macros::{
    codegen_assert::CODEGEN_ASSERT,
    dwarf_reg::{DW_REG_A64_LR, DW_REG_A64_SP, DW_REG_X64_RA, DW_REG_X64_RBP, DW_REG_X64_RSP},
  },
  records::{
    register_a_64::RegisterA64, register_x_64::RegisterX64, unwind_builder::UnwindBuilder,
    unwind_function_dwarf_2::UnwindFunctionDwarf2,
  },
};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct UnwindBuilderDwarf2 {
  pub base: UnwindBuilder,
  pub(crate) begin_offset: usize,
  pub(crate) unwind_functions: Vec<UnwindFunctionDwarf2>,
  pub(crate) raw_data: [u8; 1024],
  pub(crate) pos: *mut u8,
  pub(crate) fde_entry_start: *mut u8,
}

impl UnwindBuilderDwarf2 {
  /// `const int kCodeAlignFactor = 1;` (UnwindBuilderDwarf2.cpp:75)
  pub const K_CODE_ALIGN_FACTOR: i32 = 1;
  /// `const int kDataAlignFactor = 8;` (UnwindBuilderDwarf2.cpp:76)
  pub const K_DATA_ALIGN_FACTOR: i32 = 8;

  pub(crate) const K_RAW_DATA_LIMIT: u32 = 1024;

  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn finalize(
    &self,
    target: *mut u8,
    offset: usize,
    func_address: *mut c_void,
    block_size: usize,
  ) -> usize {
    // Safety: target 由 create_block_unwind_info 传入的 block_size 字节可写代码块, 其调用前有
    // CODEGEN_ASSERT!(block_size >= unwind_size); 拷贝长度 get_unwind_info_size 为 raw_data 内
    // 光标偏移(<=1024 且不超过数组实长), 源与目标均有效、u8 拷贝无对齐要求, 两块内存不相交。
    unsafe {
      copy_nonoverlapping(
        self.raw_data.as_ptr(),
        target,
        self.get_unwind_info_size(block_size),
      );
    }

    let k_full_block_function: u32 = u32::MAX;
    let target_u8 = target;

    // DWARF FDE 偏移相关常量
    const K_FDE_INITIAL_LOCATION_OFFSET: usize = 8;
    const K_FDE_ADDRESS_RANGE_OFFSET: usize = 16;

    for func in &self.unwind_functions {
      // 契约（贯穿下述各 unsafe 块）：fde_entry_start_pos 是 start_function 时 raw_data 内
      // 光标的偏移, 该处随后固定写入 24 字节 FDE(4+4+8+8), 故 pos+24 不超过 finish 后的
      // get_unwind_info_size、即第一段已拷入 target 前缀的字节数; target 为块首可写内存,
      // 因此 fde_entry.add(8)/add(16) 处 8 字节写均在有效范围内。writeu_64 做非对齐字节
      // 拷贝, 无对齐前提。
      // Safety: 偏移定位在同一有效块内, 见上。
      let fde_entry = unsafe { target_u8.add(func.fde_entry_start_pos as usize) };

      // Safety: 见上; 写点 = fde_entry+8, 起有 >=8 字节可写空间。
      unsafe {
        writeu_64(
          fde_entry.add(K_FDE_INITIAL_LOCATION_OFFSET),
          (func_address as usize as u64) + (offset as u64) + (func.begin_offset as u64),
        );
      }

      let address_range = if func.end_offset == k_full_block_function {
        (block_size as u64) - (offset as u64)
      } else {
        (func.end_offset as u64) - (func.begin_offset as u64)
      };

      // Safety: 见上; 写点 = fde_entry+16, 起有 >=8 字节可写空间, 落在 24 字节 FDE 内。
      unsafe {
        write_u_64(fde_entry.add(K_FDE_ADDRESS_RANGE_OFFSET), address_range);
      }
    }

    self.unwind_functions.len()
  }

  pub fn finish_function(&mut self, begin_offset: u32, end_offset: u32) {
    if let Some(last_func) = self.unwind_functions.last_mut() {
      last_func.begin_offset = begin_offset;
      last_func.end_offset = end_offset;
    }

    ulua_common::LUAU_ASSERT!(!self.fde_entry_start.is_null());

    // Safety: fde_entry_start 于 start_function 时由 self.pos 赋值, 与 pos 同为指向同一 raw_data(1024B)
    // 数组分配的光标, 故二者 as usize 相减得到同一分配内的合法距离; 上方 LUAU_ASSERT 已排除 fde_entry_start
    // 为空。align_position/writeu_32 在缓冲内以非对齐字节写回填 FDE 长度字段, 越界受既有 LUAU_ASSERT 约束。
    unsafe {
      self.pos = align_position(self.fde_entry_start, self.pos);
      let length = (self.pos as usize - self.fde_entry_start as usize - 4) as u32;
      writeu_32(self.fde_entry_start, length);
    }
  }

  pub fn finish_info(&mut self) {
    // 结束本 section
    // Safety: self.pos 是位于 1024 字节 raw_data 缓冲内的 *mut u8 光标; writeu_32 以 copy_nonoverlapping
    // 做非对齐 4 字节写并前进。紧随其后的 LUAU_ASSERT(get_unwind_info_size<=K_RAW_DATA_LIMIT) 校验含本次
    // 写入后总尺寸未越界, 故写目标始终在缓冲范围内。&mut self 独占期无并存别名。
    self.pos = unsafe { writeu_32(self.pos, 0) };

    ulua_common::LUAU_ASSERT!(self.get_unwind_info_size(0) <= Self::K_RAW_DATA_LIMIT as usize);
  }

  pub fn get_begin_offset(&self) -> usize {
    self.begin_offset
  }

  pub fn get_unwind_info_size(&self, _block_size: usize) -> usize {
    // Safety: self.pos 初始化为 raw_data.as_mut_ptr() 后仅在缓冲内单调前进, 与 raw_data.as_ptr() 同属
    // 同一 [u8;1024] 数组分配, 满足 offset_from 的"指针须落在同一分配"前提; 结果为光标相对缓冲起点的
    // 字节数(即当前已写入的 unwind 信息尺寸), 非负且 <=1024。
    (unsafe { self.pos.offset_from(self.raw_data.as_ptr()) }) as usize
  }

  pub fn prologue_a_64(&mut self, prologue_size: u32, stack_size: u32, regs: &[RegisterA64]) {
    // 契约（贯穿下述各 unsafe 块）：self.pos 为指向 raw_data([u8;1024]) 内单调前进的光标,
    // advance_location/define_* 以非对齐字节写在其上发射 CFI, 总写入量受末尾
    // K_RAW_DATA_LIMIT 断言约束不越界; &mut self 独占期无并存别名。

    CODEGEN_ASSERT!(stack_size.is_multiple_of(16));
    // Safety: 断言须整体短路求值——仅当 regs.len()>=2 成立才经 get_unchecked 读前两槽,
    // 拆成先绑定后断言会引入越界 UB, 故保留为单一 unsafe 表达式; 下标 0/1 界内性即断言本身。
    unsafe {
      CODEGEN_ASSERT!(
        regs.len() >= 2
          && (*regs.get_unchecked(0)).index() == 29
          && (*regs.get_unchecked(1)).index() == 30
      );
    }
    CODEGEN_ASSERT!((regs.len() as u32) * 8 <= stack_size);

    // Safety: 见上; 三段游标推进量为常数与 CFI 编码定长, 界内由调用序保证。
    unsafe {
      self.pos = advance_location(self.pos, 4);
      self.pos = define_cfa_expression_offset(self.pos, stack_size);
      self.pos = advance_location(self.pos, prologue_size - 4);
    }

    // 偏移按保存顺序递减：stack_size - 8*i
    for (i, reg) in regs.iter().enumerate() {
      CODEGEN_ASSERT!(reg.kind() == KindA64::X);
      // Safety: 见上; 每条 saved-reg CFI 定长写入, reg 取自同一不可变切片仅读元数据。
      unsafe {
        self.pos =
          define_saved_register_location(self.pos, reg.index() as i32, stack_size - (i as u32 * 8));
      }
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
    // 契约（贯穿本函数各 unsafe 块）：self.pos 为指向 raw_data([u8;1024]) 缓冲内、单调前进的
    // *mut u8 光标; advance_location / define_cfa_expression_offset / define_saved_register_location
    // 均以非对齐字节写在 pos 上发射 CFI 并返回新光标, 总写入量受构建流程与末尾断言约束, 不越出
    // 缓冲。gpr 为不可变切片, (*reg) 仅做 size()/index() 只读; simd 经 CODEGEN_ASSERT 保证为空。
    // 全程 &mut self 独占, 无并存别名。安全算术/入参断言/游标目标寄存器换算均移出 unsafe。
    CODEGEN_ASSERT!(stack_size > 0 && stack_size < 4096 && stack_size.is_multiple_of(8));

    let mut stack_offset: u32 = 8; // Return address was pushed by calling the function
    let mut prologue_offset: u32 = 0;

    if setup_frame {
      // push rbp
      stack_offset += 8;
      prologue_offset += 2;
      // Safety: 见上; pos 在 raw_data 缓冲内前进并写 CFI。
      unsafe {
        self.pos = advance_location(self.pos, 2);
        self.pos = define_cfa_expression_offset(self.pos, stack_offset);
        self.pos = define_saved_register_location(self.pos, DW_REG_X64_RBP, stack_offset);
      }

      // mov rbp, rsp
      prologue_offset += 3;
      // Safety: 见上; pos 在 raw_data 缓冲内前进。
      unsafe {
        self.pos = advance_location(self.pos, 3);
      }
    }

    // push reg
    for reg in gpr.iter() {
      CODEGEN_ASSERT!((*reg).size() == SizeX64::Qword);
      let dw_reg = reg_index_to_dw_reg_x_64(reg.index());

      stack_offset += 8;
      prologue_offset += 2;
      // Safety: 见上; pos 前进并为本寄存器写 CFI（dw_reg 换算已在安全域完成）。
      unsafe {
        self.pos = advance_location(self.pos, 2);
        self.pos = define_cfa_expression_offset(self.pos, stack_offset);
        self.pos = define_saved_register_location(self.pos, dw_reg, stack_offset);
      }
    }

    CODEGEN_ASSERT!(simd.is_empty());

    // sub rsp, stackSize
    stack_offset += stack_size;
    prologue_offset += if stack_size >= 128 { 7 } else { 4 };
    // Safety: 见上; pos 前进并写最终 CFA 表达式。
    unsafe {
      self.pos = advance_location(self.pos, 4);
      self.pos = define_cfa_expression_offset(self.pos, stack_offset);
    }

    CODEGEN_ASSERT!(stack_offset.is_multiple_of(16));
    CODEGEN_ASSERT!(prologue_offset == prologue_size);
  }

  pub fn set_begin_offset(&mut self, begin_offset: usize) {
    self.begin_offset = begin_offset;
  }

  pub fn start_function(&mut self) {
    // End offset 稍后填充，最后统一调整所有偏移
    let func = UnwindFunctionDwarf2 {
      begin_offset: 0,
      end_offset: 0,
      fde_entry_start_pos: (self.pos as usize - self.raw_data.as_ptr() as usize) as u32,
    };
    self.unwind_functions.push(func);

    self.fde_entry_start = self.pos; // Will be written at the end
    // Safety: self.pos 是 Default 中由 raw_data.as_mut_ptr() 初始化、随后单调前进的 *mut u8 光标, 始终位于
    // 1024 字节 raw_data 缓冲内(至多尾后一字节)。writeu_32/64 以 copy_nonoverlapping 做非对齐字节写
    // (无 4/8 字节对齐要求)并返回前进后的光标; 本块共写 4+4+8+8 字节, 越界由 finish_info 末尾的
    // LUAU_ASSERT(get_unwind_info_size<=K_RAW_DATA_LIMIT) 兜底。全程在 &mut self 独占期内, 无并存别名。
    unsafe {
      self.pos = writeu_32(self.pos, 0); // Length (to be filled later)
      self.pos = writeu_32(
        self.pos,
        (self.pos as usize - self.raw_data.as_ptr() as usize) as u32,
      ); // CIE pointer
      self.pos = writeu_64(self.pos, 0); // Initial location (to be filled later)
      self.pos = writeu_64(self.pos, 0); // Address range (to be filled later)
    }

    // 可选的 CIE augmentation 段（不存在）

    // 其后是函数 call frame 指令
  }

  pub fn start_info(&mut self, arch: Arch) {
    CODEGEN_ASSERT!(matches!(arch, Arch::A64 | Arch::X64));

    self.begin_offset = 0;
    self.unwind_functions.clear();
    self.pos = self.raw_data.as_mut_ptr();
    self.fde_entry_start = null_mut();

    let cie_length = self.pos;
    // Safety: `self.pos` 指向 `self.raw_data`([u8;1024] 内联缓冲)起点; 本函数仅写入一段 CIE 头,
    // 累计字节远小于 1024, 故每次 `writeu_32` 的 `copy_nonoverlapping`(逐字节、无对齐要求)都不越界。
    unsafe {
      self.pos = writeu_32(self.pos, 0); // Length (to be filled later)
    }

    // Safety: 同上, `self.pos` 已前进 4 字节仍在 1024 缓冲内, CIE id 的 4 字节写入在界内。
    unsafe {
      self.pos = writeu_32(self.pos, 0); // CIE id. 0 -- .eh_frame
    }
    // Safety: 同上, 1 字节版本写入落在 [u8;1024] 缓冲内, 无对齐约束。
    unsafe {
      self.pos = writeu_8(self.pos, 1); // Version
    }

    // Safety: 同上, 1 字节 augmentation 串终止符写入仍在缓冲界内。
    unsafe {
      self.pos = writeu_8(self.pos, 0); // CIE augmentation String ""
    }

    let ra = if arch == Arch::A64 {
      DW_REG_A64_LR
    } else {
      DW_REG_X64_RA
    };

    // Safety: 同上, LEB128 编码写入若干字节仍在 1024 缓冲界内; 值为编译期常量, 编码长度有界。
    unsafe {
      self.pos = writeuleb_128(self.pos, Self::K_CODE_ALIGN_FACTOR as u64); // Code align factor
    }
    // Safety: 同上, 该 data align factor 经 `& 0x7f` 限定为单字节 LEB128, 写入在界内。
    unsafe {
      self.pos = writeuleb_128(self.pos, (-Self::K_DATA_ALIGN_FACTOR & 0x7f) as u64);
      // （以有符号 LEB128 表示的）data align factor
    }
    // Safety: 同上, 1 字节返回地址寄存器编号写入落在缓冲界内。
    unsafe {
      self.pos = writeu_8(self.pos, ra as u8); // Return address register
    }

    // 可选的 CIE augmentation 段（不存在）

    // call frame 指令（所有 FDE 共用）
    if arch == Arch::A64 {
      // Safety: `define_cfa_expression` 逐字节向 `self.pos` 追加 CFA 指令, 目标仍在 [u8;1024] 缓冲界内。
      unsafe {
        self.pos = define_cfa_expression(self.pos, DW_REG_A64_SP, 0); // Define CFA to be the sp
      }
    } else {
      // Safety: 同 A64 分支, X64 的 CFA 指令写入落在 1024 缓冲界内。
      unsafe {
        self.pos = define_cfa_expression(self.pos, DW_REG_X64_RSP, 8); // Define CFA to be the rsp + 8
      }
      // Safety: 紧接上条之后追加保存 RA 的指令, `self.pos` 仍未越过缓冲末尾。
      unsafe {
        self.pos = define_saved_register_location(self.pos, DW_REG_X64_RA, 8);
        // 定义返回地址寄存器（RA）位于 CFA - 8
      }
    }

    // Safety: `align_position` 把 `cie_length`(缓冲起点)对齐前进到 `self.pos`, 二者均落在同一
    // [u8;1024] 缓冲内且 `cie_length <= self.pos`, 返回的对齐指针有效。
    unsafe {
      self.pos = align_position(cie_length, self.pos);
    }
    // Safety: 回填长度字段——`cie_length` 即 `raw_data` 起点, 4 字节写入在界内; 长度以两指针的
    // `usize` 差计算, 二者源自同一分配, 相减有效。
    unsafe {
      writeu_32(
        cie_length,
        (self.pos as usize - cie_length as usize - 4) as u32,
      ); // Length field itself is excluded from length
    }
  }
}

impl Default for UnwindBuilderDwarf2 {
  fn default() -> Self {
    let mut builder = Self {
      // vtable 槽 cpp 侧为空指针占位，null() 与 zeroed 位等价
      base: UnwindBuilder::default(),
      begin_offset: 0,
      unwind_functions: Vec::new(),
      raw_data: [0; 1024],
      pos: null_mut(),
      fde_entry_start: null_mut(),
    };
    builder.pos = builder.raw_data.as_mut_ptr();
    builder
  }
}

#[inline(always)]
/// 写入 u64 并返回推进后的指针。
/// # Safety
/// `target` 必须有效且有 ≥ 8 字节可写空间。
unsafe fn write_u_64(target: *mut u8, value: u64) -> *mut u8 {
  // Safety: 本私有 helper 仅在 finalize 的 FDE 回填处调用, 调用点已论证 target 起有 >=8 字节
  // 可写空间; writeu_64 内部为非对齐字节拷贝, 与函数头契约一致。
  unsafe { writeu_64(target, value) }
}
