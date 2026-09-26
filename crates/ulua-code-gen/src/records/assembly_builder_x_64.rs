use alloc::{string::String, vec::Vec};
use core::{
  ffi::c_void,
  fmt::{Arguments, write},
  mem::{size_of, size_of_val},
  ptr::{copy_nonoverlapping, null_mut, write_bytes},
  slice::from_raw_parts,
};

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::{
    abix_64::ABIX64, alignment_data_x_64::AlignmentDataX64, category_x_64::CategoryX64,
    condition_x_64::ConditionX64, rounding_mode_x_64::RoundingModeX64, size_x_64::SizeX64,
  },
  functions::{
    float_bits::{get_double_bits, get_float_bits},
    get_current_x_64_abi::get_current_x_64_abi,
    write_unaligned::{writef_32, writef_64, writeu_16, writeu_32, writeu_64},
  },
  macros::{
    codegen_assert::CODEGEN_ASSERT,
    x64_encoding::{
      avx_3_1, avx_3_2, avx_3_3, mod_rm, op_plus_cc, op_plus_reg, rex_b, rex_force, rex_r,
      REX_W_BIT, rex_x, sib,
    },
  },
  records::{
    avx_op_encoding::AvxOpEncoding, binary_op_encoding::BinaryOpEncoding, label::Label,
    operand_x_64::OperandX64, register_x_64::RegisterX64,
  },
};

#[derive(Debug, Clone)]
#[repr(C)]
pub struct AssemblyBuilderX64 {
  pub data: Vec<u8>,
  pub code: Vec<u8>,
  pub text: String,
  pub log_text: bool,
  pub abi: ABIX64,
  pub features: u32,
  pub(crate) next_label: u32,
  pub(crate) pending_labels: Vec<Label>,
  pub(crate) label_locations: Vec<u32>,
  pub(crate) const_cache_32: DenseHashMap<u32, i32>,
  pub(crate) const_cache_64: DenseHashMap<u64, i32>,
  pub(crate) finalized: bool,
  pub(crate) data_pos: usize,
  pub(crate) code_pos: *mut u8,
  pub(crate) code_end: *mut u8,
  pub(crate) instruction_count: u32,
}

impl AssemblyBuilderX64 {
  pub fn get_label_offset(&self, label: &Label) -> u32 {
    ulua_common::LUAU_ASSERT!(label.location != !0u32);
    label.location
  }

  pub fn add(&mut self, lhs: OperandX64, rhs: OperandX64) {
    self.place_binary("add", lhs, rhs, BinaryOpEncoding::ADD);
  }

  pub fn align(&mut self, alignment: u32, data: AlignmentDataX64) {
    if alignment & (alignment - 1) != 0 {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    let size = self.get_code_size();
    let pad = ((size + alignment - 1) & !(alignment - 1)) - size;

    match data {
      AlignmentDataX64::Nop => {
        if self.log_text {
          self.log_append(format_args!("; align {}\n", alignment));
        }

        self.nop(pad);
      }
      AlignmentDataX64::Int3 => {
        if self.log_text {
          self.log_append(format_args!("; align {} using int3\n", alignment));
        }

        while (self.code_pos as usize).wrapping_add(pad as usize) > self.code_end as usize {
          self.extend();
        }

        for _ in 0..pad {
          self.place(0xcc);
        }

        self.commit();
      }
      AlignmentDataX64::Ud2 => {
        if self.log_text {
          self.log_append(format_args!("; align {} using ud2\n", alignment));
        }

        while (self.code_pos as usize).wrapping_add(pad as usize) > self.code_end as usize {
          self.extend();
        }

        let mut i: u32 = 0;

        while i + 1 < pad {
          self.place(0x0f);
          self.place(0x0b);
          i += 2;
        }

        if i < pad {
          self.place(0xcc);
        }

        self.commit();
      }
    }
  }

  /// 常量数据段自 `data` 末尾向前分配：剩余空间不足时把容量翻倍，并把已写入的
  /// 内容整体搬到新尾部（旧头部清零），`finalize` 据此按 `data_pos..len` 取段。
  pub fn allocate_data(&mut self, size: usize, align: usize) -> usize {
    CODEGEN_ASSERT!(align > 0 && align <= 16 && (align & (align - 1)) == 0);

    if self.data_pos < size {
      let old_size = self.data.len();
      self.data.resize(old_size * 2, 0);

      // Safety: data 刚 resize 到 2*old_size 且 len=2*old_size，as_ptr 与 as_mut_ptr 指向同一
      // 存活 u8 缓冲；copy_nonoverlapping 源 [0,old_size) 与目的 [old_size,2*old_size) 互不重叠
      // 且均在界内，write_bytes 仅清零源区 [0,old_size)；单线程 &mut self 独占 data，无别名。
      unsafe {
        copy_nonoverlapping(
          self.data.as_ptr(),
          self.data.as_mut_ptr().add(old_size),
          old_size,
        );
        write_bytes(self.data.as_mut_ptr(), 0, old_size);
      }

      self.data_pos += old_size;
    }

    self.data_pos = (self.data_pos - size) & !(align - 1);
    self.data_pos
  }

  pub fn and_(&mut self, lhs: OperandX64, rhs: OperandX64) {
    // C++ `placeBinary("and", lhs, rhs, 0x80, 0x81, 0x83, 0x20, 0x21, 0x22, 0x23, 4)`.
    // 原移植用了只支持 imm 的 `place_binary_reg_mem_and_imm`（无法
    // 编码 reg/reg 或 reg/mem 形式），且在 ModRM opcode-extension
    // 数字必须是 4（AND）处传了 `0x20`——结果得到 ADD (/0) 编码。
    self.place_binary("and", lhs, rhs, BinaryOpEncoding::AND);
  }

  pub fn assembly_builder_x_64_bool_abix_64_i32(
    log_text: bool,
    abi: ABIX64,
    features: u32,
  ) -> Self {
    let mut builder = Self {
      data: Vec::new(),
      code: Vec::new(),
      text: String::new(),
      log_text,
      abi,
      features,
      next_label: 1,
      pending_labels: Vec::new(),
      label_locations: Vec::new(),
      const_cache_32: DenseHashMap::new(!0u32),
      const_cache_64: DenseHashMap::new(!0u64),
      finalized: false,
      data_pos: 0,
      code_pos: null_mut(),
      code_end: null_mut(),
      instruction_count: 0,
    };

    builder.data.resize(4096, 0);
    builder.data_pos = builder.data.len();

    builder.code.resize(4096, 0);
    builder.code_pos = builder.code.as_mut_ptr();
    // Safety: code_pos 取 resize(4096) 后的 code.as_mut_ptr() 存活基址，ptr::add 允许至多
    // one-past-end 偏移，code.len()=4096 恰为 one-past-end，故 code_end 合法且界定写入区。
    builder.code_end = unsafe { builder.code_pos.add(builder.code.len()) };

    builder
  }

  pub fn assembly_builder_x_64_bool_i32(log_text: bool, features: u32) -> Self {
    let abi = get_current_x_64_abi();
    Self::assembly_builder_x_64_bool_abix_64_i32(log_text, abi, features)
  }

  pub fn bsf(&mut self, dst: RegisterX64, src: OperandX64) {
    if self.log_text {
      self.log2("bsf", OperandX64::reg(dst), src);
    }

    if !matches!(dst.size(), SizeX64::Dword | SizeX64::Qword) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    self.place_rex_register_x_64_operand_x_64(dst, src);
    self.place(0x0f);
    self.place(0xbc);
    self.place_reg_and_mod_reg_mem(OperandX64::reg(dst), src, 0);
    self.commit();
  }

  pub fn bsr(&mut self, dst: RegisterX64, src: OperandX64) {
    if self.log_text {
      self.log2("bsr", OperandX64::reg(dst), src);
    }

    if !matches!(dst.size(), SizeX64::Dword | SizeX64::Qword) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    self.place_rex_register_x_64_operand_x_64(dst, src);
    self.place(0x0f);
    self.place(0xbd);
    self.place_reg_and_mod_reg_mem(OperandX64::reg(dst), src, 0);
    self.commit();
  }

  pub fn bswap(&mut self, dst: RegisterX64) {
    if self.log_text {
      self.log1("bswap", OperandX64::reg(dst));
    }

    if !matches!(dst.size(), SizeX64::Dword | SizeX64::Qword) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    self.place_rex_register_x_64(dst);
    self.place(0x0f);
    self.place(op_plus_reg(0xc8, dst.index()));
    self.commit();
  }

  pub fn bytes_data(&mut self, data: &[u8], align: usize) -> OperandX64 {
    let size = data.len();
    let pos = self.allocate_data(size, align);

    self.data[pos..pos + size].copy_from_slice(data);

    OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
      SizeX64::None,
      RegisterX64::NOREG,
      1,
      RegisterX64::RIP,
      (pos as i32) - (self.data.len() as i32),
    )
  }

  pub fn bytes(&mut self, ptr: *const c_void, size: usize, align: usize) -> OperandX64 {
    // Safety: 契约保证 ptr 指向可读且存活 size 字节的宿主数据（与 C++ `bytes(ptr,size)` 同构），
    // 该切片仅立即被 bytes_data 以 copy_from_slice 只读消费；u8 对齐平凡，size=0 时不产生解引用。
    let slice = unsafe { from_raw_parts(ptr as *const u8, size) };
    self.bytes_data(slice, align)
  }

  pub fn call_label(&mut self, label: &mut Label) {
    self.place(0xe8);
    self.place_label(label);

    if self.log_text {
      self.log_label(*label);
    }

    self.commit();
  }

  pub fn call_operand_x_64(&mut self, op: OperandX64) {
    let check = (if op.cat == CategoryX64::Reg {
      op.base.size()
    } else {
      op.mem_size
    }) == SizeX64::Qword;
    if !check {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    if self.log_text {
      self.log1("call", op);
    }

    // 间接绝对 call 始终以 64 位宽模式工作，REX.W 可选
    self.place_rex_no_w(op);

    self.place(0xff);
    self.place_mod_reg_mem(op, 2, 0);
    self.commit();
  }

  pub fn cdq(&mut self) {
    if self.log_text {
      self.log("cdq");
    }
    self.place(0x99);
    self.commit();
  }

  pub fn cmov(&mut self, cond: ConditionX64, lhs: RegisterX64, rhs: OperandX64) {
    let size = if rhs.cat == CategoryX64::Reg {
      rhs.base.size()
    } else {
      rhs.mem_size
    };

    if size == SizeX64::Byte || size != lhs.size() {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    if self.log_text {
      self.log2(cond.cmov_mnemonic(), OperandX64::reg(lhs), rhs);
    }

    self.place_rex_register_x_64_operand_x_64(lhs, rhs);
    self.place(0x0f);
    self.place(0x40 | cond.code());
    self.place_reg_and_mod_reg_mem(OperandX64::reg(lhs), rhs, 0);
    self.commit();
  }

  pub fn cmp(&mut self, lhs: OperandX64, rhs: OperandX64) {
    // C++ `placeBinary("cmp", lhs, rhs, 0x80, 0x81, 0x83, 0x38, 0x39, 0x3a, 0x3b, 7)`.
    // 原移植用了只支持 imm 的 `place_binary_reg_mem_and_imm`，它在
    // reg/reg 或 reg/mem 比较时 DEBUGBREAK（断言 rhs 是 imm）。
    self.place_binary("cmp", lhs, rhs, BinaryOpEncoding::CMP);
  }

  pub fn commit(&mut self) {
    // CODEGEN_ASSERT(codePos <= codeEnd);
    self.instruction_count = self.instruction_count.wrapping_add(1);

    let code_pos = self.code_pos as usize;
    let code_end = self.code_end as usize;

    if (code_end.wrapping_sub(code_pos)) < K_MAX_INSTRUCTION_LENGTH {
      self.extend();
    }
  }

  pub fn extend(&mut self) {
    let count = self.get_code_size();

    let new_size = self.code.len() * 2;
    self.code.resize(new_size, 0);

    let data_ptr = self.code.as_mut_ptr();
    // Safety: code 刚 resize 到 new_size(=2*old_len) 且 len=new_size, data_ptr 指向该存活 u8 缓冲首元素;
    // count=get_code_size()<=old_len<=new_len, 故 data_ptr.add(count) 落在 [ptr, ptr+len] 内、u8 对齐。
    self.code_pos = unsafe { data_ptr.add(count as usize) };
    // Safety: data_ptr.add(self.code.len()) 为该存活 u8 缓冲的合法 one-past-end 哨兵, 仅计算地址不解引用。
    self.code_end = unsafe { data_ptr.add(self.code.len()) };
  }

  pub fn cqo(&mut self) {
    if self.log_text {
      self.log("cqo");
    }

    self.place(0x48); // REX.W
    self.place(0x99);
    self.commit();
  }

  pub fn dec(&mut self, op: OperandX64) {
    self.place_unary_mod_reg_mem("dec", op, 0xfe, 0xff, 1);
  }

  pub fn div(&mut self, op: OperandX64) {
    self.place_unary_mod_reg_mem("div", op, 0xf6, 0xf7, 6);
  }

  pub fn f32x4(&mut self, x: f32, y: f32, z: f32, w: f32) -> OperandX64 {
    let pos = self.allocate_data(16, 16);

    // Safety: allocate_data(16,16) 预留 ≥16 字节使 pos+16 ≤ data.len()，data_pos 为其起点，
    // 四次 writef_32 落于 [pos,pos+16)；&mut self 独占 data，无别名。
    unsafe {
      let data_pos = self.data.as_mut_ptr().add(pos);
      let _ = writef_32(data_pos, x);
      let _ = writef_32(data_pos.add(4), y);
      let _ = writef_32(data_pos.add(8), z);
      let _ = writef_32(data_pos.add(12), w);
    }

    OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
      SizeX64::Xmmword,
      RegisterX64::NOREG,
      1,
      RegisterX64::RIP,
      pos as i32 - self.data.len() as i32,
    )
  }

  pub fn f64x2(&mut self, x: f64, y: f64) -> OperandX64 {
    let pos = self.allocate_data(16, 16);

    // Safety: allocate_data(16,16) 预留 ≥16 字节使 pos+16 ≤ data.len()，两次 writef_64 分写
    // [pos,pos+8) 与 [pos+8,pos+16)；&mut self 独占 data，无别名。
    unsafe {
      writef_64(self.data.as_mut_ptr().add(pos), x);
      writef_64(self.data.as_mut_ptr().add(pos + 8), y);
    }

    OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
      SizeX64::Xmmword,
      RegisterX64::NOREG,
      1,
      RegisterX64::RIP,
      (pos as i32) - (self.data.len() as i32),
    )
  }

  pub fn finalize(&mut self) -> bool {
    let code_size = (self.code_pos as usize).wrapping_sub(self.code.as_ptr() as usize);
    self.code.resize(code_size, 0);

    for fixup in self.pending_labels.iter().copied() {
      let location = self.label_locations[(fixup.id - 1) as usize];
      if !(location != !0u32) {
        ulua_common::LUAU_DEBUGBREAK!();
      }
      let value = location.wrapping_sub(fixup.location.wrapping_add(4));
      let offset = fixup.location as usize;
      self.code[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }

    let data_size = self.data.len() - self.data_pos;

    if data_size > 0 {
      self
        .data
        .copy_within(self.data_pos..self.data_pos + data_size, 0);
    }

    self.data.resize(data_size, 0);

    self.finalized = true;

    true
  }

  pub fn get_code_size(&self) -> u32 {
    let code_pos = self.code_pos as usize;
    let code_data = self.code.as_ptr() as usize;
    let code_size = code_pos.wrapping_sub(code_data);
    u32::try_from(code_size).unwrap_or(u32::MAX)
  }

  pub fn get_instruction_count(&self) -> u32 {
    self.instruction_count
  }

  pub fn get_register_name(&self, reg: RegisterX64) -> &'static str {
    static NAMES: [[&str; 16]; 7] = [
      [
        "rip", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
      ],
      [
        "al", "cl", "dl", "bl", "spl", "bpl", "sil", "dil", "r8b", "r9b", "r10b", "r11b", "r12b",
        "r13b", "r14b", "r15b",
      ],
      [
        "ax", "cx", "dx", "bx", "sp", "bp", "si", "di", "r8w", "r9w", "r10w", "r11w", "r12w",
        "r13w", "r14w", "r15w",
      ],
      [
        "eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi", "r8d", "r9d", "r10d", "r11d",
        "r12d", "r13d", "r14d", "r15d",
      ],
      [
        "rax", "rcx", "rdx", "rbx", "rsp", "rbp", "rsi", "rdi", "r8", "r9", "r10", "r11", "r12",
        "r13", "r14", "r15",
      ],
      [
        "xmm0", "xmm1", "xmm2", "xmm3", "xmm4", "xmm5", "xmm6", "xmm7", "xmm8", "xmm9", "xmm10",
        "xmm11", "xmm12", "xmm13", "xmm14", "xmm15",
      ],
      [
        "ymm0", "ymm1", "ymm2", "ymm3", "ymm4", "ymm5", "ymm6", "ymm7", "ymm8", "ymm9", "ymm10",
        "ymm11", "ymm12", "ymm13", "ymm14", "ymm15",
      ],
    ];

    let size_index = match reg.size() {
      SizeX64::None => 0,
      SizeX64::Byte => 1,
      SizeX64::Word => 2,
      SizeX64::Dword => 3,
      SizeX64::Qword => 4,
      SizeX64::Xmmword => 5,
      SizeX64::Ymmword => 6,
    };
    let index = reg.index() as usize;

    NAMES[size_index][index]
  }

  #[inline]
  pub fn get_size_name(&self, size: SizeX64) -> &'static str {
    size.into()
  }

  pub fn idiv(&mut self, op: OperandX64) {
    self.place_unary_mod_reg_mem("idiv", op, 0xf6, 0xf7, 7);
  }

  pub fn imul_operand_x_64(&mut self, op: OperandX64) {
    self.place_unary_mod_reg_mem("imul", op, 0xf6, 0xf7, 5);
  }

  pub fn imul_operand_x_64_operand_x_64(&mut self, lhs: OperandX64, rhs: OperandX64) {
    if self.log_text {
      self.log2("imul", lhs, rhs);
    }

    self.place_rex_register_x_64_operand_x_64(lhs.base, rhs);
    self.place(0x0f);
    self.place(0xaf);
    self.place_reg_and_mod_reg_mem(lhs, rhs, 0);
    self.commit();
  }

  pub fn imul_operand_x_64_operand_x_64_i32(&mut self, dst: OperandX64, lhs: OperandX64, rhs: i32) {
    if self.log_text {
      self.log3("imul", dst, lhs, OperandX64::operand_x_64_i32(rhs));
    }

    self.place_rex_register_x_64_operand_x_64(dst.base, lhs);

    if (rhs as i8) as i32 == rhs {
      self.place(0x6b);
      self.place_reg_and_mod_reg_mem(dst, lhs, 1);
      self.place_imm_8(rhs);
    } else {
      self.place(0x69);
      self.place_reg_and_mod_reg_mem(dst, lhs, 4);
      self.place_imm_32(rhs);
    }

    self.commit();
  }

  pub fn inc(&mut self, op: OperandX64) {
    // C++ 的 `placeUnaryModRegMem("inc", op, 0xfe, 0xff, 0)`——必须发出
    // 按 size 选择的 opcode (0xfe/0xff) + REX，而不是裸 ModRM 字节。
    self.place_unary_mod_reg_mem("inc", op, 0xfe, 0xff, 0);
  }

  pub fn int3(&mut self) {
    if self.log_text {
      self.log("int3");
    }

    self.place(0xcc);
    self.commit();
  }

  pub fn jcc(&mut self, cond: ConditionX64, label: &mut Label) {
    self.place_jcc(cond.jcc_mnemonic(), label, cond.code());
  }

  pub fn jmp_label(&mut self, label: &mut Label) {
    self.place(0xe9);
    self.place_label(label);

    if self.log_text {
      self.log1_label("jmp", *label);
    }

    self.commit();
  }

  pub fn jmp_operand_x_64(&mut self, op: OperandX64) {
    if !((if op.cat == CategoryX64::Reg {
      op.base.size()
    } else {
      op.mem_size
    }) == SizeX64::Qword)
    {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    if self.log_text {
      self.log1("jmp", op);
    }

    // 间接绝对 call 始终以 64 位宽模式工作，REX.W 可选
    // 本可保留该前缀，但 Windows x64 ABI 下它向 unwinder 表示尾调用
    self.place_rex_no_w(op);

    self.place(0xff);
    self.place_mod_reg_mem(op, 4, 0);
    self.commit();
  }

  pub fn lea_operand_x_64_operand_x_64(&mut self, lhs: OperandX64, mut rhs: OperandX64) {
    if self.log_text {
      self.log2("lea", lhs, rhs);
    }

    CODEGEN_ASSERT!(
      lhs.cat == CategoryX64::Reg && rhs.cat == CategoryX64::Mem && rhs.mem_size == SizeX64::None
    );
    CODEGEN_ASSERT!(rhs.base == RegisterX64::RIP || rhs.base.size() == lhs.base.size());
    CODEGEN_ASSERT!(rhs.index == RegisterX64::NOREG || rhs.index.size() == lhs.base.size());

    rhs.mem_size = lhs.base.size();

    self.place_binary_reg_and_reg_mem(lhs, rhs, 0x8d, 0x8d);
  }

  pub fn lea_register_x_64_label(&mut self, lhs: RegisterX64, label: &mut Label) {
    if lhs.size() != SizeX64::Qword {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    let rhs = OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
      SizeX64::Qword,
      RegisterX64::NOREG,
      1,
      RegisterX64::RIP,
      0,
    );

    self.place_binary_reg_and_reg_mem(OperandX64::operand_x_64_register_x_64(lhs), rhs, 0x8d, 0x8d);

    self.code_pos = self.code_pos.wrapping_sub(4);

    self.place_label(label);
    self.commit();

    if self.log_text {
      self.log_reg_label("lea", lhs, *label);
    }
  }

  pub fn log(&mut self, opcode: &str) {
    self.log_append(format_args!(" {}\n", opcode));
  }

  pub fn log1(&mut self, opcode: &str, op: OperandX64) {
    self.log_append(format_args!(" {:<12}", opcode));
    self.log_operand(op);
    self.text.push('\n');
  }

  pub fn log2(&mut self, opcode: &str, op1: OperandX64, op2: OperandX64) {
    self.log_append(format_args!(" {:<12}", opcode));
    self.log_operand(op1);
    self.text.push(',');
    self.log_operand(op2);
    self.text.push('\n');
  }

  pub fn log3(&mut self, opcode: &str, op1: OperandX64, op2: OperandX64, op3: OperandX64) {
    self.log_append(format_args!(" {:<12}", opcode));
    self.log_operand(op1);
    self.text.push(',');
    self.log_operand(op2);
    self.text.push(',');
    self.log_operand(op3);
    self.text.push('\n');
  }

  pub fn log4(
    &mut self,
    opcode: &str,
    op1: OperandX64,
    op2: OperandX64,
    op3: OperandX64,
    op4: OperandX64,
  ) {
    self.log_append(format_args!(" {:<12}", opcode));
    self.log_operand(op1);
    self.text.push(',');
    self.log_operand(op2);
    self.text.push(',');
    self.log_operand(op3);
    self.text.push(',');
    self.log_operand(op4);
    self.text.push('\n');
  }

  pub fn log1_label(&mut self, opcode: &str, label: Label) {
    self.log_append(format_args!(" {:<12}.L{}\n", opcode, label.id));
  }

  pub fn log_reg_label(&mut self, opcode: &str, reg: RegisterX64, label: Label) {
    // 不能复用 log2：它会在寄存器后追加换行，必须手动拼接 `,.L<id>\n`
    self.log_append(format_args!(" {:<12}", opcode));
    self.log_operand(reg.into());
    self.text.push(',');
    self.log_append(format_args!(".L{}\n", label.id));
  }

  pub fn log_label(&mut self, label: Label) {
    self.log_append(format_args!(".L{}:\n", label.id));
  }

  pub fn log_operand(&mut self, op: OperandX64) {
    match op.cat {
      CategoryX64::Reg => {
        let reg_name = self.get_register_name(op.base);
        self.log_append(format_args!("{}", reg_name));
      }
      CategoryX64::Mem => {
        if op.base == RegisterX64::RIP {
          if op.mem_size != SizeX64::None {
            self.log_append(format_args!("{} ptr ", self.get_size_name(op.mem_size)));
          }
          // 对应 C++ "[.start%+d]"：%+d 必须始终输出符号
          self.log_append(format_args!("[.start{:+}]", op.imm));
          return;
        }

        if op.mem_size != SizeX64::None {
          self.log_append(format_args!("{} ptr ", self.get_size_name(op.mem_size)));
        }

        self.text.push('[');

        if op.base != RegisterX64::NOREG {
          let reg_name = self.get_register_name(op.base);
          self.log_append(format_args!("{}", reg_name));
        }

        if op.index != RegisterX64::NOREG {
          let index_name = self.get_register_name(op.index);
          self.log_append(format_args!(
            "{}{}",
            if op.base != RegisterX64::NOREG {
              "+"
            } else {
              ""
            },
            index_name
          ));
        }

        if op.scale != 1 {
          self.log_append(format_args!("*{}", op.scale));
        }

        if op.imm != 0 {
          if (0..=9).contains(&op.imm) {
            self.log_append(format_args!("+{}", op.imm));
          } else if op.imm > 0 {
            self.log_append(format_args!("+0{:X}h", op.imm));
          } else {
            self.log_append(format_args!("-0{:X}h", -op.imm));
          }
        }

        self.text.push(']');
      }
      CategoryX64::Imm => {
        if (0..=9).contains(&op.imm) {
          self.log_append(format_args!("{}", op.imm));
        } else {
          self.log_append(format_args!("{:X}h", op.imm));
        }
      }
    }
  }

  pub fn log_append(&mut self, args: Arguments<'_>) {
    let _ = write(&mut self.text, args);
  }

  pub fn mov(&mut self, lhs: OperandX64, rhs: OperandX64) {
    if self.log_text {
      self.log2("mov", lhs, rhs);
    }

    if lhs.cat == CategoryX64::Reg && rhs.cat == CategoryX64::Imm {
      let size = lhs.base.size();

      // cpp:182-190 每分支各自 placeRex（一次），无分支前提升调用
      if size == SizeX64::Byte {
        self.place_rex_register_x_64(lhs.base);
        self.place(op_plus_reg(0xb0, lhs.base.index()));
        self.place_imm_8(rhs.imm);
      } else if size == SizeX64::Word {
        self.place(0x66);
        self.place_rex_register_x_64(lhs.base);
        self.place(op_plus_reg(0xb8, lhs.base.index()));
        self.place_imm_16(rhs.imm as i16);
      } else if size == SizeX64::Dword {
        self.place_rex_register_x_64(lhs.base);
        self.place(op_plus_reg(0xb8, lhs.base.index()));
        self.place_imm_32(rhs.imm);
      } else {
        // qword
        self.place_rex_register_x_64(lhs.base);
        self.place(op_plus_reg(0xb8, lhs.base.index()));
        self.place_imm_64(rhs.imm as i64);
      }
    } else if lhs.cat == CategoryX64::Mem && rhs.cat == CategoryX64::Imm {
      let size = lhs.mem_size;

      self.place_rex_operand_x_64(lhs);

      if size == SizeX64::Byte {
        self.place(0xc6);
        self.place_mod_reg_mem(lhs, 0, 1);
        self.place_imm_8(rhs.imm);
      } else if size == SizeX64::Word {
        self.place(0x66);
        self.place(0xc7);
        self.place_mod_reg_mem(lhs, 0, 2);
        self.place_imm_16(rhs.imm as i16);
      } else {
        // dword 或 qword：本例程中都以 imm32 编码
        self.place(0xc7);
        self.place_mod_reg_mem(lhs, 0, 4);
        self.place_imm_32(rhs.imm);
      }
    } else if lhs.cat == CategoryX64::Reg
      && (matches!(rhs.cat, CategoryX64::Reg | CategoryX64::Mem))
    {
      self.place_binary_reg_and_reg_mem(lhs, rhs, 0x8a, 0x8b);
    } else if lhs.cat == CategoryX64::Mem && rhs.cat == CategoryX64::Reg {
      self.place_binary_reg_mem_and_reg(lhs, rhs, 0x88, 0x89);
    } else {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    self.commit();
  }

  pub fn mov64(&mut self, lhs: RegisterX64, imm: i64) {
    if self.log_text {
      self.text.push_str(" mov         ");
      self.log_operand(lhs.into());
      self.log_append(format_args!(",{:X}h\n", imm as u64));
    }

    if lhs.size() != SizeX64::Qword {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    self.place_rex_register_x_64(lhs);
    self.place(op_plus_reg(0xb8, lhs.index()));
    self.place_imm_64(imm);
    self.commit();
  }

  pub fn movsx(&mut self, lhs: RegisterX64, rhs: OperandX64) {
    if self.log_text {
      self.log2("movsx", OperandX64::reg(lhs), rhs);
    }

    let size = if rhs.cat == CategoryX64::Reg {
      rhs.base.size()
    } else {
      rhs.mem_size
    };

    if !matches!(size, SizeX64::Byte | SizeX64::Word) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    self.place_rex_register_x_64_operand_x_64(lhs, rhs);
    self.place(0x0f);
    self.place(if size == SizeX64::Byte { 0xbe } else { 0xbf });
    self.place_reg_and_mod_reg_mem(OperandX64::reg(lhs), rhs, 0);
    self.commit();
  }

  pub fn movzx(&mut self, lhs: RegisterX64, rhs: OperandX64) {
    if self.log_text {
      self.log2("movzx", OperandX64::reg(lhs), rhs);
    }

    let size = if rhs.cat == CategoryX64::Reg {
      rhs.base.size()
    } else {
      rhs.mem_size
    };

    if !matches!(size, SizeX64::Byte | SizeX64::Word) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    self.place_rex_register_x_64_operand_x_64(lhs, rhs);
    self.place(0x0f);
    self.place(if size == SizeX64::Byte { 0xb6 } else { 0xb7 });
    self.place_reg_and_mod_reg_mem(OperandX64::reg(lhs), rhs, 0);
    self.commit();
  }

  pub fn mul(&mut self, op: OperandX64) {
    self.place_unary_mod_reg_mem("mul", op, 0xf6, 0xf7, 4);
  }

  pub fn neg(&mut self, op: OperandX64) {
    self.place_unary_mod_reg_mem("neg", op, 0xf6, 0xf7, 3);
  }

  pub fn nop(&mut self, mut length: u32) {
    while length != 0 {
      let step = if length > 9 { 9 } else { length };
      length -= step;

      match step {
        1 => {
          if self.log_text {
            self.log_append(format_args!(" nop\n"));
          }
          self.place(0x90);
        }
        2 => {
          if self.log_text {
            self.log_append(format_args!(" xchg        ax, ax ; {}-byte nop\n", step));
          }
          self.place(0x66);
          self.place(0x90);
        }
        3 => {
          if self.log_text {
            self.log_append(format_args!(
              " nop         dword ptr[rax] ; {}-byte nop\n",
              step
            ));
          }
          self.place(0x0f);
          self.place(0x1f);
          self.place(0x00);
        }
        4 => {
          if self.log_text {
            self.log_append(format_args!(
              " nop         dword ptr[rax] ; {}-byte nop\n",
              step
            ));
          }
          self.place(0x0f);
          self.place(0x1f);
          self.place(0x40);
          self.place(0x00);
        }
        5 => {
          if self.log_text {
            self.log_append(format_args!(
              " nop         dword ptr[rax+rax] ; {}-byte nop\n",
              step
            ));
          }
          self.place(0x0f);
          self.place(0x1f);
          self.place(0x44);
          self.place(0x00);
          self.place(0x00);
        }
        6 => {
          if self.log_text {
            self.log_append(format_args!(
              " nop         word ptr[rax+rax] ; {}-byte nop\n",
              step
            ));
          }
          self.place(0x66);
          self.place(0x0f);
          self.place(0x1f);
          self.place(0x44);
          self.place(0x00);
          self.place(0x00);
        }
        7 => {
          if self.log_text {
            self.log_append(format_args!(
              " nop         dword ptr[rax] ; {}-byte nop\n",
              step
            ));
          }
          self.place(0x0f);
          self.place(0x1f);
          self.place(0x80);
          self.place(0x00);
          self.place(0x00);
          self.place(0x00);
          self.place(0x00);
        }
        8 => {
          if self.log_text {
            self.log_append(format_args!(
              " nop         dword ptr[rax+rax] ; {}-byte nop\n",
              step
            ));
          }
          self.place(0x0f);
          self.place(0x1f);
          self.place(0x84);
          self.place(0x00);
          self.place(0x00);
          self.place(0x00);
          self.place(0x00);
          self.place(0x00);
        }
        9 => {
          if self.log_text {
            self.log_append(format_args!(
              " nop         word ptr[rax+rax] ; {}-byte nop\n",
              step
            ));
          }
          self.place(0x66);
          self.place(0x0f);
          self.place(0x1f);
          self.place(0x84);
          self.place(0x00);
          self.place(0x00);
          self.place(0x00);
          self.place(0x00);
          self.place(0x00);
        }
        _ => {}
      }

      self.commit();
    }
  }

  pub fn not_(&mut self, op: OperandX64) {
    // C++ 的 `placeUnaryModRegMem("not", op, 0xf6, 0xf7, 2)`——必须发出
    // 按 size 选择的 opcode (0xf6/0xf7) + REX，而不是裸 ModRM 字节。
    self.place_unary_mod_reg_mem("not", op, 0xf6, 0xf7, 2);
  }

  pub fn or_(&mut self, lhs: OperandX64, rhs: OperandX64) {
    self.place_binary("or", lhs, rhs, BinaryOpEncoding::OR);
  }

  pub fn place(&mut self, byte: u8) {
    if self.code_pos >= self.code_end {
      // Safety: assert_call_handler 为 C-ABI 诊断入口, 三个实参均为 `&[u8]`
      // 静态 NUL 结尾字节串常量的首指针, 满足 *const c_char 约定; 此块仅在不变量
      // 被破坏的诊断路径执行, 不触碰 code 缓冲。
      unsafe {
        ulua_common::assert_call_handler(
          K_ASSERT_INVARIANT.as_ptr().cast(),
          K_ASSERT_FILE.as_ptr().cast(),
          1748,
          K_ASSERT_FUNCTION.as_ptr().cast(),
        );
        ulua_common::LUAU_DEBUGBREAK!();
      }
    }
    // Safety: 汇编流程维持不变量 code_pos<code_end——commit() 在剩余空间不足 K_MAX_INSTRUCTION_LENGTH 时
    // 先 extend() 扩容, 故本处写入 *code_pos=u8 落在存活的 self.code(u8) 缓冲界内(u8 对齐平凡), 自增后不超界。
    unsafe {
      *self.code_pos = byte;
      self.code_pos = self.code_pos.add(1);
    }
  }

  pub fn place_avx2(
    &mut self,
    name: &str,
    dst: OperandX64,
    src: OperandX64,
    code: u8,
    set_w: bool,
    mode: u8,
    prefix: u8,
  ) {
    if dst.cat != CategoryX64::Reg {
      ulua_common::LUAU_DEBUGBREAK!();
    }
    if !matches!(src.cat, CategoryX64::Reg | CategoryX64::Mem) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    if self.log_text {
      self.log2(name, dst, src);
    }

    self.place_vex(
      dst,
      OperandX64::reg(RegisterX64::NOREG),
      src,
      set_w,
      mode,
      prefix,
    );
    self.place(code);
    self.place_reg_and_mod_reg_mem(dst, src, 0);

    self.commit();
  }

  pub fn place_avx2_rev(
    &mut self,
    name: &str,
    dst: OperandX64,
    src: OperandX64,
    code: u8,
    coderev: u8,
    set_w: bool,
    mode: u8,
    prefix: u8,
  ) {
    // 规避 CODEGEN_ASSERT!：本 crate 中 assert_call_handler 签名不匹配。
    if !matches!(
      (dst.cat, src.cat),
      (CategoryX64::Mem, CategoryX64::Reg)
        | (CategoryX64::Reg, CategoryX64::Mem)
        | (CategoryX64::Reg, CategoryX64::Reg)
    ) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    if self.log_text {
      self.log2(name, dst, src);
    }

    if dst.cat == CategoryX64::Mem {
      self.place_vex(
        src,
        OperandX64::reg(RegisterX64::NOREG),
        dst,
        set_w,
        mode,
        prefix,
      );
      self.place(coderev);
      self.place_reg_and_mod_reg_mem(src, dst, 0);
    } else {
      self.place_vex(
        dst,
        OperandX64::reg(RegisterX64::NOREG),
        src,
        set_w,
        mode,
        prefix,
      );
      self.place(code);
      self.place_reg_and_mod_reg_mem(dst, src, 0);
    }

    self.commit();
  }

  pub fn place_avx3(
    &mut self,
    name: &str,
    dst: OperandX64,
    src1: OperandX64,
    src2: OperandX64,
    code: u8,
    set_w: bool,
    mode: u8,
    prefix: u8,
  ) {
    if dst.cat != CategoryX64::Reg {
      ulua_common::LUAU_DEBUGBREAK!();
    }
    if src1.cat != CategoryX64::Reg {
      ulua_common::LUAU_DEBUGBREAK!();
    }
    if !matches!(src2.cat, CategoryX64::Reg | CategoryX64::Mem) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    if self.log_text {
      self.log3(name, dst, src1, src2);
    }

    self.place_vex(dst, src1, src2, set_w, mode, prefix);
    self.place(code);
    self.place_reg_and_mod_reg_mem(dst, src2, 0);
    self.commit();
  }

  pub fn place_avx_imm8(
    &mut self,
    name: &str,
    dst: OperandX64,
    src1: OperandX64,
    src2: OperandX64,
    imm8: u8,
    enc: AvxOpEncoding,
  ) {
    // 规避 CODEGEN_ASSERT!：它经 assert_call_handler 转发、期望 *const i8，
    // 而本调用点由 stringify!(...) 产生 &str。
    if dst.cat != CategoryX64::Reg {
      ulua_common::LUAU_DEBUGBREAK!();
    }
    if src1.cat != CategoryX64::Reg {
      ulua_common::LUAU_DEBUGBREAK!();
    }
    if !matches!(src2.cat, CategoryX64::Reg | CategoryX64::Mem) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    if self.log_text {
      // C++ 的 `placeAvx(..., imm8, ...)` 把 `imm8` 记为末尾操作数
      // （隐式 `OperandX64(int32_t)` -> imm 类别）。
      let imm_op = OperandX64::from(imm8 as i32);
      if src1.base == RegisterX64::NOREG {
        self.log3(name, src2, dst, imm_op);
      } else {
        self.log4(name, dst, src1, src2, imm_op);
      }
    }

    self.place_vex(dst, src1, src2, enc.set_w, enc.mode, enc.prefix);
    self.place(enc.code);
    self.place_reg_and_mod_reg_mem(dst, src2, 1);
    self.place_imm_8(imm8 as i32);

    self.commit();
  }

  pub fn place_binary(
    &mut self,
    name: &str,
    lhs: OperandX64,
    rhs: OperandX64,
    enc: BinaryOpEncoding,
  ) {
    if self.log_text {
      self.log2(name, lhs, rhs);
    }

    if (matches!(lhs.cat, CategoryX64::Reg | CategoryX64::Mem)) && rhs.cat == CategoryX64::Imm {
      self.place_binary_reg_mem_and_imm(
        lhs,
        rhs,
        enc.codeimm8,
        enc.codeimm,
        enc.codeimm_imm8,
        enc.opreg,
      );
    } else if lhs.cat == CategoryX64::Reg
      && (matches!(rhs.cat, CategoryX64::Reg | CategoryX64::Mem))
    {
      self.place_binary_reg_and_reg_mem(lhs, rhs, enc.code8, enc.code);
    } else if lhs.cat == CategoryX64::Mem && rhs.cat == CategoryX64::Reg {
      self.place_binary_reg_mem_and_reg(lhs, rhs, enc.code8rev, enc.coderev);
    } else {
      // 规避 CODEGEN_ASSERT!：本 crate 中 assert_call_handler 签名不匹配。
      ulua_common::LUAU_DEBUGBREAK!();
    }
  }

  pub fn place_binary_reg_and_reg_mem(
    &mut self,
    lhs: OperandX64,
    rhs: OperandX64,
    code8: u8,
    code: u8,
  ) {
    if !(lhs.cat == CategoryX64::Reg && matches!(rhs.cat, CategoryX64::Reg | CategoryX64::Mem)) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    let size = if rhs.cat == CategoryX64::Reg {
      rhs.base.size()
    } else {
      rhs.mem_size
    };
    if lhs.base.size() != size {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    if !matches!(
      size,
      SizeX64::Byte | SizeX64::Word | SizeX64::Dword | SizeX64::Qword
    ) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    if size == SizeX64::Word {
      self.place(0x66);
    }

    self.place_rex_register_x_64_operand_x_64(lhs.base, rhs);
    self.place(if size == SizeX64::Byte { code8 } else { code });
    self.place_reg_and_mod_reg_mem(lhs, rhs, 0);

    self.commit();
  }

  pub fn place_binary_reg_mem_and_imm(
    &mut self,
    lhs: OperandX64,
    rhs: OperandX64,
    code8: u8,
    code: u8,
    code_imm8: u8,
    opreg: u8,
  ) {
    if !matches!(lhs.cat, CategoryX64::Reg | CategoryX64::Mem) {
      ulua_common::LUAU_DEBUGBREAK!();
    }
    if rhs.cat != CategoryX64::Imm {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    let size = if lhs.cat == CategoryX64::Reg {
      lhs.base.size()
    } else {
      lhs.mem_size
    };

    if !matches!(size, SizeX64::Byte | SizeX64::Dword | SizeX64::Qword) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    self.place_rex_operand_x_64(lhs);

    if size == SizeX64::Byte {
      self.place(code8);
      self.place_mod_reg_mem(lhs, opreg, 1);
      self.place_imm_8(rhs.imm);
    } else {
      if !matches!(size, SizeX64::Dword | SizeX64::Qword) {
        ulua_common::LUAU_DEBUGBREAK!();
      }

      if (rhs.imm as i8) as i32 == rhs.imm && code != code_imm8 {
        self.place(code_imm8);
        self.place_mod_reg_mem(lhs, opreg, 1);
        self.place_imm_8(rhs.imm);
      } else {
        self.place(code);
        self.place_mod_reg_mem(lhs, opreg, 4);
        self.place_imm_32(rhs.imm);
      }
    }

    self.commit();
  }

  pub fn place_binary_reg_mem_and_reg(
    &mut self,
    lhs: OperandX64,
    rhs: OperandX64,
    code8: u8,
    code: u8,
  ) {
    // 双操作数指令中第一个操作数恒为寄存器，但数据流方向相反
    self.place_binary_reg_and_reg_mem(rhs, lhs, code8, code);
  }

  pub fn place_imm_16(&mut self, imm: i16) {
    let pos = self.code_pos;
    // Safety: CODEGEN_ASSERT!(pos.add(2) < code_end) 复核剩余空间后 writeu_16 于 [pos,pos+2)
    // 写 2 字节并前移 code_pos；pos 与 code_end 为同一 code 分配内指针（写游标只前进），u8 对齐平凡。
    unsafe {
      CODEGEN_ASSERT!(pos.add(size_of::<i16>()) < self.code_end);
      self.code_pos = writeu_16(pos, imm as u16);
    }
  }

  pub fn place_imm_32(&mut self, imm: i32) {
    let pos = self.code_pos;
    if !((pos as usize).wrapping_add(size_of_val(&imm)) < self.code_end as usize) {
      ulua_common::LUAU_DEBUGBREAK!();
    }
    unsafe {
      // Safety: 源为 imm.to_le_bytes() 的 4 字节临时数组，其 as_ptr 在本语句（含
      // copy_nonoverlapping）结束前存活；目的 pos 位于 code 缓冲内、上方按 C++ 语义校验
      // pos+4<code_end 界内，源(栈)/目(代码区)不重叠，u8 对齐平凡。
      copy_nonoverlapping(imm.to_le_bytes().as_ptr(), pos, size_of_val(&imm));
      self.code_pos = pos.add(size_of_val(&imm));
    }
  }

  pub fn place_imm_64(&mut self, imm: i64) {
    let pos = self.code_pos;
    // Safety: CODEGEN_ASSERT!(pos.add(8) < code_end) 复核剩余空间后 writeu_64 于 [pos,pos+8)
    // 写 8 字节并前移 code_pos；pos 与 code_end 为同一 code 分配内指针（写游标只前进故 pos<code_end），
    // u8 对齐平凡。
    unsafe {
      CODEGEN_ASSERT!(pos.add(size_of::<i64>()) < self.code_end);
      self.code_pos = writeu_64(pos, imm as u64);
    }
  }

  pub fn place_imm_8(&mut self, imm: i32) {
    let imm8 = imm as i8;
    self.place(imm8 as u8);
  }

  pub fn place_imm_8_or_32(&mut self, imm: i32) {
    let imm8 = imm as i8 as i32;
    if imm8 == imm {
      self.place_imm_8(imm8);
    } else {
      self.place_imm_32(imm);
    }
  }

  pub fn place_jcc(&mut self, name: &str, label: &mut Label, cc: u8) {
    self.place(0x0f);
    self.place(op_plus_cc(0x80, cc));
    self.place_label(label);

    if self.log_text {
      self.log1_label(name, *label);
    }

    self.commit();
  }

  pub fn place_label(&mut self, label: &mut Label) {
    if label.location == !0u32 {
      if label.id == 0 {
        label.id = self.next_label;
        self.next_label = self.next_label.wrapping_add(1);
        // C++ 的 `labelLocations.push_back(~0u)`——在 `label_locations`
        // 中为新 label 预留槽位。原移植误把假 Label 压进
        // `pending_labels`，导致 `label_locations` 偏短
        // （`set_label` 越界）并破坏 fixup。
        self.label_locations.push(!0u32);
      }

      self.pending_labels.push(Label {
        id: label.id,
        location: self.get_code_size(),
      });
      self.place_imm_32(0);
    } else {
      self.place_imm_32((label.location.wrapping_sub(4 + self.get_code_size())) as i32);
    }
  }

  pub fn place_mod_reg_mem(&mut self, rhs: OperandX64, regop: u8, extra_code_bytes: i32) {
    if rhs.cat == CategoryX64::Reg {
      self.place(mod_rm(0b11, regop, rhs.base.index()));
    } else if rhs.cat == CategoryX64::Mem {
      let index = rhs.index;
      let base = rhs.base;

      let mut mod_ = 0b00;

      if rhs.imm != 0 {
        if (rhs.imm as i8) as i32 == rhs.imm {
          mod_ = 0b01;
        } else {
          mod_ = 0b10;
        }
      } else {
        // r13/bp 基址寻址需要 displacement
        if (base.index() & 0x7) == 0b101 {
          mod_ = 0b01;
        }
      }

      if index != RegisterX64::NOREG && base != RegisterX64::NOREG {
        self.place(mod_rm(mod_, regop, 0b100));
        self.place(sib(rhs.scale, index.index(), base.index()));

        if mod_ != 0b00 {
          self.place_imm_8_or_32(rhs.imm);
        }
      } else if index != RegisterX64::NOREG && rhs.scale != 1 {
        self.place(mod_rm(0b00, regop, 0b100));
        self.place(sib(rhs.scale, index.index(), 0b101));
        self.place_imm_32(rhs.imm);
      } else if (base.index() & 0x7) == 0b100 {
        // r12/sp 基址寻址需要 SIB
        if rhs.scale != 1 {
          ulua_common::LUAU_DEBUGBREAK!();
        }
        if index != RegisterX64::NOREG {
          ulua_common::LUAU_DEBUGBREAK!();
        }

        self.place(mod_rm(mod_, regop, 0b100));
        self.place(sib(rhs.scale, 0b100, base.index()));

        if rhs.imm != 0 {
          self.place_imm_8_or_32(rhs.imm);
        }
      } else if base == RegisterX64::RIP {
        self.place(mod_rm(0b00, regop, 0b101));

        // 提醒：这里用 (getCodeSize() + 4) 计算当前放置指令结束处的偏移。
        // 本指令字节已全部放置，加 +4 是为计入 imm32 displacement。
        // 不过某些指令在 ModRM 之后还带一个 imm8 或 imm32 字节，
        // 那种情形这里也要一并计入。
        self.place_imm_32(-((self.get_code_size() + 4 + extra_code_bytes as u32) as i32) + rhs.imm);
      } else if base != RegisterX64::NOREG {
        self.place(mod_rm(mod_, regop, base.index()));

        if mod_ != 0b00 {
          self.place_imm_8_or_32(rhs.imm);
        }
      } else {
        self.place(mod_rm(0b00, regop, 0b100));
        self.place(sib(1, 0b100, 0b101));
        self.place_imm_32(rhs.imm);
      }
    } else {
      ulua_common::LUAU_DEBUGBREAK!();
    }
  }

  pub fn place_reg_and_mod_reg_mem(
    &mut self,
    lhs: OperandX64,
    rhs: OperandX64,
    extra_code_bytes: i32,
  ) {
    // 用显式布尔检查规避 CODEGEN_ASSERT! 的指针签名不匹配。
    if lhs.cat != CategoryX64::Reg {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    // C++ 传入 `lhs.base.index`（寄存器编号 0-15）。`{size,index}`
    // 位打包使裸 `.bits` 为 `index<<3 | size`，直接读取会把
    // SIZE 喂进 ModRM 的 reg 字段。应使用解码后的 index。
    self.place_mod_reg_mem(rhs, lhs.base.index(), extra_code_bytes);
  }

  pub fn place_rex_register_x_64(&mut self, op: RegisterX64) {
    let code: u8 = REX_W_BIT!(op.size() == SizeX64::Qword) | rex_b(op) | rex_force(op);

    if code != 0 {
      self.place(code | 0x40);
    }
  }

  pub fn place_rex_operand_x_64(&mut self, op: OperandX64) {
    let mut code: u8 = 0;

    if op.cat == CategoryX64::Reg {
      code = REX_W_BIT!(op.base.size() == SizeX64::Qword) | rex_b(op.base) | rex_force(op.base);
    } else if op.cat == CategoryX64::Mem {
      code = REX_W_BIT!(op.mem_size == SizeX64::Qword) | rex_x(op.index) | rex_b(op.base);
    } else {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    if code != 0 {
      self.place(code | 0x40);
    }
  }

  pub fn place_rex_register_x_64_operand_x_64(&mut self, lhs: RegisterX64, rhs: OperandX64) {
    let mut code = REX_W_BIT!(lhs.size() == SizeX64::Qword) | rex_force(lhs);

    if rhs.cat == CategoryX64::Imm {
      code |= rex_b(lhs);
    } else {
      if !matches!(rhs.cat, CategoryX64::Reg | CategoryX64::Mem) {
        ulua_common::LUAU_DEBUGBREAK!();
      }
      code |=
        rex_r(lhs) | rex_x(rhs.index) | rex_b(rhs.base) | rex_force(lhs) | rex_force(rhs.base);
    }

    if code != 0 {
      self.place(code | 0x40);
    }
  }

  pub fn place_rex_no_w(&mut self, op: OperandX64) {
    let mut code: u8 = 0;

    if op.cat == CategoryX64::Reg {
      code = rex_b(op.base);
    } else if op.cat == CategoryX64::Mem {
      code = rex_x(op.index) | rex_b(op.base);
    } else {
      // 规避 CODEGEN_ASSERT!：本 crate 中 assert_call_handler 签名不匹配。
      ulua_common::LUAU_DEBUGBREAK!();
    }

    if code != 0 {
      self.place(code | 0x40);
    }
  }

  pub fn place_shift(&mut self, name: &str, lhs: OperandX64, rhs: OperandX64, opreg: u8) {
    if self.log_text {
      self.log2(name, lhs, rhs);
    }

    let cl = RegisterX64 {
      bits: (1u8 << RegisterX64::INDEX_SHIFT) | SizeX64::Byte as u8,
    };

    if !matches!(lhs.cat, CategoryX64::Reg | CategoryX64::Mem) {
      ulua_common::LUAU_DEBUGBREAK!();
    }
    if !(rhs.cat == CategoryX64::Imm || (rhs.cat == CategoryX64::Reg && rhs.base == cl)) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    let size = lhs.base.size();

    self.place_rex_register_x_64(lhs.base);

    if rhs.cat == CategoryX64::Imm && rhs.imm == 1 {
      self.place(if size == SizeX64::Byte { 0xd0 } else { 0xd1 });
      self.place_mod_reg_mem(lhs, opreg, 0);
    } else if rhs.cat == CategoryX64::Imm {
      if (rhs.imm as i8) as i32 != rhs.imm {
        ulua_common::LUAU_DEBUGBREAK!();
      }

      self.place(if size == SizeX64::Byte { 0xc0 } else { 0xc1 });
      self.place_mod_reg_mem(lhs, opreg, 1);
      self.place_imm_8(rhs.imm);
    } else {
      self.place(if size == SizeX64::Byte { 0xd2 } else { 0xd3 });
      self.place_mod_reg_mem(lhs, opreg, 0);
    }

    self.commit();
  }

  pub fn place_unary_mod_reg_mem(
    &mut self,
    name: &str,
    op: OperandX64,
    code8: u8,
    code: u8,
    opreg: u8,
  ) {
    if self.log_text {
      self.log1(name, op);
    }

    if !matches!(op.cat, CategoryX64::Reg | CategoryX64::Mem) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    let size = if op.cat == CategoryX64::Reg {
      op.base.size()
    } else {
      op.mem_size
    };

    if !matches!(size, SizeX64::Byte | SizeX64::Dword | SizeX64::Qword) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    self.place_rex_operand_x_64(op);
    self.place(if size == SizeX64::Byte { code8 } else { code });
    self.place_mod_reg_mem(op, opreg, 0);

    self.commit();
  }

  pub fn place_vex(
    &mut self,
    dst: OperandX64,
    src1: OperandX64,
    src2: OperandX64,
    set_w: bool,
    mode: u8,
    prefix: u8,
  ) {
    // 保留断言，但用不走宏的显式布尔检查规避
    // CODEGEN_ASSERT! 的指针签名不匹配。
    if dst.cat != CategoryX64::Reg {
      ulua_common::LUAU_DEBUGBREAK!();
    }
    if src1.cat != CategoryX64::Reg {
      ulua_common::LUAU_DEBUGBREAK!();
    }
    if !matches!(src2.cat, CategoryX64::Reg | CategoryX64::Mem) {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    // 多个已翻译的 AVX 调用点传入的是 RAW x86 opcode-map /
    // 强制前缀字节（0x0F/0x38/0x3A 与 0x66/0xF2/0xF3），而非 C++ 常量
    // 使用的窄 VEX 字段编码（AVX_0F=0b00001、AVX_66=0b01……）。
    // 它们会溢出 5 位 mmmmm / 2 位 pp 字段并破坏邻近位。
    // 在此处（唯一的 VEX 收口点）归一化；
    // 已是合法字段形式（0..=3）的值原样通过。
    let mode = match mode {
      0x0F => 0b00001,
      0x38 => 0b00010,
      0x3A => 0b00011,
      m => m,
    };
    let prefix = match prefix {
      0x66 => 0b01,
      0xF3 => 0b10,
      0xF2 => 0b11,
      p => p,
    };

    // C++ 的 `dst.base.size == ymmword`——l（256 位）位来自
    // 目标 REGISTER 的尺寸。reg 操作数的 `memSize` 恒为 `none`，
    // 原来的 `dst.memSize` 检查会把 l 强制为 0（所有 ymm 操作都错）。
    let l: u8 = if dst.base.size() == SizeX64::Ymmword {
      1
    } else {
      0
    };

    self.place(avx_3_1());
    self.place(avx_3_2(dst.base, src2.index, src2.base, mode));
    self.place(avx_3_3(set_w, src1.base, l, prefix));
  }

  pub fn pop(&mut self, op: OperandX64) {
    if self.log_text {
      self.log1("pop", op);
    }

    if op.cat != CategoryX64::Reg || op.base.size() != SizeX64::Qword {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    self.place_rex_register_x_64(op.base);
    self.place(op_plus_reg(0x58, op.base.index()));
    self.commit();
  }

  pub fn push(&mut self, op: OperandX64) {
    if self.log_text {
      self.log1("push", op);
    }

    if op.cat != CategoryX64::Reg || op.base.size() != SizeX64::Qword {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    self.place_rex_register_x_64(op.base);
    self.place(op_plus_reg(0x50, op.base.index()));
    self.commit();
  }

  pub fn ret(&mut self) {
    if self.log_text {
      self.log("ret");
    }

    self.place(0xc3);
    self.commit();
  }

  pub fn rol(&mut self, lhs: OperandX64, rhs: OperandX64) {
    self.place_shift("rol", lhs, rhs, 0);
  }

  pub fn ror(&mut self, lhs: OperandX64, rhs: OperandX64) {
    self.place_shift("ror", lhs, rhs, 1);
  }

  pub fn sal(&mut self, lhs: OperandX64, rhs: OperandX64) {
    self.place_shift("sal", lhs, rhs, 4);
  }

  pub fn sar(&mut self, lhs: OperandX64, rhs: OperandX64) {
    self.place_shift("sar", lhs, rhs, 7);
  }

  pub fn set_label(&mut self, label: &mut Label) {
    if label.id == 0 {
      label.id = self.next_label;
      self.next_label = self.next_label.wrapping_add(1);
      self.label_locations.push(!0u32);
    }

    label.location = self.get_code_size();
    self.label_locations[(label.id - 1) as usize] = label.location;

    if self.log_text {
      self.log_label(*label);
    }
  }

  /// C++ `setLabel(Label&)` 重载的移植名，与 `set_label` 同一实现（共享代码
  /// 的 lower/emit 路径跨 x64/a64 统一按此名调用；a64 侧另有 0 参重载）。
  #[inline]
  pub fn set_label_label(&mut self, label: &mut Label) {
    self.set_label(label);
  }

  pub fn setcc(&mut self, cond: ConditionX64, op: OperandX64) {
    let size = if op.cat == CategoryX64::Reg {
      op.base.size()
    } else {
      op.mem_size
    };

    if size != SizeX64::Byte {
      ulua_common::LUAU_DEBUGBREAK!();
    }

    if self.log_text {
      self.log1(cond.setcc_mnemonic(), op);
    }

    self.place_rex_operand_x_64(op);
    self.place(0x0f);
    self.place(0x90 | cond.code());
    self.place_mod_reg_mem(op, 0, 0);
    self.commit();
  }

  pub fn shl(&mut self, lhs: OperandX64, rhs: OperandX64) {
    self.place_shift("shl", lhs, rhs, 4);
  }

  pub fn shr(&mut self, lhs: OperandX64, rhs: OperandX64) {
    self.place_shift("shr", lhs, rhs, 5);
  }

  pub fn sub(&mut self, lhs: OperandX64, rhs: OperandX64) {
    self.place_binary("sub", lhs, rhs, BinaryOpEncoding::SUB);
  }

  pub fn test(&mut self, lhs: OperandX64, rhs: OperandX64) {
    self.place_binary("test", lhs, rhs, BinaryOpEncoding::TEST);
  }

  pub fn u32x4(&mut self, x: u32, y: u32, z: u32, w: u32) -> OperandX64 {
    let pos = self.allocate_data(16, 16);

    // Safety: allocate_data(16,16) 保证在 pos 处预留 ≥16 字节（data 已按需 resize 使
    // pos+16 ≤ data.len()），四处 writeu_32 各写 4 字节落于 [pos,pos+16) 内；单线程 &mut self
    // 独占 data，无别名。
    unsafe {
      writeu_32(self.data.as_mut_ptr().add(pos), x);
      writeu_32(self.data.as_mut_ptr().add(pos + 4), y);
      writeu_32(self.data.as_mut_ptr().add(pos + 8), z);
      writeu_32(self.data.as_mut_ptr().add(pos + 12), w);
    }

    OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
      SizeX64::Xmmword,
      RegisterX64::NOREG,
      1,
      RegisterX64::RIP,
      (pos as i32) - (self.data.len() as i32),
    )
  }

  pub fn ud_2(&mut self) {
    if self.log_text {
      self.log("ud2");
    }

    self.place(0x0f);
    self.place(0x0b);
  }

  pub fn vblendvpd(
    &mut self,
    dst: RegisterX64,
    src1: RegisterX64,
    src2: OperandX64,
    mask: RegisterX64,
  ) {
    // imm8 的 bits [7:4] 用于挑选 operand 4 的寄存器
    // C++: placeAvx("vblendvpd", dst, src1, src2, mask.index << 4, 0x4b,
    //               false, AVX_0F3A, AVX_66);
    // 需要带 imm8 的重载：imm8 = mask.index << 4，
    // code(opcode) = 0x4b, mode = AVX_0F3A (0x3A -> 0b00011).
    self.place_avx_imm8(
      "vblendvpd",
      OperandX64::reg(dst),
      OperandX64::reg(src1),
      src2,
      mask.index() << 4,
      AvxOpEncoding::VBLENDVPD,
    );
  }

  pub fn vblendvps(
    &mut self,
    dst: RegisterX64,
    src1: RegisterX64,
    src2: OperandX64,
    mask: RegisterX64,
  ) {
    // imm8 的 bits [7:4] 用于挑选 operand 4 的寄存器
    // cpp:1045 传真实 dst（此前误传 NOREG 占位，日志打印时越界）
    self.place_avx_imm8(
      "vblendvps",
      OperandX64::reg(dst),
      src1.into(),
      src2,
      mask.index() << 4,
      AvxOpEncoding::VBLENDVPS,
    );
  }

  pub fn vcmpeqsd(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    // C++: placeAvx("vcmpeqsd", dst, src1, src2, 0x00, 0xc2, false, AVX_0F, AVX_F2);
    // 带 imm8 的重载：imm8=0x00，code(opcode)=0xc2。
    self.place_avx_imm8("vcmpeqsd", dst, src1, src2, 0x00, AvxOpEncoding::VCMP);
  }

  pub fn vcmpltsd(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    // C++: placeAvx("vcmpltsd", dst, src1, src2, 0x01, 0xc2, false, AVX_0F, AVX_F2);
    // 带 imm8 的重载：imm8=0x01，code(opcode)=0xc2。
    self.place_avx_imm8("vcmpltsd", dst, src1, src2, 0x01, AvxOpEncoding::VCMP);
  }

  pub fn vcvtsd2ss(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    if src2.cat == CategoryX64::Reg {
      if src2.base.size() != SizeX64::Xmmword {
        ulua_common::LUAU_DEBUGBREAK!();
      }
    } else {
      if src2.mem_size != SizeX64::Qword {
        ulua_common::LUAU_DEBUGBREAK!();
      }
    }

    let set_w = (if src2.cat == CategoryX64::Reg {
      src2.base.size()
    } else {
      src2.mem_size
    }) == SizeX64::Qword;

    self.place_avx3("vcvtsd2ss", dst, src1, src2, 0x5a, set_w, 0b0001, 0b11);
  }

  pub fn vcvtsi2sd(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    // C++: setW = (src2.cat == reg ? src2.base.size : src2.memSize) == qword.
    // 寄存器操作数的尺寸在 `base.size()` 而非 `memSize`
    // （寄存器的 memSize 是 `none`），故 qword GP 源需要 cat 检查。
    let set_w = (if src2.cat == CategoryX64::Reg {
      src2.base.size()
    } else {
      src2.mem_size
    }) == SizeX64::Qword;
    self.place_avx3("vcvtsi2sd", dst, src1, src2, 0x2a, set_w, 0x0f, 0xf2);
  }

  pub fn vcvtsi2ss(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    let is_qword = (if src2.cat == CategoryX64::Reg {
      src2.base.size()
    } else {
      src2.mem_size
    }) == SizeX64::Qword;

    self.place_avx3("vcvtsi2ss", dst, src1, src2, 0x2a, is_qword, 0x0F, 0xF3);
  }

  pub fn vcvtss2sd(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
    if src2.cat == CategoryX64::Reg {
      CODEGEN_ASSERT!(src2.base.size() == SizeX64::Xmmword);
    } else {
      CODEGEN_ASSERT!(src2.mem_size == SizeX64::Dword);
    }

    self.place_avx3("vcvtss2sd", dst, src1, src2, 0x5a, false, 0b0001, 0b10);
  }

  pub fn vcvttsd2si(&mut self, dst: OperandX64, src: OperandX64) {
    self.place_avx2(
      "vcvttsd2si",
      dst,
      src,
      0x2c,
      dst.base.size() == SizeX64::Qword,
      0x0F, // AVX_0F
      0xF2, // AVX_F2
    );
  }

  pub fn vdpps(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64, mask: u8) {
    self.place_avx_imm8("vdpps", dst, src1, src2, mask, AvxOpEncoding::VDPPS);
  }

  pub fn vmovq(&mut self, dst: OperandX64, src: OperandX64) {
    if dst.base.size() == SizeX64::Xmmword {
      if dst.cat != CategoryX64::Reg {
        ulua_common::LUAU_DEBUGBREAK!();
      }
      if src.base.size() != SizeX64::Qword {
        ulua_common::LUAU_DEBUGBREAK!();
      }
      self.place_avx2("vmovq", dst, src, 0x6e, true, 0b0001, 0b01);
    } else if dst.base.size() == SizeX64::Qword {
      if src.cat != CategoryX64::Reg {
        ulua_common::LUAU_DEBUGBREAK!();
      }
      if src.base.size() != SizeX64::Xmmword {
        ulua_common::LUAU_DEBUGBREAK!();
      }
      self.place_avx2("vmovq", src, dst, 0x7e, true, 0b0001, 0b01);
    } else {
      ulua_common::LUAU_DEBUGBREAK!();
    }
  }

  pub fn vpextrd(&mut self, dst: RegisterX64, src: RegisterX64, offset: u8) {
    // 'placeAvx' 包装器没有适配这种原型（opcode r/m, reg, imm8）的重载
    if self.log_text {
      // C++: log("vpextrd", dst, src, offset)
      self.log3(
        "vpextrd",
        dst.into(),
        src.into(),
        OperandX64::from(offset as i32),
      );
    }

    // C++: placeVex(src, noreg, dst, false, AVX_0F3A, AVX_66);
    // opcode map 是 AVX_0F3A (0x3A -> 0b00011)，不是 0x10。
    self.place_vex(
      src.into(),
      OperandX64::reg(RegisterX64::NOREG),
      dst.into(),
      false,
      0x3A,
      0x66,
    );
    self.place(0x16);
    self.place_reg_and_mod_reg_mem(src.into(), dst.into(), 1);
    self.place_imm_8(offset as i32);

    self.commit();
  }

  pub fn vpinsrd(&mut self, dst: RegisterX64, src1: RegisterX64, src2: OperandX64, offset: u8) {
    // C++: placeAvx("vpinsrd", dst, src1, src2, offset, 0x22, false,
    //               AVX_0F3A, AVX_66);
    // dst 是真正的目标寄存器（不是 noreg），opcode map
    // 为 AVX_0F3A（0x3A -> 0b00011），不是 0xF3。
    self.place_avx_imm8(
      "vpinsrd",
      OperandX64::reg(dst),
      OperandX64::reg(src1),
      src2,
      offset,
      AvxOpEncoding::VPINSRD,
    );
  }

  pub fn vpshufps(&mut self, dst: RegisterX64, src1: RegisterX64, src2: OperandX64, shuffle: u8) {
    self.place_avx_imm8(
      "vpshufps",
      OperandX64::reg(dst),
      OperandX64::reg(src1),
      src2,
      shuffle,
      AvxOpEncoding::VPSHUTFPS,
    );
  }

  pub fn vroundps(&mut self, dst: OperandX64, src: OperandX64, rounding_mode: RoundingModeX64) {
    // 'placeAvx' 包装器没有适配这种原型（opcode r/m, reg, imm8）的重载
    if self.log_text {
      // C++: log("vroundps", dst, src, uint8_t(roundingMode) | kRoundingPrecisionInexact)
      self.log3(
        "vroundps",
        dst,
        src,
        OperandX64::from((rounding_mode as u8 | 0x08) as i32),
      );
    }

    // C++: placeVex(dst, noreg, src, false, AVX_0F3A, AVX_66); place(0x08);
    //      placeRegAndModRegMem(dst, src, 1);
    //      placeImm8(roundingMode | kRoundingPrecisionInexact);
    // mode 为 AVX_0F3A (0x3A -> 0b00011)，imm8 精度标志是
    // kRoundingPrecisionInexact (0b1000 = 0x08)，不是 0x04。
    self.place_vex(
      dst,
      OperandX64::reg(RegisterX64::NOREG),
      src,
      false,
      0x3A,
      0x66,
    );
    self.place(0x08);
    self.place_reg_and_mod_reg_mem(dst, src, 1);
    self.place_imm_8((rounding_mode as u8 | 0x08) as i32);

    self.commit();
  }

  pub fn vroundsd(
    &mut self,
    dst: OperandX64,
    src1: OperandX64,
    src2: OperandX64,
    rounding_mode: RoundingModeX64,
  ) {
    // C++: placeAvx("vroundsd", dst, src1, src2,
    //               uint8_t(roundingMode) | kRoundingPrecisionInexact,
    //               0x0b, false, AVX_0F3A, AVX_66);
    // kRoundingPrecisionInexact 为 0b1000 (0x08)，opcode map 是
    // AVX_0F3A（由 0x3A 归一化而来），不是 0。
    const K_ROUNDING_PRECISION_INEXACT: u8 = 0x08;

    self.place_avx_imm8(
      "vroundsd",
      dst,
      src1,
      src2,
      (rounding_mode as u8) | K_ROUNDING_PRECISION_INEXACT,
      AvxOpEncoding::VROUNDSD,
    );
  }

  pub fn vroundss(
    &mut self,
    dst: OperandX64,
    src1: OperandX64,
    src2: OperandX64,
    rounding_mode: RoundingModeX64,
  ) {
    self.place_avx3(
      "vroundss",
      dst,
      src1,
      src2,
      rounding_mode as u8 | 0x04,
      false,
      0x0a,
      0x66,
    );
  }

  pub fn xor_(&mut self, lhs: OperandX64, rhs: OperandX64) {
    self.place_binary("xor", lhs, rhs, BinaryOpEncoding::XOR);
  }
}

/// 单条 x64 指令的最大字节数，对应 cpp/CodeGen/src/AssemblyBuilderX64.cpp:68
/// `const unsigned kMaxInstructionLength = 16;`
const K_MAX_INSTRUCTION_LENGTH: usize = 16;

// `AssemblyBuilderX64::f32` —— RIP 相对 4 字节浮点常量池（骨架见 `x64_rip_const!`）。
// 差异点：位型经 `get_float_bits` 作 key、`get_or_insert` 赋值后到覆盖、offset 用 i32 直接相减。
crate::x64_rip_const! {
  f32(value: f32):
    key get_float_bits,
    cache const_cache_32,
    size Dword,
    bytes 4,
    write |ptr, value: f32| unsafe { writef_32(ptr, value) },
    offset |pos: usize, len: usize| (pos as i32) - (len as i32),
    store |cache: &mut DenseHashMap<u32, i32>, key: u32, offset: i32| *cache.get_or_insert(key) = offset,
}

// `AssemblyBuilderX64::f64` —— RIP 相对 8 字节浮点常量池（骨架见 `x64_rip_const!`）。
// 差异点：位型经 `get_double_bits` 作 key、`get_or_insert` 赋值后到覆盖、offset 用 i32 直接相减。
crate::x64_rip_const! {
  f64(value: f64):
    key get_double_bits,
    cache const_cache_64,
    size Qword,
    bytes 8,
    write |ptr, value: f64| unsafe { writef_64(ptr, value) },
    offset |pos: usize, len: usize| (pos as i32) - (len as i32),
    store |cache: &mut DenseHashMap<u64, i32>, key: u64, offset: i32| *cache.get_or_insert(key) = offset,
}

// `AssemblyBuilderX64::i32` —— RIP 相对 4 字节整型常量池（骨架见 `x64_rip_const!`）。
// 差异点：`i32 as u32` 直转作 key、`try_insert` 先到先得、offset 用 isize 差转 i32。
crate::x64_rip_const! {
  i32(value: i32):
    key |value: i32| value as u32,
    cache const_cache_32,
    size Dword,
    bytes 4,
    write |ptr, value: i32| unsafe { writeu_32(ptr, value as u32) },
    offset |pos: usize, len: usize| (pos as isize - len as isize) as i32,
    store |cache: &mut DenseHashMap<u32, i32>, key: u32, offset: i32| {
      cache.try_insert(key, offset);
    },
}

// `AssemblyBuilderX64::i64` —— RIP 相对 8 字节整型常量池（骨架见 `x64_rip_const!`）。
// 差异点：`i64 as u64` 直转作 key、`try_insert` 先到先得、offset 用 isize 差转 i32。
crate::x64_rip_const! {
  i64(value: i64):
    key |value: i64| value as u64,
    cache const_cache_64,
    size Qword,
    bytes 8,
    write |ptr, value: i64| unsafe { writeu_64(ptr, value as u64) },
    offset |pos: usize, len: usize| (pos as isize - len as isize) as i32,
    store |cache: &mut DenseHashMap<u64, i32>, key: u64, offset: i32| {
      cache.try_insert(key, offset);
    },
}

/// C-ABI 诊断入参（review.md §10 常量形态：`&[u8]` NUL 结尾字节串，收口点
/// 仅 `.as_ptr().cast()`，不外泄 C 字符串类型）
const K_ASSERT_INVARIANT: &[u8] = b"codePos < codeEnd\0";

const K_ASSERT_FILE: &[u8] = b"CodeGen/src/AssemblyBuilderX64.cpp\0";

const K_ASSERT_FUNCTION: &[u8] = b"void Luau::CodeGen::AssemblyBuilderX64::place(uint8_t)\0";
