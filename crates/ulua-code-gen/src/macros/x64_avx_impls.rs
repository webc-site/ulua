/// x64 AVX 助记符表坍缩骨架（对照 cpp：`AssemblyBuilderX64.cpp` 各 `placeAvx(...)`
/// 单行转发体）。
///
/// 三个形状宏分别覆盖 cpp `placeAvx` 的三个重载：
/// - `x64_avx_rrm_impls!`：三操作数 `(dst, src1, src2)` 形态（code/w/mode/prefix 全
///   字面量，无前置断言、无运行期分支）；
/// - `x64_avx_rm_impls!`：双操作数 `(a, b)` 无 `coderev` 形态；
/// - `x64_avx_rm_rev_impls!`：双操作数带 `coderev`（load/store 反向码）形态。
///
/// 生成的方法体一律是 `self.place_avxN(<mnem>, 形参..., <code>, <w>, <mode>,
/// <prefix>)`，实参次序与逐字面量与原手写展开体一一对应，发射字节完全不变。
/// 每个宏同时导出对应 `*_ROWS` 常量表，测试（`tests/x64_avx_table_matches_cpp.rs`）
/// 用它与 cpp `AssemblyBuilderX64.cpp` 各调用点逐格对账（abs-r127 定表自检路线）。
///
/// 不纳入本宏的臂（差异实质性，各自文件保留手写）：
/// - `vcvtsd2ss / vcvtss2sd / vcvtsi2sd / vcvtsi2ss / vcvttsd2si`：w 位由操作数
///   尺寸运行期判定，且 vcvt 系带 cpp 前置断言；
/// - `vroundss / vroundsd / vroundps`：code 位由 `RoundingModeX64` 运行期合成；
/// - `vmovq`：按 dst 尺寸二臂分派；
/// - `vblendvpd / vblendvps / vdpps / vpextrd / vpinsrd / vpshufps / vcmpeqsd /
///   vcmpltsd`：走 `place_avx_imm8` 重载（imm8 在 modrm 之后发射，形状不同）。
#[macro_export]
macro_rules! x64_avx_rrm_impls {
  ( $( $meth:ident => ( $mnem:literal, $code:expr, $w:expr, $mode:expr, $prefix:expr ); )+ ) => {
    /// 三操作数臂的编码行（mnem, code, w, mode, prefix），供 cpp 对账测试。
    pub const X64_AVX_RRM_ROWS: &[(&str, u8, bool, u8, u8)] = &[
      $( ($mnem, $code, $w, $mode, $prefix), )+
    ];

    impl $crate::records::assembly_builder_x_64::AssemblyBuilderX64 {
      $(
        #[inline]
        pub fn $meth(
          &mut self,
          dst: $crate::records::operand_x_64::OperandX64,
          src1: $crate::records::operand_x_64::OperandX64,
          src2: $crate::records::operand_x_64::OperandX64,
        ) {
          self.place_avx3($mnem, dst, src1, src2, $code, $w, $mode, $prefix);
        }
      )+
    }
  };
}

#[macro_export]
macro_rules! x64_avx_rm_impls {
  ( $( $meth:ident ( $a:ident, $b:ident ) => ( $mnem:literal, $code:expr, $w:expr, $mode:expr, $prefix:expr ); )+ ) => {
    /// 双操作数无 coderev 臂的编码行（mnem, code, w, mode, prefix）。
    pub const X64_AVX_RM_ROWS: &[(&str, u8, bool, u8, u8)] = &[
      $( ($mnem, $code, $w, $mode, $prefix), )+
    ];

    impl $crate::records::assembly_builder_x_64::AssemblyBuilderX64 {
      $(
        #[inline]
        pub fn $meth(
          &mut self,
          $a: $crate::records::operand_x_64::OperandX64,
          $b: $crate::records::operand_x_64::OperandX64,
        ) {
          self.place_avx2($mnem, $a, $b, $code, $w, $mode, $prefix);
        }
      )+
    }
  };
}

#[macro_export]
macro_rules! x64_avx_rm_rev_impls {
  ( $( $meth:ident ( $a:ident, $b:ident ) => ( $mnem:literal, $code:expr, $coderev:expr, $w:expr, $mode:expr, $prefix:expr ); )+ ) => {
    /// 双操作数带 coderev 臂的编码行（mnem, code, coderev, w, mode, prefix）。
    pub const X64_AVX_RM_REV_ROWS: &[(&str, u8, u8, bool, u8, u8)] = &[
      $( ($mnem, $code, $coderev, $w, $mode, $prefix), )+
    ];

    impl $crate::records::assembly_builder_x_64::AssemblyBuilderX64 {
      $(
        #[inline]
        pub fn $meth(
          &mut self,
          $a: $crate::records::operand_x_64::OperandX64,
          $b: $crate::records::operand_x_64::OperandX64,
        ) {
          self.place_avx2_rev($mnem, $a, $b, $code, $coderev, $w, $mode, $prefix);
        }
      )+
    }
  };
}

pub use x64_avx_rm_impls;
pub use x64_avx_rm_rev_impls;
pub use x64_avx_rrm_impls;
