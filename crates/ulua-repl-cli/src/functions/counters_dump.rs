use core::{
  ffi::{c_char, c_int, c_void},
  ptr::from_mut,
};
use std::io::Write;

use ulua_ast::functions::optional_node::opt_node;
use ulua_common::functions::c_str::cstr_cow;
use ulua_vm::{
  functions::lua_getcounters::lua_getcounters,
  macros::{lua_getref::lua_getref, lua_pop::lua_pop},
};

use crate::{
  functions::{
    counters_function_callback::counters_function_callback, counters_init::G_COUNTERS,
    counters_value_callback::counters_value_callback, create_dump_writer::create_dump_writer,
    stack_function_name::stack_function_name,
  },
  records::{counters::Counters, module_counters::ModuleCounters},
};

// lua_getcounters 的 C 回调外壳：把同名 Rust fn 适配为 `LuaCounterFunction` /
// `LuaCounterValue` 期望的 `unsafe extern "C-unwind" fn`，形式由 `functions/mod.rs`
// 的 `c_abi_cb!` 模板单源承包；VM 裸指针（context/function）只在外壳内转成
// Rust 借用（null function 以 `Option::None` 保留给 cpp 的 `<main>`/`<anonymous>`
// 分支），核心函数为安全签名。
c_abi_cb! {
  /// # Safety
  ///
  /// 作为 lua_getcounters 的 `LuaCounterFunction` 安装，参数契约由 counters_dump 的
  /// lua_getcounters 调用处 Safety 说明保证：context 指向本次调用窗口内存活、可独占
  /// 借用的 ModuleCounters，function 为空指针或 NUL 结尾 C 串。
  fn function_callback_cb(
    context: *mut c_void,
    function: *const c_char,
    line_defined: c_int,
  ) {
    // Safety: 上面 # Safety 契约——context 非空且回调窗口内无别名；判空后的
    // function 指向 NUL 结尾串，借用仅在本调用窗口内使用（转调后立即失效）。
    unsafe {
      let counters = &mut *(context.cast::<ModuleCounters>());
      let function = (!function.is_null()).then(|| cstr_cow(function));
      counters_function_callback(counters, function.as_deref(), line_defined);
    }
  }
}

c_abi_cb! {
  /// # Safety
  ///
  /// 同 `function_callback_cb`：作为 lua_getcounters 的 `LuaCounterValue` 安装，
  /// context 指向 counters_dump 压入的存活、可独占借用的 ModuleCounters，
  /// 契约由调用处保证。
  fn value_callback_cb(
    context: *mut c_void,
    kind: c_int,
    line: c_int,
    hits: u64,
  ) {
    // Safety: context 指向本次调用窗口内存活、无别名的 ModuleCounters（上方
    // # Safety 契约，由 VM 经 counters_dump 的调用处保证）。
    unsafe { counters_value_callback(&mut *context.cast::<ModuleCounters>(), kind, line, hits) }
  }
}

// Faithful port of `void countersDump(const char* path)`.
pub(crate) fn counters_dump(path: &str) {
  G_COUNTERS.with(|cell| {
    // 与 cpp 直接访问文件静态量等价：循环体只读 module_refs（逐个复制 i32）、
    // 写 module_counters，字段不相交；回调只改本帧的局部元素（收集完成后才 push），
    // 不再访问 G_COUNTERS。
    let mut counters = cell.borrow_mut();

    // 未 init 时为 None，经 opt_node 落回 cpp 同款空指针后照常交给 VM 调用
    let l = opt_node(counters.l);

    // 解构借用：module_refs 只读遍历、module_counters 就地追加，字段不相交，
    // 借用检查器允许并行持有。
    let Counters {
      module_refs,
      module_counters,
      ..
    } = &mut *counters;

    for &fref in module_refs.iter() {
      // 一整个「取引用 → 读函数名 → 收集计数 → 弹栈」序列都在同一 VM 窗口内成立，
      // 契约合并为一条：cpp 是 push 进 Vec 后取 `&back()` 交给回调，Rust 侧改为先填
      // 局部元素、收集完成后再 push —— 回调只写该元素（见两个 cb 的 # Safety），
      // 于是既不需要 `last_mut()` 反查、也不依赖 Vec 元素地址在多次 push 间稳定。
      let module = unsafe {
        // Safety: l 为 counters_init 记录的 VM 主线程，fref 是 lua_ref 注册的引用，
        // getref 压入的栈顶即该函数值。
        lua_getref(l, fref);
        // Safety: 栈顶是上一行压入的函数（stack_function_name 的 /// # Safety）。
        let name = stack_function_name(l);

        let mut module = ModuleCounters {
          name,
          ..Default::default()
        };
        // Safety: `&mut module` 是本帧独占、在 lua_getcounters 调用窗口内存活的元素，
        // 以 `ptr::from_mut` 交出地址（无指针算术、无裸 `as` 转换）；两个回调按各自
        // # Safety 只写该元素，不触碰外层 Vec 与 G_COUNTERS。
        lua_getcounters(
          l,
          -1,
          from_mut(&mut module).cast::<c_void>(),
          Some(function_callback_cb),
          Some(value_callback_cb),
        );
        // Safety: 与 lua_getref 配平，弹出栈顶函数。
        lua_pop(l, 1);
        module
      };

      module_counters.push(module);
    }

    // cpp `fopen(path, "wb")`：std::fs 写模式打开，失败报错返回
    let Some(mut out) = create_dump_writer(path, "counters file (callgrind)") else {
      return;
    };

    // cpp 忽略 fprintf 返回值，此处一致
    let _ = write!(
      out,
      "version: 1\ncreator: Luau REPL\nevents: Regular Fallback VmExit\n"
    );

    for module_counter in module_counters.iter() {
      let _ = writeln!(out, "fl={}", module_counter.name);

      for function_counter in module_counter.functions.iter() {
        let _ = writeln!(out, "fn={}", function_counter.name);

        // BTreeMap already iterates by ascending line, matching the C++
        // "sorted by line" presentation requirement.
        for (line, counters) in function_counter.counters.iter() {
          if counters.regular_executed != 0
            || counters.fallback_executed != 0
            || counters.vm_exit_taken != 0
          {
            let _ = writeln!(
              out,
              "{line} {} {} {}",
              counters.regular_executed, counters.fallback_executed, counters.vm_exit_taken
            );
          }
        }
      }
    }

    // cpp `fclose` 隐式 flush；失败同样静默
    let _ = out.flush();

    println!(
      "Counters data written to {} ({} modules)",
      path,
      module_counters.len()
    );
  });
}
