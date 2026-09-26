use core::ptr::from_mut;

/// 平台侧具体 unwind 实现类型：`UnwindBuilder` 与实现共享 `repr(C)` 前缀布局
/// （cpp 基类指针转换的移植形态），上下文字段中的基类指针恒指向本平台真实实现对象。
#[cfg(not(target_os = "windows"))]
use crate::records::unwind_builder_dwarf_2::UnwindBuilderDwarf2 as UnwindBuilderImpl;
#[cfg(target_os = "windows")]
use crate::records::unwind_builder_win::UnwindBuilderWin as UnwindBuilderImpl;
use crate::{enums::arch::Arch, records::unwind_builder::UnwindBuilder};

/// 基类可变引用 → 具体实现可变引用：全 crate 唯一的 unwind 构建器向下转型写法。
///
/// # Safety
/// `unwind` 指向的对象必须是当前平台的具体 unwind 实现对象（上下文构造点保证）。
pub(crate) unsafe fn as_impl_mut(unwind: &mut UnwindBuilder) -> &mut UnwindBuilderImpl {
  // Safety: 见函数文档——`unwind` 由上下文构造点保证指向本平台的具体 unwind 实现对象,`UnwindBuilder`
  // 与实现共享 `repr(C)` 前缀布局,故基址重合、向下转型得到的 `&mut UnwindBuilderImpl` 类型正确且不与其他别名冲突。
  unsafe { &mut *(from_mut(unwind).cast::<UnwindBuilderImpl>()) }
}

/// 写 unwind header 的 start 信息（cpp startInfo）。
///
/// # Safety
/// 见 [`as_impl_mut`]。
pub(crate) unsafe fn start_info(unwind: &mut UnwindBuilder, arch: Arch) {
  // Safety: `as_impl_mut` 契约(见其文档)由本 `unsafe fn` 透传给调用方;`unwind` 指向具体实现对象。
  unsafe { as_impl_mut(unwind) }.start_info(arch);
}

/// 写 unwind header 的 finish 信息（cpp finishInfo）。
///
/// # Safety
/// 见 [`as_impl_mut`]。
pub(crate) unsafe fn finish_info(unwind: &mut UnwindBuilder) {
  // Safety: 同 `start_info`——透传 `as_impl_mut` 契约,`unwind` 指向具体实现对象。
  unsafe { as_impl_mut(unwind) }.finish_info();
}

/// 写 unwind header 的 begin offset（cpp setBeginOffset）。
///
/// # Safety
/// 见 [`as_impl_mut`]。
pub(crate) unsafe fn set_begin_offset(unwind: &mut UnwindBuilder, begin_offset: usize) {
  // Safety: 同上——透传 `as_impl_mut` 契约,`unwind` 指向具体实现对象。
  unsafe { as_impl_mut(unwind) }.set_begin_offset(begin_offset);
}

/// cpp 基类虚调用 `unwind.startFunction()`：A64/X64 入口函数构建两侧同款壳，
/// 原先两文件各抄一份，收口于此。
///
/// # Safety
/// 见 [`as_impl_mut`]。
pub(crate) fn unwind_start_function(unwind: &mut UnwindBuilder) {
  // Safety: `unwind` 由 code-gen 上下文构造点保证指向本平台具体 unwind 实现对象，
  // `UnwindBuilder` 与实现共享 `repr(C)` 前缀布局 → 基址重合，向下转型类型正确；
  // `&mut` 借自调用栈唯一入口，无别名。
  unsafe { as_impl_mut(unwind) }.start_function();
}

/// cpp 基类虚调用 `unwind.finishFunction(beginOffset, endOffset)`（A64/X64 同款壳）。
///
/// # Safety
/// 见 [`as_impl_mut`]。
pub(crate) fn unwind_finish_function(
  unwind: &mut UnwindBuilder,
  begin_offset: u32,
  end_offset: u32,
) {
  // Safety: 同 `unwind_start_function`——下转类型正确且该 `&mut` 借用为此刻唯一持有者。
  unsafe { as_impl_mut(unwind) }.finish_function(begin_offset, end_offset);
}
