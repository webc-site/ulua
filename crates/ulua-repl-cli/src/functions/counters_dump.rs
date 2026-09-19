use alloc::string::String;
use core::{
  ffi::{CStr, c_char, c_int, c_void},
  mem::zeroed,
  ptr::addr_of_mut,
};
use std::{
  fs::File,
  io::{BufWriter, Write},
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
  let counters_ptr = addr_of_mut!(G_COUNTERS);

  unsafe {
    let l = (*counters_ptr).l;

    // 循环体只读 module_refs、写 module_counters，字段不相交，直接借用免 clone
    for fref in (*counters_ptr).module_refs.iter().copied() {
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
  }

  let module_counters = unsafe { &(*counters_ptr).module_counters };

  // cpp `fopen(path, "wb")`：std::fs 写模式打开，失败报错返回
  let Ok(file) = File::create(path) else {
    eprintln!("Error opening counters file (callgrind) {}", path);
    return;
  };
  let mut out = BufWriter::new(file);

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
}
