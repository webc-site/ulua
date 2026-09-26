//! 指令/助记符 impl 表坍缩宏族（宏体与对账表逐字保真；r7-macros98 合并票）。

/// A64 操作数 impl 簇坍缩骨架（对照 cpp：`IrLoweringA64.cpp` 中 `MovArg`/`AddSubArg`/
/// `CmpArg` 的立即数分支）。
///
/// 这些 trait 的立即数臂互为镜像：函数体一律是
/// `build.<builder_meth>(<build 之后的形参>, self as <Cast>)`，仅差三处——
/// - `for` 的源类型（`u32`/`u16`/`u8`/`i32` …），
/// - `as` 的目标类型（同一 impl 内所有方法共用一个 `Cast`），
/// - builder 方法名的 mnemonic（`mov_*`/`add_*`/`sub_*`/`cmp_*`，随方法名而定）。
///
/// 每个顶层条目描述一枚 `impl`：`Trait for Type as Cast { 方法: (形参) => builder; }`，
/// 宏按「trait 名 × 目标类型 × builder 方法名 × cast 类型」四元参数生成 `impl` 块，
/// 生成的调用与原手写体逐字同形（同一 builder 方法、同一实参次序、同一 `as` 截断），
/// 故发射的指令序列与编码字节完全不变。
///
/// 不纳入本宏的臂（差异实质性，各文件保留手写）：
/// - `RegisterA64` 寄存器臂：走 `*_register_a_64_register_a_64*` 另一条 builder，
///   实参含移位量尾参，非「立即数镜像」；
/// - 「无 cast」的源类型臂（如 `MovArg for i32` 直接 `self`、`AddSubArg/CmpArg for u16`
///   直接 `self`）：`self as 同型` 会触发 `unnecessary_cast`，故不并入 cast 簇；
/// - `LogicArg`/`ShiftArg` 的 `i32` 臂是 `(self as uN).<meth>(...)` 转发到兄弟 impl、
///   且其整型直接臂各自唯一（无 ≥2 镜像簇），整体保留手写。
#[macro_export]
macro_rules! a64_arg_impls {
  (
    $(
      $Trait:ident for $Ty:ty as $Cast:ty {
        $( $meth:ident : ($($arg:ident : $argty:ty),*) => $builder:ident ; )+
      }
    )+
  ) => {
    $(
      impl $Trait for $Ty {
        $(
          fn $meth(
            self,
            build: &mut $crate::records::assembly_builder_a_64::AssemblyBuilderA64,
            $($arg : $argty),*
          ) {
            build.$builder($($arg,)* self as $Cast);
          }
        )+
      }
    )+
  };
}

// A64 `placeR1` 单源浮点臂坍缩骨架（对照 cpp `AssemblyBuilderA64.cpp` 各
// `placeR1(name, dst, src, op)` 三点段）。
//
// 这些指令的 Q/D/S 三臂互为镜像：仅差助记符字面量与三枚 22 位 op 常量，
// 体形一律为「两条 debug_assert + Q→D→S 级联 place_r_1」，生成的调用与
// 原手写体逐字同形（同一 place_r_1、同一实参次序、同一 op 位串），发射
// 编码与日志文本完全不变。
//
// 每次调用同时生成 `A64_R1_ROWS` 对账表（行 token 与生成体同源），由
// tests/a64_r1_table_matches_cpp.rs 与 cpp 逐格核对。
//
// 不纳入本宏的臂（差异实质性，保留手写）：
// - `fneg`：每臂各自 debug_assert 且 D→S→Q 次序不同（cpp :869-880）；
// - `fsqrt`：仅 D/S 两臂（cpp :1837-1846），无 Q 臂级联形态。
#[macro_export]
macro_rules! a64_r1_impls {
  ($( $meth:ident => ( $mnem:literal, $q:expr, $d:expr, $s:expr ); )+) => {
    /// 助记符 × Q/D/S 三臂 op 对账表（与下方生成体同源）。
    pub const A64_R1_ROWS: &[(&str, u32, u32, u32)] = &[ $( ($mnem, $q, $d, $s), )+ ];

    $(
      impl $crate::records::assembly_builder_a_64::AssemblyBuilderA64 {
        pub fn $meth(
          &mut self,
          dst: $crate::records::register_a_64::RegisterA64,
          src: $crate::records::register_a_64::RegisterA64,
        ) {
          use $crate::enums::kind_a_64::KindA64;

          debug_assert!(dst.kind() == src.kind());
          debug_assert!(matches!(dst.kind(), KindA64::D | KindA64::S | KindA64::Q));

          if dst.kind() == KindA64::Q {
            self.place_r_1($mnem, dst, src, $q);
          } else if dst.kind() == KindA64::D {
            self.place_r_1($mnem, dst, src, $d);
          } else {
            self.place_r_1($mnem, dst, src, $s);
          }
        }
      }
    )+
  };
}

// builtin 公共骨架坍缩宏（对照 cpp `IrTranslateBuiltins.cpp` 顶部的 static 助手段）。
//
// 三组函数各自体内仅差字面量（cmd 名 / 常量种类 / Lua 类型标签），逐位保持语义：
// - `builtin_load_impls!`：常量直通 + 单 op `Load*`（cpp `builtinLoadDouble` :38-44、
//   `builtinLoadInt64` :54-60）；
// - `builtin_check_impls!`：常量臂断言种类 + 寄存器臂按标签校验（cpp `builtinCheckDouble`
//   :30-36、`builtinCheckInt64` :46-52；比对方式与 cpp 一致走 `constOp(arg).kind`）；
// - `builtin_store_impls!`：结果写回 ra 并无条件补打标签（cpp 各翻译器尾部
//   `STORE_*, vmReg(ra)` + `STORE_TAG, constTag(..)` 成对出现，如 :431-432/:1524-1525）。
//
// 每次调用同时生成一张 `*_ROWS` 对账表（行 token 与生成体同源，构造即一致），
// 由 tests/builtin_linearop_table_matches_cpp.rs 与 cpp 逐格核对。

/// load 臂：常量直接返回，否则发一条单 op 装载指令。
#[macro_export]
macro_rules! builtin_load_impls {
  ($( $meth:ident => ( $cmd:expr ); )+) => {
    /// 方法名 × IR cmd 对账表（与下方生成体同源）。
    pub const BUILTIN_LOAD_ROWS: &[(&str, $crate::enums::ir_cmd::IrCmd)] =
      &[ $( (stringify!($meth), $cmd), )+ ];

    $(
      #[inline]
      pub(crate) fn $meth(
        build: &mut $crate::records::ir_builder::IrBuilder,
        arg: $crate::records::ir_op::IrOp,
      ) -> $crate::records::ir_op::IrOp {
        if arg.kind() == $crate::enums::ir_op_kind::IrOpKind::Constant {
          return arg;
        }

        build.inst_ir_cmd_ir_op($cmd, arg)
      }
    )+
  };
}

/// check 臂：常量臂 CODEGEN_ASSERT 常量种类，寄存器臂 vmExit 兜底 + 按标签校验。
#[macro_export]
macro_rules! builtin_check_impls {
  ($( $meth:ident => ( $kind:expr, $tag:expr ); )+) => {
    /// 方法名 × 期望常量种类 × Lua 标签对账表（与下方生成体同源）。
    pub const BUILTIN_CHECK_ROWS: &[(&str, $crate::enums::ir_const_kind::IrConstKind, ulua_vm::enums::lua_type::LuaType)] =
      &[ $( (stringify!($meth), $kind, $tag), )+ ];

    $(
      #[inline]
      pub(crate) fn $meth(
        build: &mut $crate::records::ir_builder::IrBuilder,
        arg: $crate::records::ir_op::IrOp,
        pcpos: i32,
      ) {
        if arg.kind() == $crate::enums::ir_op_kind::IrOpKind::Constant {
          $crate::macros::codegen_assert::CODEGEN_ASSERT!(
            build.function.const_op(arg).kind() == $kind
          );
        } else {
          let exit = build.vm_exit(pcpos as u32);
          build.load_and_check_tag(arg, $tag as u8, exit);
        }
      }
    )+
  };
}

/// store 尾：结果按 cmd 写回 ra 寄存器，并无条件补写标签。
#[macro_export]
macro_rules! builtin_store_impls {
  ($( $meth:ident => ( $cmd:expr, $tag:expr ); )+) => {
    /// 方法名 × store cmd × Lua 标签对账表（与下方生成体同源）。
    pub const BUILTIN_STORE_ROWS: &[(&str, $crate::enums::ir_cmd::IrCmd, ulua_vm::enums::lua_type::LuaType)] =
      &[ $( (stringify!($meth), $cmd, $tag), )+ ];

    $(
      #[inline]
      pub(crate) fn $meth(
        build: &mut $crate::records::ir_builder::IrBuilder,
        ra: i32,
        value: $crate::records::ir_op::IrOp,
      ) {
        let ra_reg = build.vm_reg(ra as u8);
        build.inst_ir_cmd_ir_op_ir_op($cmd, ra_reg, value);
        build.store_tag(ra_reg, $tag as u8);
      }
    )+
  };
}

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
