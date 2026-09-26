//! x64 AVX 助记符坍缩表（abs-r117 `a64_arg_impls` 路线的 x64 对偶）：cpp
//! `AssemblyBuilderX64.cpp` 中数十个「仅 mnemonic/code/w/mode/prefix 字面量不同、
//! 全部单行转发 `placeAvx`」的同构方法，收进 `x64_avx_rrm_impls!` /
//! `x64_avx_rm_impls!` / `x64_avx_rm_rev_impls!` 三张宏表。
//!
//! 表行同时以 `X64_AVX_*_ROWS` 常量导出，由
//! `tests/x64_avx_table_matches_cpp.rs` 与 cpp 各调用点逐格对账。
//!
//! 行内字面量逐字取自坍缩前各手写文件（含 cpp 窄 VEX 字段写法 `0b0001/0b11/0b10`
//! 与裸 x86 前缀字节写法的混用——两者经 `place_vex` 归一化后等价，故保留原拼写
//! 即保留原语义，不做「顺手统一」）。

// 三操作数 `(dst, src1, src2)` 形态：cpp 行号见各条目注释。
crate::x64_avx_rrm_impls! {
  // cpp AssemblyBuilderX64.cpp:745/750/755/760
  vaddpd => ("vaddpd", 0x58, false, 0x0F, 0x66);
  vaddps => ("vaddps", 0x58, false, 0x0F, 0x00);
  vaddsd => ("vaddsd", 0x58, false, 0x0F, 0xF2);
  vaddss => ("vaddss", 0x58, false, 0x0F, 0xF3);
  // cpp :820/815/810
  vandnpd => ("vandnpd", 0x55, false, 0x0F, 0x66);
  vandpd => ("vandpd", 0x54, false, 0x0F, 0x66);
  vandps => ("vandps", 0x54, false, 0x0F, 0x00);
  // 疑似与 cpp 偏差（cpp:1039/1034 走 imm8 重载：code=0xc2、imm8=0x00/0x01、
  // mode=AVX_0F、prefix=AVX_NP/AVX_F3），本表逐字保留坍缩前 Rust 字面量，不修正。
  vcmpeqps => ("vcmpeqps", 0x00, false, 0xc2, 0);
  vcmpltss => ("vcmpltss", 0x01, false, 0xc2, 0x0f);
  // cpp :805/795/800
  vdivps => ("vdivps", 0x5e, false, 0x0F, 0);
  vdivsd => ("vdivsd", 0x5e, false, 0x0f, 0xf2);
  vdivss => ("vdivss", 0x5e, false, 0x0F, 0xF3);
  // 疑似与 cpp 偏差（cpp:1090/1085：mode=AVX_0F38、prefix=AVX_66），保留原字面量。
  vfmadd213pd => ("vfmadd213pd", 0xA8, true, 0x0F, 0x38);
  vfmadd213ps => ("vfmadd213ps", 0xA8, false, 0x0F, 0x38);
  // cpp :994/999/1004（vmaxss 原文件写 `0b0000_1111u8/0b1111_0011u8`，同值）
  vmaxps => ("vmaxps", 0x5f, false, 0x0F, 0);
  vmaxsd => ("vmaxsd", 0x5f, false, 0x0f, 0xf2);
  vmaxss => ("vmaxss", 0x5f, false, 0x0F, 0xF3);
  // cpp :1009/1014/1019
  vminps => ("vminps", 0x5d, false, 0x0F, 0x00);
  vminsd => ("vminsd", 0x5d, false, 0x0f, 0xf2);
  vminss => ("vminss", 0x5d, false, 0x0f, 0xf3);
  // cpp :790/780/785
  vmulps => ("vmulps", 0x59, false, 0x0F, 0);
  vmulsd => ("vmulsd", 0x59, false, 0x0f, 0xf2);
  vmulss => ("vmulss", 0x59, false, 0x0F, 0xF3);
  // cpp :939/:949（vmovsd/vmovss 三参重载；窄 VEX 字段拼写按原文件保留。
  // Rust 无重载，方法名沿用抽取器生成的后缀名）
  vmovsd_operand_x_64_operand_x_64_operand_x_64 => ("vmovsd", 0x10, false, 0b0001, 0b11);
  vmovss_operand_x_64_operand_x_64_operand_x_64 => ("vmovss", 0x10, false, 0b0001, 0b10);
  // cpp :840/835（vorpd 原文件写 `0b0000_1111/0b0110_0110`，同值）
  vorpd => ("vorpd", 0x56, false, 0x0F, 0x66);
  vorps => ("vorps", 0x56, false, 0x0F, 0x00);
  // cpp :775/765/770
  vsubps => ("vsubps", 0x5c, false, 0x0f, 0x00);
  vsubsd => ("vsubsd", 0x5c, false, 0x0f, 0xf2);
  vsubss => ("vsubss", 0x5c, false, 0x0f, 0xf3);
  // cpp :924/929
  vsqrtsd => ("vsqrtsd", 0x51, false, 0x0F, 0xF2);
  vsqrtss => ("vsqrtss", 0x51, false, 0x0F, 0xF3);
  // cpp :830/825
  vxorpd => ("vxorpd", 0x57, false, 0x0F, 0x66);
  vxorps => ("vxorps", 0x57, false, 0x0F, 0);
}

// 双操作数无 coderev 形态；`vucomisd/vucomiss` 无目的寄存器（VEX.vvvv=1111），
// 形参名保留 cpp `AssemblyBuilderX64.cpp:843/848` 的 `(src1, src2)`。
crate::x64_avx_rm_impls! {
  // cpp :914/919（vsqrtps 原文件 mode 用窄拼写 0x01==AVX_0F，同值）
  vsqrtpd(dst, src) => ("vsqrtpd", 0x51, false, 0x0F, 0x66);
  vsqrtps(dst, src) => ("vsqrtps", 0x51, false, 0x01, 0x00);
  // cpp :845/850
  vucomisd(src1, src2) => ("vucomisd", 0x2e, false, 0x0F, 0x66);
  vucomiss(src1, src2) => ("vucomiss", 0x2e, false, 0x0F, 0x00);
}

// 双操作数带 coderev（load/store 反向码）形态。
crate::x64_avx_rm_rev_impls! {
  // cpp :954/959/964/969/934/944
  vmovapd(dst, src) => ("vmovapd", 0x28, 0x29, false, 0x0F, 0x66);
  vmovaps(dst, src) => ("vmovaps", 0x28, 0x29, false, 0x0F, 0x00);
  vmovupd(dst, src) => ("vmovupd", 0x10, 0x11, false, 0x0F, 0x66);
  vmovups(dst, src) => ("vmovups", 0x10, 0x11, false, 0x0F, 0x00);
  vmovsd_operand_x_64_operand_x_64(dst, src) => ("vmovsd", 0x10, 0x11, false, 0b0001, 0b11);
  vmovss_operand_x_64_operand_x_64(dst, src) => ("vmovss", 0x10, 0x11, false, 0b0001, 0b10);
}
