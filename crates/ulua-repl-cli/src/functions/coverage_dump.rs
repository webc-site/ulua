use core::{ffi::c_void, ptr::from_mut};
use std::{
  fs::File,
  io::{BufWriter, Write},
};

use ulua_ast::functions::optional_node::opt_node;
use ulua_vm::functions::lua_getcoverage::lua_getcoverage;

use crate::functions::{
  coverage_callback::coverage_callback, coverage_init::G_COVERAGE,
  create_dump_writer::create_dump_writer, stack_function_name::stack_function_name,
  state_ref::state,
};

// lua_getcoverage 的回调外壳：把泛型 `coverage_callback<W: Write>`（无法直接充当
// 函数指针型）适配为 `LuaCoverage`，`context` 在此落回具体 `&mut BufWriter<File>`，
// VM 侧的串体窗与行计数窗也在此单点转成 Rust 借用（`None` function 保留，供 cpp 的
// `<anonymous>` 分支与空串区分）。
// review.md §10 收形：`LuaCoverage` 已是 Rust ABI 的 `unsafe fn`——全仓无真实 C 消费者
// （ulua-capi 不导出 getcoverage 面），`function: Option<&[u8]>`、`hits: &[i32]`
// 均为原生窗，故本外壳不再有 `extern "C-unwind"`，也不再有 `cstr_cow`/`c_slice` 折转。
// DELIBERATE DEVIATION（review.md §9.3）：`context` 仍是调用方裸指针（VM 透传 POD 柄），
// 其解引用收口在本外壳单点，业务核心 `coverage_callback` 为安全泛型签名。

/// # Safety
///
/// 作为 lua_getcoverage 的 `LuaCoverage` 回调安装，context/function/hits 的有效性由
/// coverage_dump 调用处的 Safety 契约保证：context 指向存活的 BufWriter，function
/// 为空或指向本次调用窗口内存活的 VM 串体字节窗，hits 为本次调用窗口内存活的
/// 行计数窗（返回前有效，不得留存）。
unsafe fn coverage_callback_cb(
  context: *mut c_void,
  function: Option<&[u8]>,
  linedefined: i32,
  depth: i32,
  hits: &[i32],
) {
  // Safety: 上面 # Safety 契约逐条满足各门面前置——context 非空且独占可借用
  // （回调窗口内无人别名该 BufWriter）；function 借窗仅在本调用窗口内使用（lossy
  // 解码为就地读）；hits 已是 VM 交出的行计数窗，只读遍历。
  unsafe {
    let out = &mut *(context.cast::<BufWriter<File>>());
    let function = function.map(String::from_utf8_lossy);
    coverage_callback(out, function.as_deref(), linedefined, depth, hits);
  }
}

/// Faithful port of `void coverageDump(const char* path)` (`CLI/src/Coverage.cpp`)
pub(crate) fn coverage_dump(path: &str) {
  // cpp 的文件静态量 gCoverage 只在主线程（coverageDump）访问，无需同步。
  // 与 counters_dump 一致：循环体只读 functions、回调只写 out（BufWriter），
  // 字段不相交，持共享借用原地遍历，免掉整表 clone。
  G_COVERAGE.with(|cell| {
    let coverage = cell.borrow();

    // 未 init 时为 None，经 opt_node 落回 cpp 同款空指针后照常交给 VM 调用
    let l = opt_node(coverage.l);

    // cpp `fopen(path, "wb")`: 写模式打开, 失败报错返回
    let Some(mut out) = create_dump_writer(path, "coverage") else {
      return;
    };

    // cpp 忽略 fprintf 返回值, 此处一致
    let _ = out.write_all(b"TN:\n");

    for &fref in coverage.functions.iter() {
      // Safety: l 为 coverage_init 记录的 VM 主线程（有函数被 track 即已 init），
      // 经 `state` 门面物化后全走安全方法；fref 是已注册的表引用。
      let l = state(l);
      l.get_ref(fref);

      // cpp 的 short_src 是内嵌 char[256]，取不到即空串；本端口为裸指针，
      // lua_getinfo 失败时保持 null，判空取串收敛到 stack_function_name。
      let short_src = stack_function_name(l);
      let _ = writeln!(out, "SF:{short_src}");

      // Safety: `lua_getcoverage` 为 unsafe 导出；out 在本作用域内存活，context 与
      // 回调签名匹配（只写该 BufWriter）。
      unsafe {
        lua_getcoverage(
          l,
          -1,
          from_mut(&mut out).cast::<c_void>(),
          Some(coverage_callback_cb),
        )
      };
      let _ = out.write_all(b"end_of_record\n");

      // 与 lua_getref 配平，弹出栈顶函数。
      l.pop(1);
    }

    // cpp `fclose` 隐式 flush; 失败同样静默
    let _ = out.flush();

    let dumped = coverage.functions.len();
    println!("Coverage dump written to {path} ({dumped} functions)");
  });
}
