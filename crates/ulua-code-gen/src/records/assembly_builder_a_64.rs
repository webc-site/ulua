use alloc::{string::String, vec::Vec};
use core::{
  ffi::c_void,
  fmt::{Arguments, write},
  mem::take,
  ptr::{copy_nonoverlapping, null_mut, write_bytes},
  slice::from_raw_parts,
};

use ulua_common::macros::luau_assert::LUAU_UNREACHABLE;

use crate::{
  enums::{
    address_kind_a_64::AddressKindA64,
    condition_a_64::{ConditionA64, branch_mnemonic_from_code},
    features_a_64::FeaturesA64,
    kind::Kind,
    kind_a_64::{K_SF64, KindA64},
  },
  functions::{
    countlz_bit_utils::countlz_u32,
    countrz_bit_utils::countrz_u32,
    get_fmov_imm::{get_fmov_imm_fp_32, get_fmov_imm_fp_64},
    write_unaligned::{writef_32, writef_64, writeu_64},
  },
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{address_a_64::AddressA64, label::Label, patch::Patch, register_a_64::RegisterA64},
};

/// A64 `add/ldr/str` 等指令的无符号立即数上界（12 位）。
/// 对应 cpp/CodeGen/include/Luau/AssemblyBuilderA64.h `static constexpr size_t kMaxImmediate = (1 << 12) - 1;`
pub(crate) const K_MAX_IMMEDIATE: usize = (1 << 12) - 1;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct AssemblyBuilderA64 {
  // `data`/`code`/`text` 是 C++ AssemblyBuilderA64 的公开可观察输出
  // （测试读 `build.code`）；X64 builder 已把这些
  // 字段标为 `pub`。
  pub data: Vec<u8>,
  pub code: Vec<u32>,
  pub text: String,
  pub(crate) log_text: bool,
  pub(crate) features: u32,
  pub(crate) next_label: u32,
  pub(crate) pending_labels: Vec<Patch>,
  pub(crate) label_locations: Vec<u32>,
  pub(crate) finalized: bool,
  pub(crate) overflowed: bool,
  pub(crate) data_pos: usize,
  pub(crate) code_pos: *mut u32,
  pub(crate) code_end: *mut u32,
}

impl AssemblyBuilderA64 {
  pub fn add_register_a_64_register_a_64_register_a_64_i32(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
    shift: i32,
  ) {
    if src1.kind() == KindA64::X && src2.kind() == KindA64::W {
      self.place_e_r("add", dst, src1, src2, 0b00_01011, shift);
    } else {
      self.place_sr_3("add", dst, src1, src2, 0b00_01011, shift, 0);
    }
  }

  pub fn add_register_a_64_register_a_64_u16(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: u16,
  ) {
    self.place_i12("add", dst, src1, src2 as i32, 0b00_10001);
  }

  pub fn adr_data_with_align(&mut self, dst: RegisterA64, data: &[u8], align: usize) {
    let size = data.len();
    let pos = self.allocate_data(size, align);
    let location = self.get_code_size();

    self.data[pos..pos + size].copy_from_slice(data);

    // cpp AssemblyBuilderA64.cpp:709-723 adr：FarRefs/ProtectData 下走 patchDataRef
    self.patch_data_ref(dst, location, pos);
  }

  pub fn adr_data(&mut self, dst: RegisterA64, data: &[u8]) {
    self.adr_data_with_align(dst, data, 4);
  }

  pub fn adr_register_a_64_void_usize(
    &mut self,
    dst: RegisterA64,
    ptr: *const c_void,
    size: usize,
  ) {
    // Safety: 调用方保证 `ptr` 指向一段存活、至少 `size` 字节的非空内存,`*const u8` 对齐为 1 恒满足;所得切片仅在本语句内被逐字节拷贝消费。
    let slice = unsafe { from_raw_parts(ptr as *const u8, size) };
    self.adr_data(dst, slice);
  }

  pub fn adr_register_a_64_void_usize_align(
    &mut self,
    dst: RegisterA64,
    ptr: *const c_void,
    size: usize,
    align: usize,
  ) {
    // Safety: 调用方保证 `ptr` 指向一段存活、至少 `size` 字节的非空内存,`*const u8` 对齐为 1 恒满足;所得切片仅在本语句内被逐字节拷贝消费。
    let slice = unsafe { from_raw_parts(ptr as *const u8, size) };
    self.adr_data_with_align(dst, slice, align);
  }

  pub fn adr_register_a_64_u64(&mut self, dst: RegisterA64, value: u64) {
    let pos = self.allocate_data(8, 8);
    let location = self.get_code_size();

    // Safety: `allocate_data(8, 8)` 已在 `self.data` 中保留可写的 `[pos, pos+8)` 区间(必要时已扩容),
    // 故 `data.as_mut_ptr().add(pos)` 落在缓冲界内;`writeu_64` 以逐字节拷贝写入 8 字节,无对齐约束。
    unsafe {
      let p = self.data.as_mut_ptr().add(pos);
      writeu_64(p, value);
    }

    // cpp AssemblyBuilderA64.cpp:709-723 adr：FarRefs/ProtectData 下走 patchDataRef
    self.patch_data_ref(dst, location, pos);
  }

  pub fn adr_register_a_64_f32(&mut self, dst: RegisterA64, value: f32) {
    let pos = self.allocate_data(4, 4);
    let location = self.get_code_size();

    // Safety: `allocate_data(4, 4)` 保留可写的 `[pos, pos+4)`;`writef_32` 逐字节写 4 字节,无对齐约束,指针在界内。
    unsafe {
      let p = self.data.as_mut_ptr().add(pos);
      writef_32(p, value);
    }

    // cpp AssemblyBuilderA64.cpp:709-723 adr：FarRefs/ProtectData 下走 patchDataRef
    self.patch_data_ref(dst, location, pos);
  }

  pub fn adr_register_a_64_f64(&mut self, dst: RegisterA64, value: f64) {
    let pos = self.allocate_data(8, 8);
    let location = self.get_code_size();

    // Safety: `allocate_data(8, 8)` 保留可写的 `[pos, pos+8)`;`writef_64` 逐字节写 8 字节,无对齐约束,指针在界内。
    unsafe {
      let p = self.data.as_mut_ptr().add(pos);
      writef_64(p, value);
    }

    // cpp AssemblyBuilderA64.cpp:709-723 adr：FarRefs/ProtectData 下走 patchDataRef
    self.patch_data_ref(dst, location, pos);
  }

  pub fn adr_register_a_64_label(&mut self, dst: RegisterA64, label: &mut Label) {
    self.place_adr_c_char_register_a_64_u8_label("adr", dst, 0b10000, label);
  }

  pub fn allocate_data(&mut self, size: usize, align: usize) -> usize {
    CODEGEN_ASSERT!(align > 0 && align <= 16 && (align & (align - 1)) == 0);

    if self.data_pos < size {
      let old_size = self.data.len();
      self.data.resize(self.data.len() * 2, 0);

      unsafe {
        // Safety: `resize` 后 `self.data` 长度 ≥ 2*old_size，`as_ptr()` 与 `as_mut_ptr()` 指向同一存活
        // 分配。源区 [0,old_size) 与目的区 [old_size,2*old_size) 相邻不重叠，`copy_nonoverlapping` 计数
        // old_size 界内；元素为 `u8`（对齐 1）故对齐天然满足。`write_bytes` 清零 [0,old_size) 亦在同一界内。
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

  pub fn and_register_a_64_register_a_64_register_a_64_i32(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
    shift: i32,
  ) {
    self.place_sr_3("and", dst, src1, src2, 0b00_01010, shift, 0);
  }

  pub fn and_register_a_64_register_a_64_u32(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: u32,
  ) {
    self.place_bm("and", dst, src1, src2, 0b00_100100);
  }

  pub fn asr_register_a_64_register_a_64_register_a_64(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
  ) {
    self.place_r_3("asr", dst, src1, src2, 0b11010110, 0b00_1010);
  }

  pub fn asr_register_a_64_register_a_64_u8(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: u8,
  ) {
    let size = dst.kind().gpr_bits();

    debug_assert!((src2 as i32) < size);

    self.place_bfm(
      "asr",
      dst,
      src1,
      src2 as i32,
      0b00_100110,
      src2 as i32,
      size - 1,
    );
  }

  pub fn assembly_builder_a_64_assembly_builder_a_64(&mut self, log_text: bool, features: u32) {
    self.log_text = log_text;
    self.features = features;

    self.data.resize(4096, 0);
    self.data_pos = self.data.len(); // data is filled backwards

    self.code.resize(1024, 0);
    self.code_pos = self.code.as_mut_ptr();
    // Safety: code_pos 取 resize(1024) 后的 code.as_mut_ptr()（*mut u32）存活基址，ptr::add 允许
    // 至多 one-past-end，add(code.len()) 恰落在 one-past-end，故 code_end 合法且界定写入区。
    self.code_end = unsafe { self.code_pos.add(self.code.len()) };
  }

  /// C++ 的 `AssemblyBuilderA64(bool logText, unsigned int features = 0)`——真正的
  /// 构造函数。上方裸的 `assembly_builder_a_64_assembly_builder_a_64`
  /// 只是 `&mut self` 初始化体（ctor 被译成返回 `()` 的方法）；本函数
  /// 构建、初始化并返回（RETURN）该值，使调用方/测试可写
  /// `let build = AssemblyBuilderA64::...(false, 0);`。
  pub fn assembly_builder_a_64_bool_i32(log_text: bool, features: u32) -> Self {
    let mut build = AssemblyBuilderA64 {
      data: Vec::new(),
      code: Vec::new(),
      text: String::new(),
      log_text: false,
      features: 0,
      next_label: 1,
      pending_labels: Vec::new(),
      label_locations: Vec::new(),
      finalized: false,
      overflowed: false,
      data_pos: 0,
      code_pos: null_mut(),
      code_end: null_mut(),
    };
    build.assembly_builder_a_64_assembly_builder_a_64(log_text, features);
    build
  }

  pub fn b_label(&mut self, label: &mut Label) {
    self.place_b("b", label, 0b0_00101);
  }

  pub fn b_condition_a_64_label(&mut self, cond: ConditionA64, label: &mut Label) {
    let name = cond.branch_mnemonic();
    let cond_code = if matches!(cond, ConditionA64::Always | ConditionA64::Count) {
      0x3F
    } else {
      cond.code() as u8
    };
    self.place_bc(name, label, 0b0101_0100, cond_code);
  }

  pub fn bic(&mut self, dst: RegisterA64, src1: RegisterA64, src2: RegisterA64, shift: i32) {
    self.place_sr_3("bic", dst, src1, src2, 0b00_01010, shift, 1);
  }

  pub fn bif(&mut self, dst: RegisterA64, src: RegisterA64, mask: RegisterA64) {
    if self.log_text {
      self.log_append(format_args!(
        " {:<12}v{}.16b,v{}.16b,v{}.16b\n",
        "bif",
        dst.index(),
        src.index(),
        mask.index()
      ));
    }

    let op: u32 = 0b0110_1110_1110_0000_0001_1100_0000_0000;

    self.place(
      (dst.index() as u32) | ((src.index() as u32) << 5) | ((mask.index() as u32) << 16) | op,
    );

    self.commit();
  }

  pub fn bit(&mut self, dst: RegisterA64, src: RegisterA64, mask: RegisterA64) {
    if self.log_text {
      self.log_append(format_args!(
        " {:<12}v{}.16b,v{}.16b,v{}.16b\n",
        "bit",
        dst.index(),
        src.index(),
        mask.index()
      ));
    }

    let op: u32 = 0b0110_1110_1010_0000_0001_1100_0000_0000;

    self.place(dst.index() as u32 | (src.index() as u32) << 5 | (mask.index() as u32) << 16 | op);

    self.commit();
  }

  pub fn bl(&mut self, label: &mut Label) {
    self.place_b("bl", label, 0b1_00101);
  }

  pub fn blr(&mut self, src: RegisterA64) {
    self.place_br("blr", src, 0b11_0101_1000_1111_1100_0000);
  }

  pub fn br(&mut self, src: RegisterA64) {
    self.place_br("br", src, 0b11_0101_1000_0111_1100_0000);
  }

  pub fn cbnz(&mut self, src: RegisterA64, label: &mut Label) {
    self.place_bcr("cbnz", "cbz", label, 0b011_0101, src);
  }

  pub fn cbz(&mut self, src: RegisterA64, label: &mut Label) {
    self.place_bcr("cbz", "cbnz", label, 0b011_0100, src);
  }

  pub fn ccmn_register_a_64_register_a_64_condition_a_64_u8(
    &mut self,
    src1: RegisterA64,
    src2: RegisterA64,
    cond: ConditionA64,
    nzcv: u8,
  ) {
    if self.log_text {
      self.log_append(format_args!("{:<12}", "ccmn"));
      self.log_reg(src1);
      self.text.push(',');
      self.log_reg(src2);
      self.log_append(format_args!(",#{},{}", nzcv, cond.as_str()));
    }

    debug_assert!(matches!(src1.kind(), KindA64::W | KindA64::X));
    debug_assert!(src2.kind() == src1.kind());

    let sf = src1.kind().sf_bit();

    // ccmn: sf 01 11010010 Rm cond 00 Rn 0 nzcv
    let word = (nzcv & 0x0F) as u32
      | ((src1.index() as u32) << 5)
      | (cond.code() << 12)
      | ((src2.index() as u32) << 16)
      | (0b0111010010u32 << 21)
      | sf;

    self.place(word);
    self.commit();
  }

  pub fn ccmn_register_a_64_u8_condition_a_64_u8(
    &mut self,
    src1: RegisterA64,
    src2: u8,
    cond: ConditionA64,
    nzcv: u8,
  ) {
    if self.log_text {
      self.log_append(format_args!("{:<12}", "ccmn"));
      self.log_reg(src1);
      self.log_append(format_args!(",#{},#{},{}", src2, nzcv, cond.as_str()));
    }

    CODEGEN_ASSERT!(matches!(src1.kind(), KindA64::W | KindA64::X));
    CODEGEN_ASSERT!(src2 <= 31);

    let sf = src1.kind().sf_bit();

    // ccmn: sf 01 11010010 imm5 cond 10 Rn 0 nzcv
    let word = (nzcv & 0x0F) as u32
      | ((src1.index() as u32) << 5)
      | (1 << 11)
      | (cond.code() << 12)
      | ((src2 as u32) << 16)
      | (0b0111010010u32 << 21)
      | sf;

    self.place(word);
    self.commit();
  }

  pub fn ccmp(&mut self, src1: RegisterA64, src2: RegisterA64, cond: ConditionA64, nzcv: u8) {
    if self.log_text {
      self.log_append(format_args!("{:<12}", "ccmp"));
      self.log_reg(src1);
      self.text.push(',');
      self.log_reg(src2);
      self.log_append(format_args!(",#{},{}", nzcv & 0x0F, cond as u32));
    }

    assert!(matches!(src1.kind(), KindA64::W | KindA64::X));
    assert!(src2.kind() == src1.kind());

    let sf = src1.kind().sf_bit();

    let word = (nzcv & 0x0F) as u32
      | ((src1.index() as u32) << 5)
      | (cond.code() << 12)
      | ((src2.index() as u32) << 16)
      | (0b1111010010u32 << 21)
      | sf;

    self.place(word);
    self.commit();
  }

  pub fn clz(&mut self, dst: RegisterA64, src: RegisterA64) {
    debug_assert!(matches!(dst.kind(), KindA64::W | KindA64::X));
    debug_assert!(dst.kind() == src.kind());

    self.place_r_1("clz", dst, src, 0b1_0110_1011_0000_0000_0100);
  }

  pub fn cmn(&mut self, src1: RegisterA64, src2: u16) {
    let dst = src1.zero_reg();

    self.place_i12("cmn", dst, src1, src2 as i32, 0b01_10001);
  }

  pub fn cmp_register_a_64_register_a_64(&mut self, src1: RegisterA64, src2: RegisterA64) {
    let dst = src1.zero_reg();

    self.place_sr_3("cmp", dst, src1, src2, 0b11_01011, 0, 0);
  }

  pub fn cmp_register_a_64_u16(&mut self, src1: RegisterA64, src2: u16) {
    let dst = src1.zero_reg();

    self.place_i12("cmp", dst, src1, src2 as i32, 0b11_10001);
  }

  pub fn commit(&mut self) {
    CODEGEN_ASSERT!(self.code_pos <= self.code_end);

    if self.code_end == self.code_pos {
      self.extend();
    }
  }

  pub fn csel(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
    cond: ConditionA64,
  ) {
    debug_assert!(matches!(dst.kind(), KindA64::X | KindA64::W));

    self.place_cs("csel", dst, src1, src2, cond, 0b11010100, 0b00, 0);
  }

  pub fn cset(&mut self, dst: RegisterA64, cond: ConditionA64) {
    CODEGEN_ASSERT!(matches!(dst.kind(), KindA64::X | KindA64::W));

    let src = dst.zero_reg();

    self.place_cs("cset", dst, src, src, cond, 0b1101_0100, 0b01, 1);
  }

  // sdiv/udiv 同构对（cpp AssemblyBuilderA64.cpp :208-220 / :223-235）：
  // 除助记符与位 [12:10] opc 段（sdiv 0b000011、udiv 0b000010）外逐字相同，
  // 共享骨架收敛于此，日志文本与发射编码不变。
  fn place_div(
    &mut self,
    mnem: &str,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
    opc: u32,
  ) {
    if self.log_text {
      self.log_shift(mnem, dst, src1, src2, 0);
    }

    // 此处避免 CODEGEN_ASSERT!：它展开为 ulua_common::assert_call_handler，
    // 后者期望 *const i8 参数，其他调用点可能类型不匹配。
    assert!(matches!(dst.kind(), KindA64::W | KindA64::X));
    assert!(dst.kind() == src1.kind() && dst.kind() == src2.kind());

    let sf = dst.kind().sf_bit();

    self.place(
      (dst.index() as u32)
        | ((src1.index() as u32) << 5)
        | (opc << 10)
        | ((src2.index() as u32) << 16)
        | (0b0011010110u32 << 21)
        | sf,
    );
    self.commit();
  }

  pub fn sdiv(&mut self, dst: RegisterA64, src1: RegisterA64, src2: RegisterA64) {
    self.place_div("sdiv", dst, src1, src2, 0b000011); // cpp :218
  }

  pub fn udiv(&mut self, dst: RegisterA64, src1: RegisterA64, src2: RegisterA64) {
    self.place_div("udiv", dst, src1, src2, 0b000010); // cpp :233
  }

  /// dup（scalar/vector 4s 臂同构，cpp AssemblyBuilderA64.cpp dup 模板）：
  /// S 臂与 Q 臂仅 dst 助记符前缀（`s{}` vs `v{}.4s`）与编码 Q 位（op 段 bit18）不同，
  /// 其余断言/日志/发射序列一致，收敛为单臂参数化。
  pub fn dup_4s(&mut self, dst: RegisterA64, src: RegisterA64, index: u8) {
    let scalar = dst.kind() == KindA64::S;

    CODEGEN_ASSERT!(src.kind() == KindA64::Q);
    CODEGEN_ASSERT!(index < 4);

    if self.log_text {
      let dst_name = if scalar {
        format_args!("s{}", dst.index())
      } else {
        format_args!("v{}.4s", dst.index())
      };
      self.log_append(format_args!(
        " {:<12}{},v{}.s[{}]\n",
        "dup",
        dst_name,
        src.index(),
        index
      ));
    }

    // 标量（s{}）臂在 op 段内置 Q 位（编码 bit18，place 前整体左移 10）
    let op: u32 = 0b01_0011_1000_0001_0000_0001 | ((scalar as u32) << 18);
    self.place(dst.index() as u32 | (src.index() as u32) << 5 | op << 10 | (index as u32) << 19);

    self.commit();
  }

  pub fn eor_register_a_64_register_a_64_register_a_64_i32(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
    shift: i32,
  ) {
    self.place_sr_3("eor", dst, src1, src2, 0b1001010, shift, 0);
  }

  pub fn eor_register_a_64_register_a_64_u32(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: u32,
  ) {
    self.place_bm("eor", dst, src1, src2, 0b10100100);
  }

  pub fn extend(&mut self) {
    let count = self.get_code_size();

    let new_size = self.code.len().wrapping_mul(2);
    self.code.resize(new_size, 0);

    let data_ptr = self.code.as_mut_ptr();
    // Safety: code 刚 resize 到 new_size(=2*old_len) 且 len=new_size, data_ptr 指向该存活 u32 缓冲首元素;
    // count=get_code_size()<=old_len<=new_len, 故 data_ptr.add(count) 落在 [ptr, ptr+len] 内、u32 对齐。
    self.code_pos = unsafe { data_ptr.add(count as usize) };
    // Safety: data_ptr.add(self.code.len()) 为该存活 u32 缓冲的合法 one-past-end 哨兵, 仅计算地址不解引用。
    self.code_end = unsafe { data_ptr.add(self.code.len()) };
  }

  pub fn fadd(&mut self, dst: RegisterA64, src1: RegisterA64, src2: RegisterA64) {
    if dst.kind() == KindA64::D {
      debug_assert!(src1.kind() == KindA64::D && src2.kind() == KindA64::D);
      self.place_r_3("fadd", dst, src1, src2, 0b1111_0011, 0b00_1010);
    } else if dst.kind() == KindA64::S {
      debug_assert!(src1.kind() == KindA64::S && src2.kind() == KindA64::S);
      self.place_r_3("fadd", dst, src1, src2, 0b1111_0001, 0b00_1010);
    } else {
      debug_assert!(
        dst.kind() == KindA64::Q && src1.kind() == KindA64::Q && src2.kind() == KindA64::Q
      );
      self.place_vr("fadd", dst, src1, src2, 0b0_0111_0001, 0b11_0101);
    }
  }

  pub fn faddp(&mut self, dst: RegisterA64, src: RegisterA64) {
    CODEGEN_ASSERT!(matches!(dst.kind(), KindA64::D | KindA64::S));
    CODEGEN_ASSERT!(dst.kind() == src.kind());

    let is_d = if dst.kind() == KindA64::D { 1 } else { 0 };
    let op = 0b01_1111_1000_1100_0011_0110 | (is_d << 12);

    self.place_r_1("faddp", dst, src, op);
  }

  // fcmeq_4s/fcmgt_4s 同构对（cpp AssemblyBuilderA64.cpp :1113-1124 / :1126-1137）：
  // 除助记符与整条 32 位 op 外逐字相同（v.4s 三源日志 + place + commit），
  // 共享骨架收敛于此，日志文本与发射编码不变。
  fn place_fcmm_4s(
    &mut self,
    mnem: &str,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
    op: u32,
  ) {
    if self.log_text {
      self.log_append(format_args!(
        " {:<12}v{}.4s,v{}.4s,v{}.4s\n",
        mnem,
        dst.index(),
        src1.index(),
        src2.index()
      ));
    }

    self.place(
      (dst.index() as u32) | ((src1.index() as u32) << 5) | ((src2.index() as u32) << 16) | op,
    );

    self.commit();
  }

  pub fn fcmeq_4s(&mut self, dst: RegisterA64, src1: RegisterA64, src2: RegisterA64) {
    //               Q U      ESz Rm    Opcode Rn    Rd
    self.place_fcmm_4s(
      "fcmeq",
      dst,
      src1,
      src2,
      0b0100_1110_0010_0000_1110_0100_0000_0000,
    ); // cpp :1113-1124
  }

  pub fn fcmgt_4s(&mut self, dst: RegisterA64, src1: RegisterA64, src2: RegisterA64) {
    //               Q U      ESz Rm    Opcode Rn    Rd
    self.place_fcmm_4s(
      "fcmgt",
      dst,
      src1,
      src2,
      0b0110_1110_1010_0000_1110_0100_0000_0000,
    ); // cpp :1126-1137
  }

  pub fn fcmp(&mut self, src1: RegisterA64, src2: RegisterA64) {
    debug_assert!(src1.kind() == src2.kind());
    debug_assert!(matches!(src1.kind(), KindA64::D | KindA64::S));

    if src1.kind() == KindA64::D {
      self.assembly_builder_a_64_place_fcmp("fcmp", src1, src2, 0b1111_0011, 0b00);
    } else {
      self.assembly_builder_a_64_place_fcmp("fcmp", src1, src2, 0b1111_0001, 0b00);
    }
  }

  pub fn fcmpz(&mut self, src: RegisterA64) {
    CODEGEN_ASSERT!(matches!(src.kind(), KindA64::D | KindA64::S));

    let zero_reg = RegisterA64 {
      bits: src.kind() as u8,
    };

    if src.kind() == KindA64::D {
      self.assembly_builder_a_64_place_fcmp("fcmp", src, zero_reg, 0b1111_0011, 0b01);
    } else {
      self.assembly_builder_a_64_place_fcmp("fcmp", src, zero_reg, 0b1111_0001, 0b01);
    }
  }

  pub fn fcsel(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
    cond: ConditionA64,
  ) {
    CODEGEN_ASSERT!(dst.kind() == src1.kind() && src1.kind() == src2.kind());
    CODEGEN_ASSERT!(matches!(dst.kind(), KindA64::D | KindA64::S));

    if src1.kind() == KindA64::D {
      self.place_cs("fcsel", dst, src1, src2, cond, 0b1111_0011, 0b11, 0);
    } else {
      self.place_cs("fcsel", dst, src1, src2, cond, 0b1111_0001, 0b11, 0);
    }
  }

  pub fn fcvt(&mut self, dst: RegisterA64, src: RegisterA64) {
    if dst.kind() == KindA64::S && src.kind() == KindA64::D {
      self.place_r_1("fcvt", dst, src, 0b111_1001_1000_1001_0000);
    } else if dst.kind() == KindA64::D && src.kind() == KindA64::S {
      self.place_r_1("fcvt", dst, src, 0b111_1000_1000_1011_0000);
    } else {
      ulua_common::LUAU_ASSERT!(false);
    }
  }

  pub fn fcvtzs(&mut self, dst: RegisterA64, src: RegisterA64) {
    debug_assert!(matches!(dst.kind(), KindA64::W | KindA64::X));
    debug_assert!(src.kind() == KindA64::D);

    self.place_r_1("fcvtzs", dst, src, 0b00_0111_1001_1110_0000_0000);
  }

  pub fn fcvtzu(&mut self, dst: RegisterA64, src: RegisterA64) {
    debug_assert!(matches!(dst.kind(), KindA64::W | KindA64::X));
    debug_assert!(src.kind() == KindA64::D);

    self.place_r_1("fcvtzu", dst, src, 0b00_0111_1001_1110_0100_0000);
  }

  pub fn fdiv(&mut self, dst: RegisterA64, src1: RegisterA64, src2: RegisterA64) {
    if dst.kind() == KindA64::D {
      // 避免调用 CODEGEN_ASSERT!：它经 ulua_common::assert_call_handler 展开，
      // 目前会在本翻译集中造成类型不匹配。
      debug_assert!(src1.kind() == KindA64::D && src2.kind() == KindA64::D);

      self.place_r_3("fdiv", dst, src1, src2, 0b1111_0011, 0b00_0110);
    } else if dst.kind() == KindA64::S {
      debug_assert!(src1.kind() == KindA64::S && src2.kind() == KindA64::S);

      self.place_r_3("fdiv", dst, src1, src2, 0b1111_0001, 0b00_0110);
    } else {
      debug_assert!(
        dst.kind() == KindA64::Q && src1.kind() == KindA64::Q && src2.kind() == KindA64::Q
      );

      self.place_vr("fdiv", dst, src1, src2, 0b1_0111_0001, 0b11_1111);
    }
  }

  pub fn finalize(&mut self) -> bool {
    let code_pos_ptr = self.code_pos as *const u32;
    let code_data_ptr = self.code.as_ptr();
    // Safety: code_pos_ptr 与 self.code.as_ptr() 同源自一个 code 分配，code_pos 为 ≥ 基址的写游标，
    // offset_from 在同一分配同序指针上合法，得已写入的 u32 元素数用于随后 resize。
    let code_len = unsafe { code_pos_ptr.offset_from(code_data_ptr) as usize };
    self.code.resize(code_len, 0);

    let pending_labels = take(&mut self.pending_labels);
    for fixup in pending_labels {
      let label = fixup.label;
      CODEGEN_ASSERT!(self.label_locations[label as usize - 1] != !0u32);
      let value = (self.label_locations[label as usize - 1] as i32) - (fixup.location as i32);

      self.patch_offset(fixup.location, value, fixup.kind);
    }

    let data_size = self.data.len() - self.data_pos;

    if data_size > 0 {
      self.data.copy_within(self.data_pos.., 0);
    }

    self.data.resize(data_size, 0);

    self.finalized = true;

    !self.overflowed
  }

  pub fn fjcvtzs(&mut self, dst: RegisterA64, src: RegisterA64) {
    debug_assert!(dst.kind() == KindA64::W);
    debug_assert!(src.kind() == KindA64::D);
    debug_assert!(FeaturesA64::FeatureJscvt.is_set(self.features));

    self.place_r_1("fjcvtzs", dst, src, 0b00_0111_1001_1111_1000_0000);
  }

  pub fn fmla(&mut self, dst: RegisterA64, src1: RegisterA64, src2: RegisterA64) {
    // FMLA 指令没有标量版本
    // 两种情形都用向量指令，配好 sz 位。

    //                Q U        Sz  Rm    Opcode Rn    Rd
    let op: u32 = 0b0000_1110_0010_0000_1100_1100_0000_0000;
    let q_bit: u32 = 1 << 30;
    let sz_bit: u32 = 1 << 22;

    if dst.kind() == KindA64::D {
      assert!(src1.kind() == KindA64::D && src2.kind() == KindA64::D);

      if self.log_text {
        self.log_append(format_args!(
          " {:<12}sd{},d{},d{}\n",
          "fmla",
          dst.index(),
          src1.index(),
          src2.index()
        ));
      }

      self.place(
        (dst.index() as u32)
          | ((src1.index() as u32) << 5)
          | ((src2.index() as u32) << 16)
          | op
          | q_bit
          | sz_bit,
      );
    } else if dst.kind() == KindA64::S {
      assert!(src1.kind() == KindA64::S && src2.kind() == KindA64::S);

      if self.log_text {
        self.log_append(format_args!(
          " {:<12}ss{},s{},s{}\n",
          "fmla",
          dst.index(),
          src1.index(),
          src2.index()
        ));
      }

      self.place(
        (dst.index() as u32) | ((src1.index() as u32) << 5) | ((src2.index() as u32) << 16) | op,
      );
    } else {
      assert!(dst.kind() == KindA64::Q && src1.kind() == KindA64::Q && src2.kind() == KindA64::Q);

      if self.log_text {
        self.log_append(format_args!(
          " {:<12}sv{}.4s,v{}.4s,v{}.4s\n",
          "fmla",
          dst.index(),
          src1.index(),
          src2.index()
        ));
      }

      self.place(
        (dst.index() as u32)
          | ((src1.index() as u32) << 5)
          | ((src2.index() as u32) << 16)
          | op
          | q_bit,
      );
    }

    self.commit();
  }

  pub fn fmov_register_a_64_register_a_64(&mut self, dst: RegisterA64, src: RegisterA64) {
    if dst.kind() == KindA64::D && src.kind() == KindA64::D {
      self.place_r_1("fmov", dst, src, 0b0_0111_1001_1000_0001_0000);
    } else if dst.kind() == KindA64::D && src.kind() == KindA64::X {
      self.place_r_1("fmov", dst, src, 0b0_0111_1001_1001_1100_0000);
    } else if dst.kind() == KindA64::X && src.kind() == KindA64::D {
      self.place_r_1("fmov", dst, src, 0b0_0111_1001_1001_1000_0000);
    } else if dst.kind() == KindA64::S && src.kind() == KindA64::S {
      self.place_r_1("fmov", dst, src, 0b0_0111_1000_1000_0001_0000);
    } else if dst.kind() == KindA64::S && src.kind() == KindA64::W {
      self.place_r_1("fmov", dst, src, 0b0_0111_1000_1001_1100_0000);
    } else {
      // 编译器内部不变量（cpp AssemblyBuilderA64.cpp:803 同位 CODEGEN_ASSERT(!"Unsupported fmov kind")）：
      // 调用方仅传上面枚举的 5 种 D/X/S/W 组合，其余 kind 组合无合法编码器。
      panic!("不支持的 fmov 寄存器 kind 组合");
    }
  }

  pub fn fmov_register_a_64_f64(&mut self, dst: RegisterA64, src: f64) {
    let dst_kind = dst.kind();
    debug_assert!(matches!(dst_kind, KindA64::D | KindA64::Q));

    let imm = get_fmov_imm_fp_64(src);
    debug_assert!((0..=256).contains(&imm));

    // fmov 无法编码 0，但 movi 可以；movi 编码重复模式，不适合一般 fp 立即数
    if dst_kind == KindA64::D {
      if imm == 256 {
        self.place_fmov("movi", dst, src, 0b001_0111_1000_0000_0111_0010_0000);
      } else {
        self.place_fmov(
          "fmov",
          dst,
          src,
          0b000_1111_0011_0000_0000_1000_0000 | ((imm as u32) << 8),
        );
      }
    } else {
      if imm == 256 {
        self.place_fmov("movi.4s", dst, src, 0b010_0111_1000_0000_0000_0010_0000);
      } else {
        self.place_fmov(
          "fmov.4s",
          dst,
          src,
          0b010_0111_1000_0000_0111_1010_0000 | ((((imm as u32) >> 5) << 11) | (imm as u32 & 31)),
        );
      }
    }
  }

  pub fn fmov_register_a_64_f32(&mut self, dst: RegisterA64, src: f32) {
    debug_assert!(matches!(dst.kind(), KindA64::S | KindA64::Q));

    let imm = get_fmov_imm_fp_32(src);
    debug_assert!((0..=256).contains(&imm));

    // fmov 无法编码 0，但 movi 可以；movi 编码重复模式，不适合一般 fp 立即数
    if dst.kind() == KindA64::S {
      if imm == 256 {
        self.place_fmov("movi", dst, src as f64, 0b001_0111_1000_0000_0111_0010_0000);
      } else {
        self.place_fmov(
          "fmov",
          dst,
          src as f64,
          0b000_1111_0001_0000_0000_1000_0000 | ((imm as u32) << 8),
        );
      }
    } else {
      if imm == 256 {
        self.place_fmov(
          "movi.4s",
          dst,
          src as f64,
          0b010_0111_1000_0000_0000_0010_0000,
        );
      } else {
        self.place_fmov(
          "fmov.4s",
          dst,
          src as f64,
          0b010_0111_1000_0000_0111_1010_0000 | (((imm as u32) >> 5) << 11) | (imm as u32 & 31),
        );
      }
    }
  }

  pub fn fmul(&mut self, dst: RegisterA64, src1: RegisterA64, src2: RegisterA64) {
    if dst.kind() == KindA64::D {
      // CODEGEN_ASSERT! 现经 ulua_common::assert_call_handler 展开，本翻译集
      // 中存在指针与 &str 的签名不匹配，故改用 debug_assert!。
      debug_assert!(src1.kind() == KindA64::D && src2.kind() == KindA64::D);

      self.place_r_3("fmul", dst, src1, src2, 0b1111_0011, 0b00_0010);
    } else if dst.kind() == KindA64::S {
      debug_assert!(src1.kind() == KindA64::S && src2.kind() == KindA64::S);

      self.place_r_3("fmul", dst, src1, src2, 0b1111_0001, 0b00_0010);
    } else {
      debug_assert!(
        dst.kind() == KindA64::Q && src1.kind() == KindA64::Q && src2.kind() == KindA64::Q
      );

      self.place_vr("fmul", dst, src1, src2, 0b1_0111_0001, 0b11_0111);
    }
  }

  pub fn fneg(&mut self, dst: RegisterA64, src: RegisterA64) {
    if dst.kind() == KindA64::D {
      debug_assert!(src.kind() == KindA64::D);

      self.place_r_1("fneg", dst, src, 0b00_0111_1001_1000_0101_0000);
    } else if dst.kind() == KindA64::S {
      debug_assert!(src.kind() == KindA64::S);

      self.place_r_1("fneg", dst, src, 0b00_0111_1000_1000_0101_0000);
    } else {
      debug_assert!(dst.kind() == KindA64::Q && src.kind() == KindA64::Q);

      self.place_r_1("fneg", dst, src, 0b01_1011_1010_1000_0011_1110);
    }
  }

  pub fn fsqrt(&mut self, dst: RegisterA64, src: RegisterA64) {
    debug_assert!(dst.kind() == src.kind());
    debug_assert!(matches!(dst.kind(), KindA64::D | KindA64::S));

    if dst.kind() == KindA64::D {
      self.place_r_1("fsqrt", dst, src, 0b00_0111_1001_1000_0111_0000);
    } else {
      self.place_r_1("fsqrt", dst, src, 0b00_0111_1000_1000_0111_0000);
    }
  }

  pub fn fsub(&mut self, dst: RegisterA64, src1: RegisterA64, src2: RegisterA64) {
    if dst.kind() == KindA64::D {
      debug_assert!(src1.kind() == KindA64::D && src2.kind() == KindA64::D);

      self.place_r_3("fsub", dst, src1, src2, 0b1111_0011, 0b00_1110);
    } else if dst.kind() == KindA64::S {
      debug_assert!(src1.kind() == KindA64::S && src2.kind() == KindA64::S);

      self.place_r_3("fsub", dst, src1, src2, 0b1111_0001, 0b00_1110);
    } else {
      debug_assert!(
        dst.kind() == KindA64::Q && src1.kind() == KindA64::Q && src2.kind() == KindA64::Q
      );

      self.place_vr("fsub", dst, src1, src2, 0b0_0111_0101, 0b11_0101);
    }
  }

  pub fn get_code_size(&self) -> u32 {
    // C++：uint32_t(codePos - code.data())，code 是 std::vector<uint32_t>，
    // 指针相减得到元素（字）数而非字节数。
    let code_pos = self.code_pos as *const u32;
    let code_data = self.code.as_ptr();
    // Safety: code_pos 与 code.as_ptr() 均源自同一 self.code 分配（code_pos 为其内部写游标，
    // 只前进故 ≥ 基址），offset_from 在同一分配同序指针上合法，返回 u32 元素（字）数差，
    // 与 C++ uint32_t(codePos-code.data()) 语义一致。
    let count = unsafe { code_pos.offset_from(code_data) };
    u32::try_from(count).unwrap_or(u32::MAX)
  }

  pub fn get_instruction_count(&self) -> u32 {
    self.get_code_size()
  }

  pub fn get_label_offset(&self, label: &Label) -> u32 {
    CODEGEN_ASSERT!(label.location != !0u32);
    label.location * 4
  }

  pub fn ins_4_s_register_a_64_register_a_64_u8(
    &mut self,
    dst: RegisterA64,
    src: RegisterA64,
    index: u8,
  ) {
    debug_assert!(dst.kind() == KindA64::Q && src.kind() == KindA64::W);
    debug_assert!(index < 4);

    if self.log_text {
      self.log_append(format_args!(
        " {:<12}v{}.s[{}],w{}\n",
        "ins",
        dst.index(),
        index,
        src.index()
      ));
    }

    let op: u32 = 0b01_0011_1000_0001_0000_0111;

    self.place(dst.index() as u32 | (src.index() as u32) << 5 | op << 10 | (index as u32) << 19);
    self.commit();
  }

  pub fn ins_4_s_register_a_64_u8_register_a_64_u8(
    &mut self,
    dst: RegisterA64,
    dst_index: u8,
    src: RegisterA64,
    src_index: u8,
  ) {
    debug_assert!(dst.kind() == KindA64::Q && src.kind() == KindA64::Q);
    debug_assert!(dst_index < 4);
    debug_assert!(src_index < 4);

    if self.log_text {
      self.log_append(format_args!(
        " {:<12}v{}.s[{}],v{}.s[{}]\n",
        "ins",
        dst.index(),
        dst_index,
        src.index(),
        src_index
      ));
    }

    let op: u32 = 0b01_1011_1000_0001_0000_0001;

    self.place(
      (dst.index() as u32)
        | ((src.index() as u32) << 5)
        | (op << 10)
        | ((dst_index as u32) << 19)
        | ((src_index as u32) << 13),
    );
    self.commit();
  }

  // fmov 立即数可编码性探测（cpp 侧以 getFmovImm>=0 就地内联，无独立函数）：
  // fp_32/fp_64 双生仅差宽度与对应 get_fmov_imm 助手。
  pub fn is_fmov_supported_fp_32(&mut self, value: f32) -> bool {
    get_fmov_imm_fp_32(value) >= 0
  }

  pub fn is_fmov_supported_fp_64(&mut self, value: f64) -> bool {
    get_fmov_imm_fp_64(value) >= 0
  }

  pub fn is_mask_supported(&mut self, mask: u32) -> bool {
    let lz = countlz_u32(mask);
    let rz = countrz_u32(mask);

    lz + rz > 0 && lz + rz < 32 && (mask >> rz) == (1u32 << (32 - lz - rz)) - 1
  }

  pub fn ldp(&mut self, dst1: RegisterA64, dst2: RegisterA64, src: AddressA64) {
    debug_assert!(matches!(dst1.kind(), KindA64::X | KindA64::W));
    debug_assert!(dst1.kind() == dst2.kind());

    let is_x = dst1.kind() == KindA64::X;
    self.place_p(
      "ldp",
      dst1,
      dst2,
      src,
      0b1010_0101,
      (is_x as u8) << 1,
      if is_x { 3 } else { 2 },
    );
  }

  pub fn ldr(&mut self, dst: RegisterA64, src: AddressA64) {
    match dst.kind() {
      KindA64::W => self.place_a("ldr", dst, src, 0b10_1110_0001, 2),
      KindA64::X => self.place_a("ldr", dst, src, 0b11_1110_0001, 3),
      KindA64::S => self.place_a("ldr", dst, src, 0b10_1111_0001, 2),
      KindA64::D => self.place_a("ldr", dst, src, 0b11_1111_0001, 3),
      KindA64::Q => self.place_a("ldr", dst, src, 0b00_1111_0011, 4),
      KindA64::None => {
        CODEGEN_ASSERT!(false, "Unexpected register kind");
        LUAU_UNREACHABLE!();
      }
    }
  }

  pub fn ldrb(&mut self, dst: RegisterA64, src: AddressA64) {
    debug_assert!(dst.kind() == KindA64::W);

    self.place_a("ldrb", dst, src, 0b00_1110_0001, 0);
  }

  pub fn ldrh(&mut self, dst: RegisterA64, src: AddressA64) {
    debug_assert!(dst.kind() == KindA64::W);

    self.place_a("ldrh", dst, src, 0b01_1110_0001, 1);
  }

  pub fn ldrsb(&mut self, dst: RegisterA64, src: AddressA64) {
    debug_assert!(matches!(dst.kind(), KindA64::X | KindA64::W));

    let opsize = if dst.kind() == KindA64::W {
      0b00_1110_0010 | 0b01
    } else {
      0b00_1110_0010
    };

    self.place_a("ldrsb", dst, src, opsize as u16, 0);
  }

  pub fn ldrsh(&mut self, dst: RegisterA64, src: AddressA64) {
    debug_assert!(matches!(dst.kind(), KindA64::X | KindA64::W));

    let opsize = if dst.kind() == KindA64::W {
      0b01_1110_0010 | 0b01
    } else {
      0b01_1110_0010
    };

    self.place_a("ldrsh", dst, src, opsize as u16, 1);
  }

  pub fn ldrsw(&mut self, dst: RegisterA64, src: AddressA64) {
    debug_assert!(dst.kind() == KindA64::X);

    self.place_a("ldrsw", dst, src, 0b10_1110_0010, 2);
  }

  pub fn log(&mut self, opcode: &str) {
    self.log_append(format_args!(" {}\n", opcode));
  }

  pub fn log1(&mut self, opcode: &str, src: RegisterA64) {
    self.log_append(format_args!(" {:<12}", opcode));
    self.log_reg(src);
    self.text.push('\n');
  }

  pub fn log2(&mut self, opcode: &str, dst: RegisterA64, src: RegisterA64) {
    self.log_append(format_args!(" {:<12}", opcode));
    self.log_reg(dst);
    self.text.push(',');
    self.log_reg(src);
    self.text.push('\n');
  }

  pub fn log_imm(&mut self, opcode: &str, dst: RegisterA64, src1: RegisterA64, src2: i32) {
    self.log_append(format_args!(" {:<12}", opcode));

    if dst != RegisterA64::XZR && dst != RegisterA64::WZR {
      self.log_reg(dst);
      self.text.push(',');
    }

    self.log_reg(src1);
    self.text.push(',');
    self.log_append(format_args!("#{}", src2));
    self.text.push('\n');
  }

  pub fn log_imm_shift(&mut self, opcode: &str, dst: RegisterA64, src: i32, shift: i32) {
    self.log_append(format_args!(" {:<12}", opcode));
    self.log_reg(dst);
    self.text.push(',');
    self.log_append(format_args!("#{}", src));
    if shift > 0 {
      self.log_append(format_args!(" LSL #{}", shift));
    }
    self.text.push('\n');
  }

  pub fn log_shift(
    &mut self,
    opcode: &str,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
    shift: i32,
  ) {
    self.log_append(format_args!(" {:<12}", opcode));

    if dst != RegisterA64::XZR && dst != RegisterA64::WZR {
      self.log_reg(dst);
      self.text.push(',');
    }

    self.log_reg(src1);
    self.text.push(',');
    self.log_reg(src2);

    if src1.kind() == KindA64::X && src2.kind() == KindA64::W {
      self.log_append(format_args!(" UXTW #{}", shift));
    } else if shift > 0 {
      self.log_append(format_args!(" LSL #{}", shift));
    } else if shift < 0 {
      self.log_append(format_args!(" LSR #{}", -shift));
    }

    self.text.push('\n');
  }

  pub fn log_cond(
    &mut self,
    opcode: &str,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
    cond: ConditionA64,
  ) {
    self.log_append(format_args!(" {:<12}", opcode));
    self.log_reg(dst);

    if src1 != RegisterA64::WZR && src1 != RegisterA64::XZR
      || src2 != RegisterA64::WZR && src2 != RegisterA64::XZR
    {
      self.text.push(',');
      self.log_reg(src1);
      self.text.push(',');
      self.log_reg(src2);
    }

    self.text.push(',');
    self.log_append(format_args!("{}", cond.as_str()));
    self.text.push('\n');
  }

  pub fn log_f64(&mut self, opcode: &str, dst: RegisterA64, src: f64) {
    self.log_append(format_args!(" {:<12}", opcode));
    self.log_reg(dst);
    self.text.push(',');
    // C++ 用 "#%.17g"；Rust 无 %g，Display（{}）输出最短可回读十进制，语义等价（如 0.25 -> "0.25"）。
    self.log_append(format_args!("#{}", src));
    self.text.push('\n');
  }

  pub fn log_addr(&mut self, opcode: &str, dst: RegisterA64, src: AddressA64) {
    self.log_append(format_args!(" {:<12}", opcode));
    self.log_reg(dst);
    self.text.push(',');
    self.log_address(src);
    self.text.push('\n');
  }

  pub fn log2_addr(&mut self, opcode: &str, dst1: RegisterA64, dst2: RegisterA64, src: AddressA64) {
    self.log_append(format_args!(" {:<12}", opcode));
    self.log_reg(dst1);
    self.text.push(',');
    self.log_reg(dst2);
    self.text.push(',');
    self.log_address(src);
    self.text.push('\n');
  }

  pub fn log_label(&mut self, opcode: &str, label: Label) {
    self.log_append(format_args!(" {:<12}.L{}\n", opcode, label.id));
  }

  pub fn log_reg_label(&mut self, opcode: &str, src: RegisterA64, label: Label, imm: i32) {
    self.log_append(format_args!(" {:<12}", opcode));
    self.log_reg(src);
    self.text.push(',');
    if imm >= 0 {
      self.log_append(format_args!("#{},", imm));
    }
    self.log_append(format_args!(".L{}\n", label.id));
  }

  pub fn log_bind(&mut self, label: Label) {
    self.log_append(format_args!(".L{}:\n", label.id));
  }

  pub fn log_reg(&mut self, reg: RegisterA64) {
    match reg.kind() {
      KindA64::W => {
        if reg.index() == 31 {
          self.text.push_str("wzr");
        } else {
          self.log_append(format_args!("w{}", reg.index()));
        }
      }
      KindA64::X => {
        if reg.index() == 31 {
          self.text.push_str("xzr");
        } else {
          self.log_append(format_args!("x{}", reg.index()));
        }
      }
      KindA64::S => {
        self.log_append(format_args!("s{}", reg.index()));
      }
      KindA64::D => {
        self.log_append(format_args!("d{}", reg.index()));
      }
      KindA64::Q => {
        self.log_append(format_args!("q{}", reg.index()));
      }
      KindA64::None => {
        if reg.index() == 31 {
          self.text.push_str("sp");
        } else {
          CODEGEN_ASSERT!(false, "Unexpected register kind");
        }
      }
    }
  }

  pub fn log_address(&mut self, addr: AddressA64) {
    self.text.push('[');
    match addr.kind {
      AddressKindA64::Reg => {
        self.log_reg(addr.base);
        self.text.push(',');
        self.log_reg(addr.offset);
      }
      AddressKindA64::Imm => {
        self.log_reg(addr.base);
        if addr.data != 0 {
          self.log_append(format_args!(",#{}", addr.data));
        }
      }
      AddressKindA64::Pre => {
        self.log_reg(addr.base);
        if addr.data != 0 {
          self.log_append(format_args!(",#{}", addr.data));
        }
        self.text.push(']');
        self.text.push('!');
        return;
      }
      AddressKindA64::Post => {
        self.log_reg(addr.base);
        self.text.push(']');
        self.text.push('!');
        if addr.data != 0 {
          self.log_append(format_args!(",#{}", addr.data));
        }
        return;
      }
    }
    self.text.push(']');
  }

  pub fn log_append(&mut self, args: Arguments<'_>) {
    let _ = write(&mut self.text, args);
  }

  pub fn lsl_register_a_64_register_a_64_register_a_64(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
  ) {
    self.place_r_3("lsl", dst, src1, src2, 0b11010110, 0b00_1000);
  }

  pub fn lsl_register_a_64_register_a_64_u8(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: u8,
  ) {
    let size = dst.kind().gpr_bits();

    debug_assert!(src2 as i32 >= 0);
    debug_assert!((src2 as i32) < size);

    self.place_bfm(
      "lsl",
      dst,
      src1,
      -(src2 as i32),
      0b10_100110,
      (-(src2 as i32)) & (size - 1),
      size - 1 - (src2 as i32),
    );
  }

  pub fn lsr_register_a_64_register_a_64_register_a_64(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
  ) {
    self.place_r_3("lsr", dst, src1, src2, 0b11010110, 0b001001);
  }

  pub fn lsr_register_a_64_register_a_64_u8(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: u8,
  ) {
    let size = dst.kind().gpr_bits();
    CODEGEN_ASSERT!(src2 < size as u8);

    self.place_bfm(
      "lsr",
      dst,
      src1,
      src2 as i32,
      0b10_100110,
      src2 as i32,
      size - 1,
    );
  }

  pub fn mov_register_a_64_register_a_64(&mut self, dst: RegisterA64, src: RegisterA64) {
    if dst.kind() != KindA64::Q {
      debug_assert!(matches!(dst.kind(), KindA64::W | KindA64::X) || dst == RegisterA64::SP);
      debug_assert!(
        dst.kind() == src.kind()
          || (dst.kind() == KindA64::X && src == RegisterA64::SP)
          || (dst == RegisterA64::SP && src.kind() == KindA64::X)
      );

      if dst == RegisterA64::SP || src == RegisterA64::SP {
        self.place_r_1("mov", dst, src, 0b0_0100_0100_0000_0000_0000);
      } else {
        self.place_sr_2("mov", dst, src, 0b01_01010, 0);
      }
    } else {
      debug_assert!(dst.kind() == src.kind());

      self.place_r_1(
        "mov",
        dst,
        src,
        0b1_0011_1010_1000_0000_0111 | ((src.index() as u32) << 6),
      );
    }
  }

  pub fn mov_register_a_64_i32(&mut self, dst: RegisterA64, src: i32) {
    if src >= 0 {
      self.movz(dst, (src & 0xffff) as u16, 0);
      if src > 0xffff {
        self.movk(dst, ((src >> 16) & 0xffff) as u16, 16);
      }
    } else {
      self.movn(dst, (!src & 0xffff) as u16, 0);
      if src < -0x10000 {
        self.movk(dst, ((src >> 16) & 0xffff) as u16, 16);
      }
    }
  }

  pub fn movk(&mut self, dst: RegisterA64, src: u16, shift: i32) {
    self.place_i16("movk", dst, src as i32, 0b1110_0101, shift);
  }

  pub fn movn(&mut self, dst: RegisterA64, src: u16, shift: i32) {
    self.place_i16("movn", dst, src as i32, 0b00_100101, shift);
  }

  pub fn movz(&mut self, dst: RegisterA64, src: u16, shift: i32) {
    self.place_i16("movz", dst, src as i32, 0b1010_0101, shift);
  }

  pub fn msub(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
    src3: RegisterA64,
  ) {
    if self.log_text {
      self.log_shift("msub", dst, src1, src2, 0);
    }

    // 此处避免 CODEGEN_ASSERT! 宏：它目前展开为 ulua_common::assert_call_handler，
    // 后者期望 *const i8 参数，其他调用点可能类型不匹配。
    assert!(matches!(dst.kind(), KindA64::W | KindA64::X));
    assert!(dst.kind() == src1.kind() && dst.kind() == src2.kind() && dst.kind() == src3.kind());

    let sf = dst.kind().sf_bit();

    self.place(
      (dst.index() as u32)
        | ((src1.index() as u32) << 5)
        | ((src3.index() as u32) << 10)
        | (1 << 15)
        | ((src2.index() as u32) << 16)
        | (0b0011011000u32 << 21)
        | sf,
    );
    self.commit();
  }

  pub fn mul(&mut self, dst: RegisterA64, src1: RegisterA64, src2: RegisterA64) {
    if self.log_text {
      self.log_shift("mul", dst, src1, src2, 0);
    }

    CODEGEN_ASSERT!(matches!(dst.kind(), KindA64::W | KindA64::X));
    CODEGEN_ASSERT!(dst.kind() == src1.kind() && dst.kind() == src2.kind());

    let sf = dst.kind().sf_bit();

    self.place(
      (dst.index() as u32)
        | ((src1.index() as u32) << 5)
        | (0b11111 << 10)
        | ((src2.index() as u32) << 16)
        | (0b0011011000u32 << 21)
        | sf,
    );
    self.commit();
  }

  pub fn mvn_(&mut self, dst: RegisterA64, src: RegisterA64) {
    self.place_sr_2("mvn", dst, src, 0b01_01010, 0b1);
  }

  pub fn neg(&mut self, dst: RegisterA64, src: RegisterA64) {
    self.place_sr_2("neg", dst, src, 0b10_01011, 0);
  }

  pub fn nop(&mut self, bytes: u32) {
    let count = bytes / 4;
    for _ in 0..count {
      self.place_0("nop", 0b11010101000000110010000000011111u32);
    }
  }

  pub fn orr_register_a_64_register_a_64_register_a_64_i32(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
    shift: i32,
  ) {
    self.place_sr_3("orr", dst, src1, src2, 0b01_01010, shift, 0);
  }

  pub fn orr_register_a_64_register_a_64_u32(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: u32,
  ) {
    self.place_bm("orr", dst, src1, src2, 0b01_100100);
  }

  /// cpp CodeGen/src/AssemblyBuilderA64.cpp:1852 patchDataRef：
  /// 数据引用在 ±2^18 字内用 ADR，否则降级为 ADRP+ADD 页相对寻址。
  /// `location` 为 ADR 指令所在的指令字序号，`pos` 为数据在 data 段中的字节偏移。
  pub fn patch_data_ref(&mut self, dst: RegisterA64, location: u32, pos: usize) {
    let offset = -(location as i32) - (((self.data.len() - pos) / 4) as i32);

    if offset > -(1 << 18) && offset < (1 << 18) {
      self.place_adr_c_char_register_a_64_u8("adr", dst, 0b10000);
      self.patch_offset(location, offset, Kind::IMM19);
    } else {
      let page_offset = (location * 4) & 0xfff;
      let target_from_page = page_offset as i64 + (offset as i64) * 4;

      self.place_adrp(dst, (target_from_page >> 12) as i32);
      self.add_register_a_64_register_a_64_u16(dst, dst, (target_from_page & 0xfff) as u16);
    }
  }

  /// cpp CodeGen/src/AssemblyBuilderA64.cpp:1677 placeADRP：
  /// 21 位立即数拆分为 immLo（bits 30:29）与 immHi（bits 23:5）
  pub fn place_adrp(&mut self, dst: RegisterA64, page_offset: i32) {
    if self.log_text {
      self.log_imm_shift("adrp", dst, page_offset, -1);
    }

    if !(-(1 << 20)..(1 << 20)).contains(&page_offset) {
      self.overflowed = true;
      return;
    }

    let imm_lo = (page_offset as u32) & 0x3;
    let imm_hi = ((page_offset as u32) >> 2) & ((1 << 19) - 1);

    self
      .place(dst.index() as u32 | (imm_hi << 5) | (0b10000u32 << 24) | (imm_lo << 29) | (1 << 31));
    self.commit();
  }

  pub fn patch_label(&mut self, label: &mut Label, kind: Kind) {
    let location = self.get_code_size().wrapping_sub(1);

    if label.location == !0u32 {
      if label.id == 0 {
        label.id = self.next_label;
        self.next_label = self.next_label.wrapping_add(1);
        self.label_locations.push(!0u32);
      }

      self.pending_labels.push(Patch {
        kind,
        label: label.id,
        location,
      });
    } else {
      let value = label.location as i32 - location as i32;
      self.patch_offset(location, value, kind);
    }
  }

  /// cpp CodeGen/src/AssemblyBuilderA64.cpp:1870 patchLabelFar：
  /// 反向跳转超出 IMM19/IMM14 范围时，反转条件落到紧跟其后的 trampoline，
  /// 再经 IMM26 无条件跳转抵达远目标，并返回 skip label 供日志标注。
  pub fn patch_label_far(&mut self, label: &mut Label, kind: Kind, invert_bit: u32) -> Label {
    // 尚未放置的 label 仍按近跳转生成
    if label.location == !0u32 {
      self.patch_label(label, kind);
      return Label::default();
    }

    let location = self.get_code_size().wrapping_sub(1);

    // 检查反向跳转是否在范围内
    let value = label.location as i32 - location as i32;
    let range: i32 = if kind == Kind::IMM19 {
      1 << 19
    } else {
      1 << 14
    };

    if value > -(range >> 1) && value < (range >> 1) {
      self.patch_label(label, kind);
      return Label::default();
    }

    // 反转条件，跳到紧随其后的 trampoline
    self.code[location as usize] ^= 1u32 << invert_bit;
    self.patch_offset(location, 2, kind);

    // 放置更大范围的无条件跳转（与 placeB 相同编码但不打日志）
    self.place(0b0_00101 << 26);
    self.commit();

    self.patch_label(label, Kind::IMM26);

    let skip_label = Label {
      id: self.next_label,
      location: self.get_code_size(),
    };
    self.next_label = self.next_label.wrapping_add(1);
    self.label_locations.push(skip_label.location);
    skip_label
  }

  pub fn patch_offset(&mut self, location: u32, value: i32, kind: Kind) {
    let offset = if kind == Kind::IMM26 { 0 } else { 5 };
    let range = match kind {
      Kind::IMM19 => 1 << 19,
      Kind::IMM26 => 1 << 26,
      Kind::IMM14 => 1 << 14,
    };

    debug_assert!((self.code[location as usize] & (((range - 1) as u32) << offset)) == 0);

    if value > -(range >> 1) && value < (range >> 1) {
      self.code[location as usize] |= ((value as u32) & (range - 1) as u32) << offset;
    } else {
      self.overflowed = true;
    }
  }

  pub fn place(&mut self, word: u32) {
    if !(self.code_pos < self.code_end) {
      // Safety: assert_call_handler 为 C-ABI 诊断入口, 三个实参均为 `&[u8]`
      // 静态 NUL 结尾字节串常量的首指针, 满足 *const c_char 约定; 此块仅在不变量
      // 被破坏的诊断路径执行, 不触碰 code 缓冲。
      unsafe {
        ulua_common::assert_call_handler(
          K_ASSERT_INVARIANT.as_ptr().cast(),
          K_ASSERT_FILE.as_ptr().cast(),
          0,
          K_ASSERT_FUNCTION.as_ptr().cast(),
        );
        ulua_common::LUAU_DEBUGBREAK!();
      }
    }
    // Safety: 汇编流程维持不变量 code_pos<code_end——commit() 在剩余空间不足时先 extend() 扩容, 故本处
    // 写入 *code_pos=u32 落在存活的 self.code(u32) 缓冲界内且对齐, 自增 1 个字后仍不超过 code_end。
    unsafe {
      *self.code_pos = word;
      self.code_pos = self.code_pos.add(1);
    }
  }

  pub fn place_0(&mut self, name: &str, op: u32) {
    if self.log_text {
      self.log(name);
    }

    self.place(op);
    self.commit();
  }

  pub fn place_a(
    &mut self,
    name: &str,
    dst: RegisterA64,
    src: AddressA64,
    opsize: u16,
    sizelog: i32,
  ) {
    if self.log_text {
      self.log_addr(name, dst, src);
    }

    match src.kind {
      AddressKindA64::Reg => {
        self.place(
          dst.index() as u32
            | ((src.base.index() as u32) << 5)
            | (0b01_1010 << 10)
            | ((src.offset.index() as u32) << 16)
            | (1 << 21)
            | ((opsize as u32) << 22),
        );
      }
      AddressKindA64::Imm => {
        let data = src.data as u32;
        let shift_mask = (1 << sizelog) - 1;
        if (data >> sizelog as u32) < 1024 && (data & shift_mask) == 0 {
          self.place(
            dst.index() as u32
              | ((src.base.index() as u32) << 5)
              | ((data >> sizelog as u32) << 10)
              | ((opsize as u32) << 22)
              | (1 << 24),
          );
        } else if src.data >= -256 && src.data <= 255 {
          self.place(
            dst.index() as u32
              | ((src.base.index() as u32) << 5)
              | (((src.data as u32) & ((1 << 9) - 1)) << 12)
              | ((opsize as u32) << 22),
          );
        } else {
          self.overflowed = true;

          // 编译器内部不变量（cpp AssemblyBuilderA64.cpp:1512 同位
          // CODEGEN_ASSERT(!"Unable to encode large immediate offset")）：越过两档可编码
          // 窗口的立即偏移说明上层寻址折叠失效，继续编码只会产出静默错误指令。
          panic!("无法编码大立即偏移");
        }
      }
      AddressKindA64::Pre => {
        // 原代码用带指针 handler 的 CODEGEN_ASSERT；此处避免。
        assert!(src.data >= -256 && src.data <= 255);
        self.place(
          dst.index() as u32
            | ((src.base.index() as u32) << 5)
            | (0b11 << 10)
            | (((src.data as u32) & ((1 << 9) - 1)) << 12)
            | ((opsize as u32) << 22),
        );
      }
      AddressKindA64::Post => {
        // 原代码用带指针 handler 的 CODEGEN_ASSERT；此处避免。
        assert!(src.data >= -256 && src.data <= 255);
        self.place(
          dst.index() as u32
            | ((src.base.index() as u32) << 5)
            | (0b01 << 10)
            | (((src.data as u32) & ((1 << 9) - 1)) << 12)
            | ((opsize as u32) << 22),
        );
      }
    }

    self.commit();
  }

  pub fn place_adr_c_char_register_a_64_u8(&mut self, name: &str, dst: RegisterA64, op: u8) {
    if self.log_text {
      self.log1(name, dst);
    }

    // 此处避免调用 CODEGEN_ASSERT! 宏：它会触发
    // 代码库其他地方 assert_call_handler 参数的类型不匹配。
    debug_assert!(dst.kind() == KindA64::X);

    self.place((dst.index() as u32) | ((op as u32) << 24));
    self.commit();
  }

  pub fn place_adr_c_char_register_a_64_u8_label(
    &mut self,
    name: &str,
    dst: RegisterA64,
    op: u8,
    label: &mut Label,
  ) {
    assert!(dst.kind() == KindA64::X);

    self.place(dst.index() as u32 | ((op as u32) << 24));
    self.commit();

    self.patch_label(label, Kind::IMM19);

    if self.log_text {
      // C++ 记录 `log(name, dst, label)`；imm 参数默认 -1（不打印）。
      self.log_reg_label(name, dst, *label, -1);
    }
  }

  pub fn place_b(&mut self, name: &str, label: &mut Label, op: u8) {
    self.place((op as u32) << 26);
    self.commit();

    self.patch_label(label, Kind::IMM26);

    if self.log_text {
      self.log_label(name, *label);
    }
  }

  pub fn place_bc(&mut self, name: &str, label: &mut Label, op: u8, cond: u8) {
    self.place(cond as u32 | ((op as u32) << 24));
    self.commit();

    // cpp CodeGen/src/AssemblyBuilderA64.cpp:1544-1567：
    // FarRefs 开启时条件分支经 patchLabelFar 支持远距离回跳
    let skip_label = self.patch_label_far(label, Kind::IMM19, 0);

    if self.log_text {
      if skip_label.id != 0 {
        self.log_label(branch_mnemonic_from_code(cond ^ 1), skip_label);
        self.log_label("b", *label);
        self.log_bind(skip_label);
      } else {
        self.log_label(name, *label);
      }
    }
  }

  pub fn place_bcr(
    &mut self,
    name: &str,
    name_inv: &str,
    label: &mut Label,
    op: u8,
    cond: RegisterA64,
  ) {
    assert!(matches!(cond.kind(), KindA64::W | KindA64::X));

    let sf = cond.kind().sf_bit();

    self.place(cond.index() as u32 | ((op as u32) << 24) | sf);
    self.commit();

    // cpp CodeGen/src/AssemblyBuilderA64.cpp:1574-1604：
    // FarRefs 开启时 cbz/cbnz 经 patchLabelFar 支持远距离回跳
    let skip_label = self.patch_label_far(label, Kind::IMM19, 24);

    if self.log_text {
      if skip_label.id != 0 {
        self.log_reg_label(name_inv, cond, skip_label, -1);
        self.log_label("b", *label);
        self.log_bind(skip_label);
      } else {
        // C++ 记录 `log(name, cond, label)`；imm 参数默认 -1（不打印）。
        self.log_reg_label(name, cond, *label, -1);
      }
    }
  }

  pub fn place_bfm(
    &mut self,
    name: &str,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: i32,
    op: u8,
    immr: i32,
    imms: i32,
  ) {
    if self.log_text {
      self.log_imm(name, dst, src1, src2);
    }

    // 此处避免调用 CODEGEN_ASSERT! 宏：它会触发
    // 代码库其他地方 assert_call_handler 参数的类型不匹配。
    debug_assert!(matches!(dst.kind(), KindA64::W | KindA64::X));
    debug_assert!(dst.kind() == src1.kind());

    let sf = dst.kind().sf_bit();
    let n = if dst.kind().is_x64() { 1 << 22 } else { 0 };

    self.place(
      (dst.index() as u32)
        | ((src1.index() as u32) << 5)
        | ((imms as u32) << 10)
        | ((immr as u32) << 16)
        | n
        | ((op as u32) << 23)
        | sf,
    );
    self.commit();
  }

  pub fn place_bm(&mut self, name: &str, dst: RegisterA64, src1: RegisterA64, src2: u32, op: u8) {
    if self.log_text {
      self.log_imm(name, dst, src1, src2 as i32);
    }

    assert!(matches!(dst.kind(), KindA64::W | KindA64::X));
    assert!(dst.kind() == src1.kind());
    assert!(Self::is_mask_supported(self, src2));

    let sf = dst.kind().sf_bit();

    let lz = countlz_u32(src2);
    let rz = countrz_u32(src2);

    let imms = 31 - lz - rz;
    let immr = (32 - rz) & 31;

    self.place(
      (dst.index() as u32)
        | ((src1.index() as u32) << 5)
        | ((imms as u32) << 10)
        | ((immr as u32) << 16)
        | ((op as u32) << 23)
        | sf,
    );
    self.commit();
  }

  pub fn place_br(&mut self, name: &str, src: RegisterA64, op: u32) {
    if self.log_text {
      self.log1(name, src);
    }

    // 此处避免调用 CODEGEN_ASSERT! 宏：它会触发
    // 代码库其他地方 assert_call_handler 参数的类型不匹配。
    debug_assert!(src.kind() == KindA64::X);

    self.place((src.index() as u32) << 5 | (op << 10));
    self.commit();
  }

  pub fn place_btr(
    &mut self,
    name: &str,
    name_inv: &str,
    label: &mut Label,
    op: u8,
    cond: RegisterA64,
    bit: u8,
  ) {
    debug_assert!(
      matches!(cond.kind(), KindA64::X | KindA64::W),
      "cond.kind() == KindA64::x || cond.kind() == KindA64::w"
    );
    debug_assert!(
      bit < cond.kind().gpr_bits() as u8,
      "bit < (cond.kind() == KindA64::x ? 64 : 32)"
    );

    self.place(
      cond.index() as u32
        | (((bit & 0x1f) as u32) << 19)
        | ((op as u32) << 24)
        | (((bit >> 5) as u32) << 31),
    );
    self.commit();

    // cpp CodeGen/src/AssemblyBuilderA64.cpp:1614-1649：
    // FarRefs 开启时 tbz/tbnz 经 patchLabelFar 支持远距离回跳
    let skip_label = self.patch_label_far(label, Kind::IMM14, 24);

    if self.log_text {
      if skip_label.id != 0 {
        self.log_reg_label(name_inv, cond, skip_label, bit as i32);
        self.log_label("b", *label);
        self.log_bind(skip_label);
      } else {
        self.log_reg_label(name, cond, *label, bit as i32);
      }
    }
  }

  pub fn place_cs(
    &mut self,
    name: &str,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
    cond: ConditionA64,
    op: u8,
    opc: u8,
    invert: i32,
  ) {
    if self.log_text {
      self.log_cond(name, dst, src1, src2, cond);
    }

    // 此处避免调用 CODEGEN_ASSERT! 宏：它会触发
    // 代码库其他地方 assert_call_handler 参数的类型不匹配。
    debug_assert!(dst.kind() == src1.kind() && dst.kind() == src2.kind());

    let sf = dst.kind().sf_bit();

    let code_for_condition = [
      0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,
    ];

    let cond_val = code_for_condition[cond as usize] as u32;

    self.place(
      (dst.index() as u32)
        | ((src1.index() as u32) << 5)
        | ((opc as u32) << 10)
        | ((cond_val ^ (invert as u32)) << 12)
        | ((src2.index() as u32) << 16)
        | ((op as u32) << 21)
        | sf,
    );
    self.commit();
  }

  pub fn place_e_r(
    &mut self,
    name: &str,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
    op: u8,
    shift: i32,
  ) {
    if self.log_text {
      self.log_shift(name, dst, src1, src2, shift);
    }

    // CODEGEN_ASSERT! 目前与 ulua_common::assert_call_handler 不兼容
    // （参数类型不匹配：期望 *const i8，宏给 &str）。
    // 改用普通 Rust 断言镜像这些检查。
    assert!(dst.kind() == KindA64::X && src1.kind() == KindA64::X);
    assert!(src2.kind() == KindA64::W);
    assert!((0..=4).contains(&shift));

    let sf = dst.kind().sf_bit();

    let option = 0b010; // UXTW

    self.place(
      (dst.index() as u32)
        | ((src1.index() as u32) << 5)
        | ((shift as u32) << 10)
        | ((option as u32) << 13)
        | ((src2.index() as u32) << 16)
        | (1 << 21)
        | ((op as u32) << 24)
        | sf,
    );
    self.commit();
  }

  pub fn assembly_builder_a_64_place_fcmp(
    &mut self,
    name: &str,
    src1: RegisterA64,
    src2: RegisterA64,
    op: u8,
    opc: u8,
  ) {
    if self.log_text {
      if opc != 0 {
        // C++：log(name, src1, 0) -> reg + "#0" 立即数（shift 默认 0）
        self.log_imm_shift(name, src1, 0, 0);
      } else {
        self.log2(name, src1, src2);
      }
    }

    assert!(src1.kind() == src2.kind());

    self.place(
      ((opc as u32) << 3)
        | ((src1.index() as u32) << 5)
        | ((0b1000u32) << 10)
        | ((src2.index() as u32) << 16)
        | ((op as u32) << 21),
    );
    self.commit();
  }

  pub fn place_fmov(&mut self, name: &str, dst: RegisterA64, src: f64, op: u32) {
    if self.log_text {
      self.log_f64(name, dst, src);
    }

    self.place(dst.index() as u32 | (op << 5));
    self.commit();
  }

  pub fn place_i12(&mut self, name: &str, dst: RegisterA64, src1: RegisterA64, src2: i32, op: u8) {
    if self.log_text {
      self.log_imm(name, dst, src1, src2);
    }

    // 此处避免 CODEGEN_ASSERT!：它当前展开为 ulua_common::assert_call_handler，
    // 后者期望 *const i8 参数，但宏给的是 &str。
    // 同样的逻辑检查改用普通 Rust 断言保留。

    assert!(matches!(dst.kind(), KindA64::W | KindA64::X) || dst == RegisterA64::SP);
    assert!(
      dst.kind() == src1.kind()
        || (dst.kind() == KindA64::X && src1 == RegisterA64::SP)
        || (dst == RegisterA64::SP && src1.kind() == KindA64::X)
    );
    assert!((0..(1 << 12)).contains(&src2));

    let sf = if dst.kind().is_w32() { 0 } else { K_SF64 };

    self.place(
      (dst.index() as u32)
        | ((src1.index() as u32) << 5)
        | ((src2 as u32) << 10)
        | ((op as u32) << 24)
        | sf,
    );
    self.commit();
  }

  pub fn place_i16(&mut self, name: &str, dst: RegisterA64, src: i32, op: u8, shift: i32) {
    if self.log_text {
      self.log_imm_shift(name, dst, src, shift);
    }

    // 此处避免 CODEGEN_ASSERT!：它展开为 ulua_common::assert_call_handler，
    // 后者期望 *const i8 参数，但宏给的是 &str。
    assert!(matches!(dst.kind(), KindA64::W | KindA64::X));
    assert!((0..=0xffff).contains(&src));
    assert!(matches!(shift, 0 | 16 | 32 | 48));

    let sf = dst.kind().sf_bit();

    self.place(
      (dst.index() as u32)
        | ((src as u32) << 5)
        | (((shift >> 4) as u32) << 21)
        | ((op as u32) << 23)
        | sf,
    );
    self.commit();
  }

  pub fn place_p(
    &mut self,
    name: &str,
    src1: RegisterA64,
    src2: RegisterA64,
    dst: AddressA64,
    op: u8,
    opc: u8,
    sizelog: i32,
  ) {
    if self.log_text {
      self.log2_addr(name, src1, src2, dst);
    }

    assert!(dst.kind == AddressKindA64::Imm);
    assert!(dst.data >= (-128 * (1i32 << sizelog)) && dst.data <= (127 * (1i32 << sizelog)));
    assert!(dst.data % (1i32 << sizelog) == 0);

    self.place(
      (src1.index() as u32)
        | ((dst.base.index() as u32) << 5)
        | ((src2.index() as u32) << 10)
        | (((dst.data >> sizelog) as u32 & 127) << 15)
        | ((op as u32) << 22)
        | ((opc as u32) << 30),
    );
    self.commit();
  }

  pub fn place_r_1(&mut self, name: &str, dst: RegisterA64, src: RegisterA64, op: u32) {
    if self.log_text {
      self.log2(name, dst, src);
    }

    // 两源任一为 64 位 GPR 即置 sf；按位或与本行原 if/else 逐位等价。
    let sf: u32 = dst.kind().sf_bit() | src.kind().sf_bit();

    self.place(dst.index() as u32 | ((src.index() as u32) << 5) | (op << 10) | sf);
    self.commit();
  }

  pub fn place_r_3(
    &mut self,
    name: &str,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
    op: u8,
    op2: u8,
  ) {
    if self.log_text {
      self.log_shift(name, dst, src1, src2, 0);
    }

    // 此处避免 CODEGEN_ASSERT! 宏：它目前展开为 ulua_common::assert_call_handler，
    // 后者期望 *const i8 参数，其他调用点可能类型不匹配。
    assert!(matches!(
      dst.kind(),
      KindA64::W | KindA64::X | KindA64::D | KindA64::S
    ));
    assert!(dst.kind() == src1.kind() && dst.kind() == src2.kind());

    let sf = dst.kind().sf_bit();

    self.place(
      (dst.index() as u32)
        | ((src1.index() as u32) << 5)
        | ((op2 as u32) << 10)
        | ((src2.index() as u32) << 16)
        | ((op as u32) << 21)
        | sf,
    );
    self.commit();
  }

  pub fn place_sr_2(&mut self, name: &str, dst: RegisterA64, src: RegisterA64, op: u8, op2: u8) {
    if self.log_text {
      self.log2(name, dst, src);
    }

    // 此处避免调用 CODEGEN_ASSERT! 宏：它会触发
    // 代码库其他地方 assert_call_handler 参数的类型不匹配。
    debug_assert!(matches!(dst.kind(), KindA64::W | KindA64::X));
    debug_assert!(dst.kind() == src.kind());

    let sf = dst.kind().sf_bit();

    self.place(
      dst.index() as u32
        | (0x1f << 5)
        | (src.index() as u32) << 16
        | (op2 as u32) << 21
        | (op as u32) << 24
        | sf,
    );
    self.commit();
  }

  pub fn place_sr_3(
    &mut self,
    name: &str,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
    op: u8,
    shift: i32,
    n: i32,
  ) {
    if self.log_text {
      self.log_shift(name, dst, src1, src2, shift);
    }

    // 此处避免调用 CODEGEN_ASSERT! 宏：它会触发
    // 代码库其他地方 assert_call_handler 参数的类型不匹配。
    debug_assert!(matches!(dst.kind(), KindA64::W | KindA64::X));
    debug_assert!(dst.kind() == src1.kind() && dst.kind() == src2.kind());
    debug_assert!((-63..=63).contains(&shift));

    let sf = dst.kind().sf_bit();

    let shift_abs = if shift < 0 { -shift } else { shift };

    self.place(
      dst.index() as u32
        | (src1.index() as u32) << 5
        | (shift_abs as u32) << 10
        | (src2.index() as u32) << 16
        | ((n as u32) << 21)
        | ((shift < 0) as u32) << 22
        | (op as u32) << 24
        | sf,
    );
    self.commit();
  }

  pub fn place_vr(
    &mut self,
    name: &str,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
    op: u16,
    op2: u8,
  ) {
    if self.log_text {
      self.log_append(format_args!(
        " {:<12}v{}.4s,v{}.4s,v{}.4s\n",
        name,
        dst.index(),
        src1.index(),
        src2.index()
      ));
    }

    // 此处避免 CODEGEN_ASSERT! 宏：它经 ulua_common::assert_call_handler 展开，
    // 目前会在本翻译集中造成类型不匹配。
    debug_assert!(dst.kind() == KindA64::Q);
    debug_assert!(dst.kind() == src1.kind() && dst.kind() == src2.kind());

    self.place(
      (dst.index() as u32)
        | ((src1.index() as u32) << 5)
        | ((op2 as u32) << 10)
        | ((src2.index() as u32) << 16)
        | ((op as u32) << 21)
        | (1u32 << 30),
    );
    self.commit();
  }

  pub fn rbit(&mut self, dst: RegisterA64, src: RegisterA64) {
    debug_assert!(matches!(dst.kind(), KindA64::W | KindA64::X));
    debug_assert!(dst.kind() == src.kind());

    self.place_r_1("rbit", dst, src, 0b1_0110_1011_0000_0000_0000);
  }

  pub fn rem(&mut self, dst: RegisterA64, src1: RegisterA64, src2: RegisterA64) {
    // dst 必须持有紧邻的 sdiv/udiv 的商。
    // dst != src1，因为 mul 会在 sub 读 src1 之前破坏 dst。
    assert!(dst.index() != src1.index());

    // dst = src1 - (dst * src2);
    self.msub(dst, dst, src2, src1);
  }

  pub fn ret(&mut self) {
    self.place_0("ret", 0b1101_0110_0101_1111_0000_0011_1100_0000);
  }

  pub fn rev(&mut self, dst: RegisterA64, src: RegisterA64) {
    CODEGEN_ASSERT!(matches!(dst.kind(), KindA64::W | KindA64::X));
    CODEGEN_ASSERT!(dst.kind() == src.kind());

    self.place_r_1(
      "rev",
      dst,
      src,
      0b1_0110_1011_0000_0000_0010 | (dst.kind() == KindA64::X) as u32,
    );
  }

  pub fn ror_register_a_64_register_a_64_register_a_64(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
  ) {
    self.place_r_3("ror", dst, src1, src2, 0b11010110, 0b001011);
  }

  pub fn ror_register_a_64_register_a_64_u8(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: u8,
  ) {
    let size = dst.kind().gpr_bits();

    debug_assert!(src2 < size as u8);

    self.place_bfm(
      "ror",
      dst,
      src1,
      src2 as i32,
      0b00_100111,
      src1.index() as i32,
      src2 as i32,
    );
  }

  pub fn sbfiz(&mut self, dst: RegisterA64, src: RegisterA64, f: u8, w: u8) {
    let size = dst.kind().gpr_bits();

    CODEGEN_ASSERT!(w > 0 && (f as i32) + (w as i32) <= size);

    self.place_bfm(
      "sbfiz",
      dst,
      src,
      (f as i32) * 100 + (w as i32),
      0b00_100110,
      (-(f as i32)) & (size - 1),
      (w as i32) - 1,
    );
  }

  pub fn sbfx_register_a_64_register_a_64_u8_u8(
    &mut self,
    dst: RegisterA64,
    src: RegisterA64,
    f: u8,
    w: u8,
  ) {
    let size = dst.kind().gpr_bits() as u32;

    CODEGEN_ASSERT!(w > 0 && (f as u32) + (w as u32) <= size);

    // f * 100 + w 只用于反汇编打印；将来或许可拆成两个字段以提高可读性
    self.place_bfm(
      "sbfx",
      dst,
      src,
      (f as i32) * 100 + w as i32,
      0b00_100110,
      f as i32,
      f as i32 + w as i32 - 1,
    );
  }

  pub fn scvtf(&mut self, dst: RegisterA64, src: RegisterA64) {
    debug_assert!(dst.kind() == KindA64::D);
    debug_assert!(matches!(src.kind(), KindA64::W | KindA64::X));

    self.place_r_1("scvtf", dst, src, 0b00_0111_1001_1000_1000_0000);
  }

  pub fn set_label(&mut self) -> Label {
    let label = Label {
      id: self.next_label,
      location: self.get_code_size(),
    };
    self.next_label = self.next_label.wrapping_add(1);
    self.label_locations.push(!0u32);

    if self.log_text {
      self.log_bind(label);
    }

    label
  }

  pub fn set_label_label(&mut self, label: &mut Label) {
    if label.id == 0 {
      label.id = self.next_label;
      self.next_label = self.next_label.wrapping_add(1);
      self.label_locations.push(!0u32);
    }

    label.location = self.get_code_size();
    self.label_locations[(label.id - 1) as usize] = label.location;

    if self.log_text {
      self.log_bind(*label);
    }
  }

  pub fn stp(&mut self, src1: RegisterA64, src2: RegisterA64, dst: AddressA64) {
    debug_assert!(matches!(src1.kind(), KindA64::X | KindA64::W));
    debug_assert!(src1.kind() == src2.kind());

    let is_x = src1.kind() == KindA64::X;
    self.place_p(
      "stp",
      src1,
      src2,
      dst,
      0b1010_0100,
      (is_x as u8) << 1,
      if is_x { 3 } else { 2 },
    );
  }

  pub fn str(&mut self, src: RegisterA64, dst: AddressA64) {
    assert!(matches!(
      src.kind(),
      KindA64::X | KindA64::W | KindA64::S | KindA64::D | KindA64::Q
    ));

    match src.kind() {
      KindA64::W => self.place_a("str", src, dst, 0b10_11100000, 2),
      KindA64::X => self.place_a("str", src, dst, 0b11_11100000, 3),
      KindA64::S => self.place_a("str", src, dst, 0b10_11110000, 2),
      KindA64::D => self.place_a("str", src, dst, 0b11_11110000, 3),
      KindA64::Q => self.place_a("str", src, dst, 0b00_11110010, 4),
      KindA64::None => unreachable!("Unexpected register kind"),
    }
  }

  pub fn strb(&mut self, src: RegisterA64, dst: AddressA64) {
    debug_assert!(src.kind() == KindA64::W);

    self.place_a("strb", src, dst, 0b00_1110_0000, 0);
  }

  pub fn strh(&mut self, src: RegisterA64, dst: AddressA64) {
    debug_assert!(src.kind() == KindA64::W);

    self.place_a("strh", src, dst, 0b01_11100000, 1);
  }

  pub fn sub_register_a_64_register_a_64_register_a_64_i32(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: RegisterA64,
    shift: i32,
  ) {
    if src1.kind() == KindA64::X && src2.kind() == KindA64::W {
      self.place_e_r("sub", dst, src1, src2, 0b10_01011, shift);
    } else {
      self.place_sr_3("sub", dst, src1, src2, 0b10_01011, shift, 0);
    }
  }

  pub fn sub_register_a_64_register_a_64_u16(
    &mut self,
    dst: RegisterA64,
    src1: RegisterA64,
    src2: u16,
  ) {
    self.place_i12("sub", dst, src1, src2 as i32, 0b10_10001);
  }

  pub fn tbnz(&mut self, src: RegisterA64, bit: u8, label: &mut Label) {
    self.place_btr("tbnz", "tbz", label, 0b0110111, src, bit);
  }

  pub fn tbz(&mut self, src: RegisterA64, bit: u8, label: &mut Label) {
    self.place_btr("tbz", "tbnz", label, 0b0110110, src, bit);
  }

  pub fn tst_register_a_64_register_a_64_i32(
    &mut self,
    src1: RegisterA64,
    src2: RegisterA64,
    shift: i32,
  ) {
    let dst = src1.zero_reg();

    self.place_sr_3("tst", dst, src1, src2, 0b11_01010, shift, 0);
  }

  pub fn tst_register_a_64_u32(&mut self, src1: RegisterA64, src2: u32) {
    let dst = src1.zero_reg();

    self.place_bm("tst", dst, src1, src2, 0b11_100100);
  }

  pub fn ubfiz(&mut self, dst: RegisterA64, src: RegisterA64, f: u8, w: u8) {
    let size = dst.kind().gpr_bits();

    CODEGEN_ASSERT!(w > 0 && (f as i32 + w as i32) <= size);

    self.place_bfm(
      "ubfiz",
      dst,
      src,
      (f as i32) * 100 + (w as i32),
      0b10_100110,
      (-(f as i32)) & (size - 1),
      (w as i32) - 1,
    );
  }

  pub fn ubfx(&mut self, dst: RegisterA64, src: RegisterA64, f: u8, w: u8) {
    let size = dst.kind().gpr_bits();

    CODEGEN_ASSERT!(w > 0 && f as i32 + w as i32 <= size);

    // f * 100 + w 只用于反汇编打印；将来或许可拆成两个字段以提高可读性
    self.place_bfm(
      "ubfx",
      dst,
      src,
      (f as i32) * 100 + (w as i32),
      0b10_100110,
      f as i32,
      f as i32 + w as i32 - 1,
    );
  }

  pub fn ucvtf(&mut self, dst: RegisterA64, src: RegisterA64) {
    // CODEGEN_ASSERT! 目前展开为 ulua_common::assert_call_handler(...)，
    // 它期望裸指针。这里避免调用它。
    debug_assert!(matches!(dst.kind(), KindA64::D | KindA64::S));
    debug_assert!(matches!(src.kind(), KindA64::W | KindA64::X));

    if dst.kind() == KindA64::D {
      self.place_r_1("ucvtf", dst, src, 0b00_0111_1001_1000_1100_0000);
    } else {
      self.place_r_1("ucvtf", dst, src, 0b00_0111_1000_1000_1100_0000);
    }
  }

  pub fn udf(&mut self) {
    self.place_0("udf", 0);
  }

  pub fn umov_4s(&mut self, dst: RegisterA64, src: RegisterA64, index: u8) {
    CODEGEN_ASSERT!(dst.kind() == KindA64::W);
    CODEGEN_ASSERT!(src.kind() == KindA64::Q);
    CODEGEN_ASSERT!(index < 4);

    if self.log_text {
      self.log_append(format_args!(
        " {:<12}w{},v{}.s[{}]\n",
        "umov",
        dst.index(),
        src.index(),
        index
      ));
    }

    let op: u32 = 0b0000_1110_0000_0100_0011_1100_0000_0000;

    self.place(dst.index() as u32 | (src.index() as u32) << 5 | op | (index as u32) << 19);

    self.commit();
  }
}

/// C-ABI 诊断入参（review.md §10 常量形态：`&[u8]` NUL 结尾字节串，收口点
/// 仅 `.as_ptr().cast()`，不外泄 C 字符串类型）
const K_ASSERT_INVARIANT: &[u8] = b"codePos < codeEnd\0";

const K_ASSERT_FILE: &[u8] = b"CodeGen/src/AssemblyBuilderA64.cpp\0";

const K_ASSERT_FUNCTION: &[u8] = b"void Luau::CodeGen::AssemblyBuilderA64::place(uint32_t)\0";
