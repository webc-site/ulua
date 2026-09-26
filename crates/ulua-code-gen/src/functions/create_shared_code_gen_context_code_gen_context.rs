use alloc::alloc::{alloc, dealloc, handle_alloc_error};
use core::{
  alloc::Layout,
  ffi::c_void,
  ptr::{NonNull, drop_in_place, null_mut},
};

use ulua_common::fint::{LuauCodeGenBlockSize, LuauCodeGenMaxTotalSize};

use crate::{
  records::shared_code_gen_context::SharedCodeGenContext,
  type_aliases::{
    allocation_callback::AllocationCallback,
    unique_shared_code_gen_context::UniqueSharedCodeGenContext,
  },
};

pub fn create_shared_code_gen_context() -> UniqueSharedCodeGenContext {
  let block_size = LuauCodeGenBlockSize.get() as usize;
  let max_total_size = LuauCodeGenMaxTotalSize.get() as usize;

  // Safety: 被调 `unsafe fn` 的契约是「回调指针为 null 或指向存活 AllocationCallback」。这里传
  // null_mut() 正是「无自定义分配回调」的哨兵：BaseCodeGenContext 构造点以 `is_null()` 判定后回落到
  // 默认分配器（见 base_code_gen_context_..._code_gen_context.rs），context 为 null 亦被 C 回调
  // 契约允许；block_size/max_total_size 取自 fint 全局的合法正值，故满足被调契约。
  unsafe {
    create_shared_code_gen_context_usize_usize_allocation_callback_void(
      block_size,
      max_total_size,
      null_mut(),
      null_mut(),
    )
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn create_shared_code_gen_context_usize_usize_allocation_callback_void(
  block_size: usize,
  max_total_size: usize,
  allocation_callback: *mut AllocationCallback,
  allocation_callback_context: *mut c_void,
) -> UniqueSharedCodeGenContext {
  // Safety: `Layout::new::<SharedCodeGenContext>()` 的 size/align 由类型自身导出，恒有效，故 `alloc`
  // 返回满足对齐的非空块（null 时 handle_alloc_error 中止，不再向下）；随后 `(*ptr).构造/init_header`
  // 是在该存活对象上初始化与只读判旗。失败分支先 `drop_in_place(ptr)` 再 `dealloc(ptr,layout)`，二者
  // 用同一 layout 且 drop 先于释放、无 double-free；成功分支 ptr 已证非空。单线程串行、ptr 未被他人借用。
  // 以下各窄块统一援引本契约。
  let layout = Layout::new::<SharedCodeGenContext>();
  let ptr = unsafe { alloc(layout) as *mut SharedCodeGenContext };
  if ptr.is_null() {
    handle_alloc_error(layout);
  }

  // Safety: 依上契约——ptr 指向存活未初始化对象，ctor 在其上整体初始化。
  unsafe {
    (*ptr).shared_code_gen_context_shared_code_gen_context(
      block_size,
      max_total_size,
      allocation_callback,
      allocation_callback_context,
    );
  }

  // Safety: 依上契约——init_header 为存活对象上的只读判旗（方法本身 safe）。
  if !unsafe { (*ptr).base.init_header_functions() } {
    // Safety: 依上契约——drop 先于释放、同一 layout，无 double-free。
    unsafe {
      drop_in_place(ptr);
      dealloc(ptr as *mut u8, layout);
    }
    return NonNull::dangling();
  }

  // Safety: 依上契约——成功分支 ptr 已证非空。
  unsafe { NonNull::new_unchecked(ptr) }
}
