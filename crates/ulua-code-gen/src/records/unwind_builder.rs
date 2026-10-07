use crate::enums::options::Arch;
#[cfg(not(target_os = "windows"))]
use crate::records::unwind_builder_dwarf_2::UnwindBuilderDwarf2;
#[cfg(target_os = "windows")]
use crate::records::unwind_builder_win::UnwindBuilderWin;

/// cpp 抽象基类 `UnwindBuilder` 的移植残留标记：仅保留对外暴露的平台常量
/// （`X64`/`A64`）。真实实现不再经虚表分派，而是按平台静态选取
/// [`UnwindBuilderImpl`]，故本类型是无状态的零大小标记（无 vtable 槽、无继承）。
#[derive(Debug, Clone, Copy, Default)]
pub struct UnwindBuilder;

impl UnwindBuilder {
  pub const X64: Arch = Arch::X64;
  pub const A64: Arch = Arch::A64;
}

/// 当前平台的具体 unwind 构建器类型：替代 cpp 的基类指针 + 虚函数下行转换。
#[cfg(target_os = "windows")]
pub type UnwindBuilderImpl = UnwindBuilderWin;

#[cfg(not(target_os = "windows"))]
pub type UnwindBuilderImpl = UnwindBuilderDwarf2;
