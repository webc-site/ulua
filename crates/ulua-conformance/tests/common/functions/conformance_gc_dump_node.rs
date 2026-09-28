use core::ffi::{c_char, c_void};

use ulua_vm::enums::lua_type::LuaType;

use crate::common::{
  functions::cstr_text::cstr_text,
  records::{conformance_gc_dump_enum_context::ConformanceGcDumpEnumContext, heap_node::HeapNode},
};

/// cpp `Conformance.test.cpp:3570-3605` 里 `luaC_enumheap` 的 node 回调：校验上游用
/// `CHECK` 断言的节点名字，并把节点登记进堆快照。
///
/// # Safety
///
/// `context` 必须指向存活的 [`ConformanceGcDumpEnumContext`]，`name` 必须是 `nullptr`
/// 或以 NUL 结尾的 C 字符串，`ptr` 必须是本次枚举期间存活的 GC 对象指针。
pub unsafe extern "C-unwind" fn conformance_gc_dump_node(
  context: *mut c_void,
  ptr: *mut c_void,
  tt: u8,
  // C ABI 回调签名固定要带内存分类，上游 HeapNode 也存了它，但 GCDump 用例从不读取，
  // 因此本端口的 HeapNode 不保留该字段（见 records/heap_node.rs）。
  _memcat: u8,
  size: usize,
  name: *const c_char,
) {
  // Safety: 函数级 `# Safety` 契约保证 `context` 指向存活的枚举上下文（并行测试
  // 下由本用例独占、无并发访问；指针由本用例 Box/栈上构造且本帧存活）。
  let context = unsafe { &mut *(context as *mut ConformanceGcDumpEnumContext) };

  // Safety: 函数级 `# Safety` 契约保证 `name` 为 null 或以 NUL 结尾的 C 字符串，
  // 且在本回调帧内可读；上游 `if (name)` 判的是 `nullptr`，不是空串——这里用
  // Option 表达同一个区分。
  let name = unsafe { (!name.is_null()).then(|| cstr_text(name).into_owned()) };

  validate_and_record(context, ptr, tt, size, name);
}

/// 校验节点名/大小并登记堆快照——cpp `CHECK` 序列的纯安全转写，无指针操作。
fn validate_and_record(
  context: &mut ConformanceGcDumpEnumContext,
  ptr: *mut c_void,
  tt: u8,
  size: usize,
  name: Option<String>,
) {
  if let Some(name) = &name {
    match tt as i32 {
      x if x == LuaType::UserData as i32 => {
        if name != "u42" {
          context
            .errors
            .push(format!("unexpected userdata name: {name}"));
        }
      }
      x if x == LuaType::Proto as i32 => {
        if !matches!(
          name.as_str(),
          "proto unnamed:1 =GCDump" | "proto foo:7 =GCDump" | "proto f:4 =GCDump"
        ) {
          context
            .errors
            .push(format!("unexpected Proto name: {name}"));
        }
      }
      x if x == LuaType::Function as i32 => {
        // 后两个名字来自 `class HeapClass` 实例化时 luaR_newclass 建出的两个 C 闭包
        // （cpp `VM/src/lclass.cpp:47-50`、`cpp/VM/src/lclass.cpp:67-72`）。
        if !matches!(
          name.as_str(),
          "test"
            | "unnamed:1 =GCDump"
            | "foo:7 =GCDump"
            | "f:4 =GCDump"
            | "luaR_defaultcreateobject"
            | "luaR_constructobject"
        ) {
          context
            .errors
            .push(format!("unexpected function name: {name}"));
        }
      }
      x if x == LuaType::Thread as i32 && name != "thread at unnamed:1 =GCDump" => {
        context
          .errors
          .push(format!("unexpected thread name: {name}"));
      }
      _ => {}
    }
  } else if tt as i32 == LuaType::String as i32 && size >= 100_000 {
    if context.seen_target_string {
      context
        .errors
        .push("saw more than one target-sized string".to_owned());
    }

    context.seen_target_string = true;

    if size <= 100_000 {
      context.errors.push(format!(
        "target string size did not include overhead: {size}"
      ));
    }
  }

  context.heap.nodes.insert(
    ptr as usize,
    HeapNode {
      ptr: ptr as usize,
      tag: tt,
      size,
      name: name.unwrap_or_default(),
      marked: false,
    },
  );
}
