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

pub use a64_arg_impls;
