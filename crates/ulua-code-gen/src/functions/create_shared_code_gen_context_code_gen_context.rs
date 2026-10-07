use alloc::alloc::{alloc, dealloc, handle_alloc_error};
use core::{
  alloc::Layout,
  ffi::c_void,
  ptr::{NonNull, drop_in_place, null_mut, write},
};

use ulua_common::fint::{LuauCodeGenBlockSize, LuauCodeGenMaxTotalSize};

use crate::{
  enums::options::CodeGenContextKind,
  records::shared_code_gen_context::SharedCodeGenContext,
  type_aliases::{
    api::AllocationCallback, unique_shared_code_gen_context::UniqueSharedCodeGenContext,
  },
};

pub fn create_shared_code_gen_context() -> UniqueSharedCodeGenContext {
  let block_size = LuauCodeGenBlockSize.get() as usize;
  let max_total_size = LuauCodeGenMaxTotalSize.get() as usize;

  // None 即 cpp `nullptr` 形参——「无自定义分配回调」，构造点回落默认分配器；
  // 宿主 user-data 指针同传 null（C 回调契约允许，见 `CodeAllocator::context` 字段注）。
  create_shared_code_gen_context_with_callback(block_size, max_total_size, None, null_mut())
}

/// 在专用堆块上落位一个 Shared 上下文。`unsafe` 全部为自足的内部裸内存操作
/// （新鲜 layout 分配/未初始化块写入/配对回收），无需调用方前提，故签名为 safe fn。
pub(crate) fn create_shared_code_gen_context_with_callback(
  block_size: usize,
  max_total_size: usize,
  allocation_callback: Option<AllocationCallback>,
  allocation_callback_context: *mut c_void,
) -> UniqueSharedCodeGenContext {
  // Safety: `Layout::new::<SharedCodeGenContext>()` 的 size/align 由类型自身导出，恒有效，故 `alloc`
  // 返回满足对齐的非空块（null 时 handle_alloc_error 中止，不再向下）；`ptr::write` 把构造器
  // 返回值整体落位到该存活未初始化块（无旧值需 drop），随后 `pin_shared_allocator` 在最终地址
  // 上完成共享分配器回指接线（此后对象不再搬移），`init_header` 为存活对象上的只读判旗。
  // 失败分支先 `drop_in_place(ptr)` 再 `dealloc(ptr,layout)`，二者用同一 layout 且 drop 先于
  // 释放、无 double-free；成功分支 ptr 已证非空。单线程串行、ptr 未被他人借用。
  // 以下各窄块统一援引本契约。
  let layout = Layout::new::<SharedCodeGenContext>();
  let ptr = unsafe { alloc(layout) as *mut SharedCodeGenContext };
  if ptr.is_null() {
    handle_alloc_error(layout);
  }

  // Safety: 依上契约——构造器为 safe fn（回调以 `Option` 表达缺席、宿主指针原样透传），
  // write/pin 均在存活未初始化块/已落位对象上进行。
  unsafe {
    write(
      ptr,
      SharedCodeGenContext::new(
        CodeGenContextKind::Shared,
        block_size,
        max_total_size,
        allocation_callback,
        allocation_callback_context,
      ),
    );
    (*ptr).pin_shared_allocator();
  }

  // Safety: 依上契约——init_header 为存活对象上的 safe 方法。
  if !unsafe { (*ptr).init_header_functions() } {
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
