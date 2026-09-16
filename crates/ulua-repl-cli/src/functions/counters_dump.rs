use alloc::string::String;
use core::{
  ffi::{CStr, c_char, c_int, c_void},
  fmt,
  mem::zeroed,
  ptr::addr_of_mut,
};

use ulua_vm::{
  functions::{lua_getcounters::lua_getcounters, lua_getinfo::lua_getinfo},
  macros::{lua_getref::lua_getref, lua_pop::lua_pop},
  records::lua_debug::LuaDebug,
};

use crate::{
  functions::{
    counters_function_callback::counters_function_callback, counters_init::G_COUNTERS,
    counters_value_callback::counters_value_callback,
  },
  records::module_counters::ModuleCounters,
};

unsafe extern "C" {
  fn fopen(path: *const c_char, mode: *const c_char) -> *mut c_void;
  fn fclose(stream: *mut c_void) -> c_int;
  fn fwrite(ptr: *const c_void, size: usize, n: usize, stream: *mut c_void) -> usize;
}

/// 向 C `FILE*` 写格式化输出（Rust fmt + 栈缓冲），取代变参 `fprintf`/`fputs`。
///
/// # Safety
/// `f` 必须是有效的 `FILE*`。
unsafe fn file_write(f: *mut c_void, args: fmt::Arguments<'_>) {
  struct FmtFile<'a> {
    buf: &'a mut [u8],
    len: usize,
  }

  impl fmt::Write for FmtFile<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
      let bytes = s.as_bytes();
      let rest = &mut self.buf[self.len..];
      let n = bytes.len().min(rest.len());
      rest[..n].copy_from_slice(&bytes[..n]);
      self.len += n;
      Ok(())
    }
  }

  const CAP: usize = 192;
  let mut buf = [0u8; CAP];
  let len = {
    let mut out = FmtFile {
      buf: &mut buf,
      len: 0,
    };
    let _ = fmt::write(&mut out, args);
    out.len
  };
  unsafe {
    fwrite(buf.as_ptr().cast(), 1, len, f);
  }
}

// `extern "C-unwind"` adapters bridging the counter callbacks to lua_getcounters'
// `LuaCounterFunction` / `LuaCounterValue` function-pointer types.
unsafe extern "C-unwind" fn function_callback_cb(
  context: *mut c_void,
  function: *const c_char,
  line_defined: c_int,
) {
  unsafe {
    counters_function_callback(context, function, line_defined);
  }
}

unsafe extern "C-unwind" fn value_callback_cb(
  context: *mut c_void,
  kind: c_int,
  line: c_int,
  hits: u64,
) {
  unsafe {
    counters_value_callback(context, kind, line, hits);
  }
}

// Faithful port of `void countersDump(const char* path)`.
pub fn counters_dump(path: &str) {
  unsafe {
    let counters_ptr = addr_of_mut!(G_COUNTERS);
    let l = (*counters_ptr).l;

    let refs = (*counters_ptr).module_refs.clone();
    for fref in refs {
      lua_getref(l, fref);

      // C++ `LuaDebug ar = {}`.
      let mut ar: LuaDebug = zeroed();
      lua_getinfo(l, -1, c"s".as_ptr(), &mut ar as *mut LuaDebug);

      (*counters_ptr).module_counters.push(ModuleCounters {
        name: if ar.short_src.is_null() {
          String::new()
        } else {
          CStr::from_ptr(ar.short_src).to_string_lossy().into_owned()
        },
        ..Default::default()
      });
      // Stable pointer to the just-pushed element; the callbacks only
      // mutate this element (never the outer vector), matching C++.
      let module_counters =
        (*counters_ptr).module_counters.last_mut().unwrap() as *mut ModuleCounters as *mut c_void;

      lua_getcounters(
        l,
        -1,
        module_counters,
        Some(function_callback_cb),
        Some(value_callback_cb),
      );

      lua_pop(l, 1);
    }

    let path_c = alloc::format!("{}\0", path);
    let f = fopen(path_c.as_ptr() as *const c_char, c"wb".as_ptr());
    if f.is_null() {
      eprintln!("Error opening counters file (callgrind) {}", path);
      return;
    }

    file_write(f, format_args!("version: 1\n"));
    file_write(f, format_args!("creator: Luau REPL\n"));
    file_write(f, format_args!("events: Regular Fallback VmExit\n"));

    for module_counter in (*counters_ptr).module_counters.iter() {
      file_write(f, format_args!("fl={}\n", module_counter.name));

      for function_counter in module_counter.functions.iter() {
        file_write(f, format_args!("fn={}\n", function_counter.name));

        // BTreeMap already iterates by ascending line, matching the C++
        // "sorted by line" presentation requirement.
        for (line, counters) in function_counter.counters.iter() {
          if counters.regular_executed != 0
            || counters.fallback_executed != 0
            || counters.vm_exit_taken != 0
          {
            file_write(
              f,
              format_args!(
                "{} {} {} {}\n",
                line, counters.regular_executed, counters.fallback_executed, counters.vm_exit_taken
              ),
            );
          }
        }
      }
    }

    fclose(f);

    println!(
      "Counters data written to {} ({} modules)",
      path,
      (*counters_ptr).module_counters.len()
    );
  }
}
