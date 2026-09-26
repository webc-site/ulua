use core::ffi::{c_char, c_int, c_void};
use std::{
  fs::File,
  io::{BufWriter, Write},
};

use ulua_ast::functions::optional_node::opt_node;
use ulua_common::functions::{c_slice::c_slice, c_str::cstr_cow};
use ulua_vm::{
  functions::lua_getcoverage::lua_getcoverage,
  macros::{lua_getref::lua_getref, lua_pop::lua_pop},
};

use crate::functions::{
  coverage_callback::coverage_callback, coverage_init::G_COVERAGE,
  create_dump_writer::create_dump_writer, stack_function_name::stack_function_name,
};

// lua_getcoverage 的 C 回调外壳：把泛型 `coverage_callback<W: Write>`（无法直接充当
// C ABI）适配为 `LuaCoverage` 期望的 `unsafe extern "C-unwind" fn`，context 在此
// 落回具体 `&mut BufWriter<File>`，VM 裸指针（function/hits）也在此单点转成 Rust
// 借用（null function 以 `Option::None` 保留，供 cpp 的 `<anonymous>` 分支区分）。
// FFI 外壳形式由 `functions/mod.rs` 的 `c_abi_cb!` 模板单源承包。
c_abi_cb! {
  /// # Safety
  ///
  /// 作为 lua_getcoverage 的 C 回调安装，context/function/hits 的有效性由
  /// coverage_dump 调用处的 Safety 契约保证：context 指向存活的 BufWriter，
  /// function 为空指针或 NUL 结尾 C 串，hits 为空指针或指向 size 个可读
  /// c_int，均在回调返回前保持有效。
  fn coverage_callback_cb(
    context: *mut c_void,
    function: *const c_char,
    linedefined: c_int,
    depth: c_int,
    hits: *const c_int,
    size: usize,
  ) {
    // Safety: 上面 # Safety 契约逐条满足各门面前置——context 非空且独占可借用
    // （回调窗口内无人别名该 BufWriter）；判空后的 function 指向 NUL 结尾串，
    // 借用仅在本调用窗口内使用；hits/size 由 c_slice 收敛（null/零长为空切片）。
    unsafe {
      let out = &mut *(context.cast::<BufWriter<File>>());
      let function = (!function.is_null()).then(|| cstr_cow(function));
      coverage_callback(
        out,
        function.as_deref(),
        linedefined,
        depth,
        c_slice(hits, size),
      );
    }
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
      // Safety: l 为 coverage_init 记录的 VM 主线程，fref 是已注册的表引用。
      unsafe { lua_getref(l, fref) };

      // cpp 的 short_src 是内嵌 char[256]，取不到即空串；本端口为裸指针，
      // lua_getinfo 失败时保持 null，判空取串收敛到 stack_function_name。
      // Safety: 栈顶是 lua_getref 压入的函数。
      let short_src = unsafe { stack_function_name(l) };
      let _ = writeln!(out, "SF:{short_src}");

      // Safety: out 在本作用域内存活，context 与回调签名匹配（只写该 BufWriter）。
      unsafe {
        lua_getcoverage(
          l,
          -1,
          &mut out as *mut BufWriter<File> as *mut c_void,
          Some(coverage_callback_cb),
        )
      };
      let _ = out.write_all(b"end_of_record\n");

      // Safety: 与 lua_getref 配平，弹出栈顶函数。
      unsafe { lua_pop(l, 1) };
    }

    // cpp `fclose` 隐式 flush; 失败同样静默
    let _ = out.flush();

    let dumped = coverage.functions.len();
    println!("Coverage dump written to {path} ({dumped} functions)");
  });
}
