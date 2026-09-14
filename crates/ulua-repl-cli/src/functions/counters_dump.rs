use alloc::string::String;
use core::{
  ffi::{CStr, c_char, c_int, c_longlong, c_void},
  mem::zeroed,
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
  fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
  fn fputs(s: *const c_char, stream: *mut c_void) -> c_int;
}

// `extern "C"` adapters bridging the counter callbacks to lua_getcounters'
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
    let counters_ptr = core::ptr::addr_of_mut!(G_COUNTERS);
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

    fputs(c"version: 1\n".as_ptr(), f);
    fputs(c"creator: Luau REPL\n".as_ptr(), f);
    fputs(c"events: Regular Fallback VmExit\n".as_ptr(), f);

    for module_counter in (*counters_ptr).module_counters.iter() {
      let name_c = alloc::format!("{}\0", module_counter.name);
      fprintf(f, c"fl=%s\n".as_ptr(), name_c.as_ptr());

      for function_counter in module_counter.functions.iter() {
        let fn_name_c = alloc::format!("{}\0", function_counter.name);
        fprintf(f, c"fn=%s\n".as_ptr(), fn_name_c.as_ptr());

        // BTreeMap already iterates by ascending line, matching the C++
        // "sorted by line" presentation requirement.
        for (line, counters) in function_counter.counters.iter() {
          if counters.regular_executed != 0
            || counters.fallback_executed != 0
            || counters.vm_exit_taken != 0
          {
            fprintf(
              f,
              c"%d %lld %lld %lld\n".as_ptr(),
              *line as c_int,
              counters.regular_executed as c_longlong,
              counters.fallback_executed as c_longlong,
              counters.vm_exit_taken as c_longlong,
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
