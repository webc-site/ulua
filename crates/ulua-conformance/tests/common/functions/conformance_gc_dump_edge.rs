use core::ffi::{c_char, c_void};

use crate::common::{
  functions::cstr_text::cstr_text,
  records::{conformance_gc_dump_enum_context::ConformanceGcDumpEnumContext, heap_edge::HeapEdge},
};

/// cpp `Conformance.test.cpp:3607-3612` 里 `luaC_enumheap` 的 edge 回调：原样追加一条边。
///
/// 同一个源节点会有多条出边（表的多个值、线程的栈槽与 upvalue 等），因此必须 push 到
/// `Vec`，不能以源指针为键写进映射——那会让每个源只剩最后一条出边。
///
/// # Safety
///
/// `context` 必须指向存活的 [`ConformanceGcDumpEnumContext`]，`from`/`to` 必须是本次枚举
/// 期间存活的 GC 对象指针，`name` 必须是 `nullptr` 或以 NUL 结尾的 C 字符串。
pub unsafe extern "C-unwind" fn conformance_gc_dump_edge(
  context: *mut c_void,
  from: *mut c_void,
  to: *mut c_void,
  name: *const c_char,
) {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`to` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    let context = &mut *(context as *mut ConformanceGcDumpEnumContext);

    context.heap.edges.push(HeapEdge {
      source: from as usize,
      target: to as usize,
      // Safety: 空指针按 cpp 的 `nullptr` 区分保留为 `None`；非空时 `name` NUL 结尾。
      name: (!name.is_null()).then(|| cstr_text(name).into_owned()),
    });
  }
}
